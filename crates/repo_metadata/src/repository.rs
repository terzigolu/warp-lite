use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(feature = "local_fs")]
use std::path::Path;

use futures::future::ready;
#[cfg(feature = "local_fs")]
use ignore::gitignore::Gitignore;
use warp_util::standardized_path::StandardizedPath;
use warpui::r#async::{BoxFuture, SpawnedFutureHandle};
#[cfg(feature = "local_fs")]
use warpui::SingletonEntity;
use warpui::{Entity, ModelContext, ModelHandle};

#[cfg(feature = "local_fs")]
use crate::watcher::DirectoryWatcher;
#[cfg(feature = "local_fs")]
use crate::{
    entry::{matches_gitignores, should_ignore_git_path},
    gitignores_for_directory,
};
use crate::{watcher::TaskQueue, RepoMetadataError, RepositoryUpdate};

/// Trait for entities that want to subscribe to repository file changes.
pub trait RepositorySubscriber: Send + Sync {
    /// Called when the subscriber is first added to build initial state.
    /// Returns a Future that completes when the scan is finished.
    fn on_scan(
        &mut self,
        repository: &Repository,
        ctx: &mut ModelContext<Repository>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    /// Called when file changes are detected in the repository.
    /// Returns a Future that completes once updates are processed.
    fn on_files_updated(
        &mut self,
        repository: &Repository,
        update: &RepositoryUpdate,
        ctx: &mut ModelContext<Repository>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    fn on_unsubscribe(&mut self, _ctx: &mut ModelContext<Repository>) {}
}

/// A unique identifier for repository subscribers.
pub type SubscriberId = usize;

/// Controls whether a subscriber needs Git metadata in addition to ordinary files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryWatchMode {
    FilesystemOnly,
    GitRepository,
}

struct RepositorySubscription {
    #[cfg_attr(not(feature = "local_fs"), allow(dead_code))]
    mode: RepositoryWatchMode,
    subscriber: Box<dyn RepositorySubscriber>,
}

pub struct StartWatching {
    pub subscriber_id: SubscriberId,
    pub registration_future: BoxFuture<'static, Result<(), RepoMetadataError>>,
}

/// Model for tracking a code repository that Warp is aware of.
pub struct Repository {
    /// The root directory of the repository.
    root_dir: StandardizedPath,
    /// External git directory path (e.g., for worktrees). This is the
    /// path to the **exact** per-worktree gitdir (e.g. `.git/worktrees/foo`).
    /// For the main worktree this is `None` (the gitdir is `root_dir/.git`).
    external_git_directory: Option<StandardizedPath>,
    /// The shared `.git` root directory that all worktrees of the same repo
    /// have in common. Derived from `external_git_directory` by walking up to
    /// the `.git` component. `None` when the repo is not a linked worktree.
    common_git_directory: Option<StandardizedPath>,
    /// Collection of subscribers interested in file changes.
    subscribers: HashMap<SubscriberId, RepositorySubscription>,
    /// Counter for generating unique subscriber IDs.
    next_subscriber_id: SubscriberId,
    /// Cached gitignore patterns for this repository.
    #[cfg(feature = "local_fs")]
    gitignores: Vec<Gitignore>,

    task_queue: ModelHandle<TaskQueue>,
}

impl Repository {
    /// Creates a new Repository instance.
    pub(super) fn new(
        root_dir: StandardizedPath,
        external_git_directory: Option<StandardizedPath>,
        task_queue: ModelHandle<TaskQueue>,
    ) -> Self {
        #[cfg(feature = "local_fs")]
        let gitignores = {
            let local_path = root_dir.to_local_path_lossy();
            gitignores_for_directory(&local_path)
        };

        let common_git_directory = external_git_directory.as_ref().and_then(|ext| {
            ext.to_local_path()
                .and_then(|local| Self::derive_common_git_dir(&local))
                .and_then(|p| StandardizedPath::try_from_local(&p).ok())
                // Only store when it differs from external_git_directory.
                .filter(|common| common != ext)
        });

        Self {
            root_dir,
            external_git_directory,
            common_git_directory,
            subscribers: HashMap::new(),
            next_subscriber_id: 0,
            #[cfg(feature = "local_fs")]
            gitignores,
            task_queue,
        }
    }

    /// Walk ancestors of the given path to find the `.git` component and return
    /// it as the shared git root. For example,
    /// `/repo/.git/worktrees/foo` → `/repo/.git`.
    fn derive_common_git_dir(external_git_dir: &std::path::Path) -> Option<std::path::PathBuf> {
        for ancestor in external_git_dir.ancestors() {
            if ancestor.file_name().and_then(|n| n.to_str()) == Some(".git") {
                return Some(ancestor.to_path_buf());
            }
        }
        None
    }

