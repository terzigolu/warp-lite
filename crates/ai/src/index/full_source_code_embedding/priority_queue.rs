// warp-lite: ai inert stub (v0.5 phase 2)
//
// Original: priority queue used by `CodebaseIndexManager` to schedule which
// codebase to (re)index next. The lightweight fork has no manager work, so
// this just keeps `BuildQueue` as an opaque empty placeholder.

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Priority {
    ActiveSession = 0,
    OpenSession = 1,
    #[default]
    PersistedSnapshot = 2,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub(super) struct BuildQueue;