    /// The root directory of this repository.
    pub fn root_dir(&self) -> &StandardizedPath {
        &self.root_dir
    }

    /// The external git directory of this repository, if any.
    /// This is used for worktrees where the .git directory is external to the working tree.
    pub fn external_git_directory(&self) -> Option<&StandardizedPath> {
        self.external_git_directory.as_ref()
    }

    /// Returns the path to the actual `.git` directory for this repository.
    ///
    /// For normal repositories this is `root_dir/.git`. For worktrees, the
    /// `.git` entry in the working tree is a file (not a directory), so this
    /// returns the resolved `external_git_directory` instead.
    /// Subscribers should use this for per-worktree files like `index.lock`.
    pub fn git_dir(&self) -> std::path::PathBuf {
        self.external_git_directory
            .as_ref()
            .and_then(|d| d.to_local_path())
            .unwrap_or_else(|| self.root_dir.to_local_path_lossy().join(".git"))
    }

    /// Returns the shared `.git` root directory.
    ///
    /// For normal repos this is the same as `git_dir()`. For linked worktrees
    /// this is the common `.git` directory that all worktrees share (e.g.
    /// `/repo/.git`), distinct from the per-worktree gitdir.
    pub fn common_git_dir(&self) -> std::path::PathBuf {
        self.common_git_directory
            .as_ref()
            .and_then(|d| d.to_local_path())
            .unwrap_or_else(|| self.git_dir())
    }

    /// Returns the current watcher count.
    pub fn watcher_count(&self) -> usize {
        self.subscribers.len()
    }

    #[cfg(feature = "local_fs")]
    pub(crate) fn git_watch_paths(&self) -> Vec<StandardizedPath> {
        let mut paths = Vec::new();
        if let Some(external_git_dir) = &self.external_git_directory {
            paths.push(external_git_dir.clone());
        }
        if let Some(common_git_dir) = &self.common_git_directory {
            if let Some(common_local) = common_git_dir.to_local_path() {
                let refs_dir = common_local.join("refs").join("heads");
                if let Ok(refs_std) = StandardizedPath::from_local_canonicalized(&refs_dir) {
                    paths.push(refs_std);
                }
            }
        }
        paths
    }

    #[cfg(feature = "local_fs")]
    pub(crate) fn has_git_repository_subscribers(&self) -> bool {
        self.subscribers
            .values()
            .any(|subscription| subscription.mode == RepositoryWatchMode::GitRepository)
    }

    /// Starts watching this repository with the given subscriber.
    ///
    /// If this is the first subscriber, the repository root will be added to the
    /// RepositoryWatcher's set of watched paths.
    #[cfg_attr(not(feature = "local_fs"), allow(unused_variables))]
    pub fn start_watching(
        &mut self,
        subscriber: Box<dyn RepositorySubscriber>,
        ctx: &mut ModelContext<Self>,
    ) -> StartWatching {
        self.start_watching_with_mode(RepositoryWatchMode::GitRepository, subscriber, ctx)
    }

    /// Starts watching with an explicit mode. Filesystem-only consumers avoid registering and
    /// processing Git metadata paths, while legacy callers retain Git-aware behavior.
    #[cfg_attr(not(feature = "local_fs"), allow(unused_variables))]
    pub fn start_watching_with_mode(
        &mut self,
        mode: RepositoryWatchMode,
        subscriber: Box<dyn RepositorySubscriber>,
        ctx: &mut ModelContext<Self>,
    ) -> StartWatching {
        let subscriber_id = self.next_subscriber_id;
        self.next_subscriber_id += 1;

        #[cfg(feature = "local_fs")]
        let should_start_filesystem_watching = self.subscribers.is_empty();
        #[cfg(feature = "local_fs")]
        let should_start_git_watching =
            mode == RepositoryWatchMode::GitRepository && !self.has_git_repository_subscribers();

        self.subscribers
            .insert(subscriber_id, RepositorySubscription { mode, subscriber });

        #[cfg(feature = "local_fs")]
        let registration_future: BoxFuture<'static, Result<(), RepoMetadataError>> = {
            let mut directories_to_watch = Vec::new();
            if should_start_filesystem_watching {
                directories_to_watch.push(self.root_dir.clone());
            }
            if should_start_git_watching {
                directories_to_watch.extend(self.git_watch_paths());
            }
            if directories_to_watch.is_empty() {
                Box::pin(ready(Ok(())))
            } else {
                Box::pin(DirectoryWatcher::handle(ctx).update(ctx, |watcher, ctx| {
                    watcher.start_watching_directories(directories_to_watch, ctx)
                }))
            }
        };

        #[cfg(not(feature = "local_fs"))]
        let registration_future: BoxFuture<'static, Result<(), RepoMetadataError>> =
            Box::pin(async move { Ok(()) });

        let self_handle = ctx.handle();
        self.task_queue.update(ctx, |queue, ctx| {
            queue.enqueue_scan(self_handle, subscriber_id, ctx);
        });

        StartWatching {
            subscriber_id,
            registration_future,
        }
    }

    /// Stops watching this repository for the given subscriber.
    ///
    /// If this was the last subscriber, the repository root will be removed from the
    /// RepositoryWatcher's set of watched paths.
    #[cfg_attr(not(feature = "local_fs"), allow(unused_variables))]
    pub fn stop_watching(&mut self, subscriber_id: SubscriberId, ctx: &mut ModelContext<Self>) {
        let Some(mut subscription) = self.subscribers.remove(&subscriber_id) else {
            return;
        };

        subscription.subscriber.on_unsubscribe(ctx);

        #[cfg(feature = "local_fs")]
        let should_stop_git_watching = subscription.mode == RepositoryWatchMode::GitRepository
            && !self.has_git_repository_subscribers();

        let should_stop_filesystem_watching = self.subscribers.is_empty();
        if should_stop_filesystem_watching {
            // If this was the last subscriber, notify the RepWatcher to stop watching.
            log::debug!(
                "All subscribers removed for {}, stopping watcher",
                self.root_dir
            );
        }

        #[cfg(feature = "local_fs")]
        if should_stop_filesystem_watching || should_stop_git_watching {
            let root_dir = self.root_dir.clone();
            let git_paths = if should_stop_git_watching {
                self.git_watch_paths()
            } else {
                Vec::new()
            };
            DirectoryWatcher::handle(ctx).update(ctx, |watcher, ctx| {
                if should_stop_filesystem_watching {
                    std::mem::drop(watcher.stop_watching_directory(&root_dir, ctx));
                }
                watcher.stop_watching_unused_git_directories(&root_dir, git_paths, ctx);
            });
        }
    }

    /// Calls scan on a specific subscriber if it exists. Returns Some(Future) if the subscriber exists, None otherwise.
    pub(crate) fn scan_subscriber(
        &mut self,
        subscriber_id: SubscriberId,
        ctx: &mut ModelContext<Self>,
    ) -> Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>> {
        if let Some(mut subscription) = self.subscribers.remove(&subscriber_id) {
            let future = subscription.subscriber.on_scan(self, ctx);
            self.subscribers.insert(subscriber_id, subscription);
            Some(future)
        } else {
            None
        }
    }

    /// Notifies a specific subscriber about file changes.
    #[cfg(feature = "local_fs")]
    pub(crate) fn notify_subscriber(
        &mut self,
        subscriber_id: SubscriberId,
        update: &RepositoryUpdate,
        ctx: &mut ModelContext<Self>,
    ) -> Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>> {
        if let Some(mut subscription) = self.subscribers.remove(&subscriber_id) {
            let future = subscription.subscriber.on_files_updated(self, update, ctx);
            self.subscribers.insert(subscriber_id, subscription);
            Some(future)
        } else {
            None
        }
    }

    /// Returns updates filtered for each subscriber's watch mode.
    #[cfg(feature = "local_fs")]
    pub(crate) fn subscriber_updates(
        &self,
        update: &RepositoryUpdate,
    ) -> Vec<(SubscriberId, RepositoryUpdate)> {
        self.subscribers
            .iter()
            .filter_map(|(&subscriber_id, subscription)| {
                let mut update = update.clone();
                if subscription.mode == RepositoryWatchMode::FilesystemOnly {
                    update.commit_updated = false;
                    update.index_lock_detected = false;
                }
                (!update.is_empty()).then_some((subscriber_id, update))
            })
            .collect()
    }

    /// Checks if a path is gitignored within this repository.
    #[cfg(feature = "local_fs")]
    pub fn check_gitignore_status(&self, path: &Path) -> bool {
        // Check if path is a .git internal file
        if should_ignore_git_path(path) {
            return true;
        }

        // Check if path matches gitignore patterns
        let is_dir = path.is_dir();
        matches_gitignores(path, is_dir, &self.gitignores, true)
    }
}

impl Entity for Repository {
    type Event = ();
}

/// Coalescing merge for RepositoryUpdate with normalization rules.
fn merge_repository_updates(acc: &mut RepositoryUpdate, incoming: &RepositoryUpdate) {
    // 1) Moves first
    for (to, from) in &incoming.moved {
        if acc.added.remove(from) {
            acc.added.insert(to.clone());
            return;
        }
        if acc.modified.remove(from) {
            acc.modified.insert(to.clone());
            return;
        }

        // Collapse chain: if `from` was a prior destination, pull its original source
        let original_from = if let Some(prev_from) = acc.moved.remove(from) {
            prev_from
        } else {
            from.clone()
        };
        acc.moved.insert(to.clone(), original_from);
    }

    // 2) Adds next
    for p in &incoming.added {
        acc.deleted.remove(p);
        acc.moved.remove(p);
        acc.modified.remove(p);
        acc.added.insert(p.clone());
    }

    // 3) Modifies next
    for p in &incoming.modified {
        if acc.added.contains(p) {
            continue;
        }
        acc.deleted.remove(p);
        acc.moved.remove(p);
        acc.modified.insert(p.clone());
    }

    // 4) Deletes last
    for p in &incoming.deleted {
        // Added then removed within window => cancel
        if acc.added.remove(p) {
            continue;
        }

        acc.modified.remove(p);

        // Removing a move target => delete original source instead
        if let Some(from) = acc.moved.remove(p) {
            acc.deleted.insert(from);
            continue;
        }
        // Deleting the source of a recorded move is redundant; move already implies source removal
        let is_from_of_some_move = acc.moved.values().any(|f| f == p);
        if is_from_of_some_move {
            continue;
        }
        acc.deleted.insert(p.clone());
    }

    acc.commit_updated |= incoming.commit_updated;
    acc.index_lock_detected |= incoming.index_lock_detected;
}

/// A generic debouncing layer for any RepositorySubscriber.
pub struct BufferingRepositorySubscriber<S> {
    inner: Arc<Mutex<S>>,
    state: Arc<Mutex<BufferState>>,
    debounce: Duration,
}

#[derive(Default)]
struct BufferState {
    pending: RepositoryUpdate,
    /// Monotonic counter incremented for each incoming update; used to implement true debounce.
    version: u64,
    /// Whether the background flusher loop is currently running.
    flush_handle: Option<SpawnedFutureHandle>,
}

impl<S> BufferingRepositorySubscriber<S> {
    pub fn new(inner: S, debounce: Duration) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
            state: Arc::new(Mutex::new(BufferState::default())),
            debounce,
        }
    }
}

impl<S> RepositorySubscriber for BufferingRepositorySubscriber<S>
where
    S: RepositorySubscriber + Send + Sync + 'static,
{
    fn on_scan(
        &mut self,
        repository: &Repository,
        ctx: &mut ModelContext<Repository>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        self.inner.lock().unwrap().on_scan(repository, ctx)
    }

    fn on_files_updated(
        &mut self,
        _repository: &Repository,
        update: &RepositoryUpdate,
        ctx: &mut ModelContext<Repository>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        {
            let mut st = self.state.lock().unwrap();
            merge_repository_updates(&mut st.pending, update);
            st.version = st.version.wrapping_add(1);

            // Start a single background flusher if it's not already running.
            if st.flush_handle.is_none() {
                let inner = Arc::clone(&self.inner);
                let state = Arc::clone(&self.state);
                let wait = self.debounce;

                st.flush_handle = Some(ctx.spawn(
                    async move {
                        // Loop until we observe a quiet period (version stable for `wait`).
                        loop {
                            // Capture current version, then wait.
                            let start_version = {
                                let st = state.lock().unwrap();
                                st.version
                            };
                            warpui::r#async::Timer::after(wait).await;

                            // If version unchanged, we're quiet; flush pending and exit loop.
                            let maybe_merged = {
                                // Yield before flushing to check if the current flush is cancelled.
                                futures_lite::future::yield_now().await;

                                let mut st = state.lock().unwrap();
                                if st.version == start_version {
                                    st.flush_handle = None;
                                    Some(std::mem::take(&mut st.pending))
                                } else {
                                    // Newer update arrived during the wait; try waiting again.
                                    None
                                }
                            };

                            if let Some(merged) = maybe_merged {
                                break (inner, merged);
                            }
                        }
                    },
                    |repo_model, (inner, merged), repo_ctx| {
                        if merged.is_empty() {
                            return;
                        }
                        if let Ok(mut inner) = inner.lock() {
                            let fut = inner.on_files_updated(repo_model, &merged, repo_ctx);
                            // Drive the subscriber's async update to completion.
                            repo_ctx.spawn(fut, |_, _, _| {});
                        }
                    },
                ));
            }
        }

        Box::pin(ready(()))
    }

    fn on_unsubscribe(&mut self, _ctx: &mut ModelContext<Repository>) {
        let Ok(mut st) = self.state.lock() else {
            return;
        };
        if let Some(handle) = st.flush_handle.take() {
            handle.abort();
        }
    }
}
