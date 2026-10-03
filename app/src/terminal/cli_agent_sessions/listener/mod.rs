use warpui::{EntityId, ModelContext, ModelHandle, SingletonEntity};

use super::{CLIAgentEvent, CLIAgentSessionsModel};
use crate::terminal::cli_agent_sessions::event::parse_event;
use crate::terminal::cli_agent_sessions::event::{CLIAgentEventPayload, CLIAgentEventType};
use crate::terminal::model_events::{ModelEvent, ModelEventDispatcher};
use crate::terminal::CLIAgent;

/// Per-agent handler that filters and transforms parsed CLI agent events.
/// Each CLI agent can have a different implementation depending on which events
/// it cares about.
trait CLIAgentSessionHandler {
    /// Attempt to parse a raw `PluggableNotification` into a typed event.
    /// The default implementation delegates to the structured JSON parser
    /// (`parse_event`); agents with non-JSON notification formats (e.g. Codex
    /// OSC 9 plain text) should override this.
    fn try_parse(&self, title: Option<&str>, body: &str) -> Option<CLIAgentEvent> {
        parse_event(title, body)
    }

    /// Decide whether a parsed event should be forwarded to the sessions model.
    /// Returns the event (possibly transformed) if it should be processed.
    fn handle_event(&mut self, event: CLIAgentEvent) -> Option<CLIAgentEvent>;

    /// Whether this handler provides meaningful, fine-grained status
    /// (e.g. in-progress / blocked / success) that should be shown in the UI.
    /// Handlers backed by the structured plugin protocol report rich status;
    /// handlers that only forward opaque OS notifications (e.g. Codex) do not.
    fn supports_rich_status(&self) -> bool {
        true
    }
}

/// Whether the listener for the given agent provides rich status.
/// Returns `false` for agents without a handler or whose handler opts out.
pub fn agent_supports_rich_status(agent: &CLIAgent) -> bool {
    create_handler(agent).is_some_and(|h| h.supports_rich_status())
}

/// Returns `true` if the given CLI agent has a supported session handler.
pub fn is_agent_supported(agent: &CLIAgent) -> bool {
    matches!(
        agent,
        CLIAgent::Claude
            | CLIAgent::OpenCode
            | CLIAgent::Codex
            | CLIAgent::Grok
            | CLIAgent::Gemini
            | CLIAgent::Auggie
    )
}

/// Creates the appropriate handler for the given CLI agent.
fn create_handler(agent: &CLIAgent) -> Option<Box<dyn CLIAgentSessionHandler>> {
    match agent {
        // Auggie is supported via the community-maintained auggie-warp plugin
        // (https://github.com/augmentmoogi/auggie-warp), which emits the same
        // structured OSC 777 events as the first-party Claude/OpenCode/Gemini
        // plugins. We don't ship an install flow for it — we just listen.
        CLIAgent::Claude | CLIAgent::OpenCode | CLIAgent::Gemini | CLIAgent::Auggie => {
            Some(Box::new(DefaultSessionListener))
        }
        CLIAgent::Codex | CLIAgent::Grok => Some(Box::new(Osc9FallbackSessionHandler {
            agent: *agent,
            rich_seen: std::cell::Cell::new(false),
        })),
        CLIAgent::Amp
        | CLIAgent::Droid
        | CLIAgent::Copilot
        | CLIAgent::Pi
        | CLIAgent::CursorCli
        | CLIAgent::Kiro
        | CLIAgent::Unknown => None,
    }
}

/// Default handler shared by agents whose events need no special filtering
/// beyond skipping the initial `SessionStart`.
struct DefaultSessionListener;

impl CLIAgentSessionHandler for DefaultSessionListener {
    fn handle_event(&mut self, event: CLIAgentEvent) -> Option<CLIAgentEvent> {
        // Skip session_start events (handled during listener construction)
        if event.event == CLIAgentEventType::SessionStart {
            return None;
        }

        Some(event)
    }
}

/// Handler for Codex/Grok plain-text OSC 9 desktop notifications
/// into CLI agent events.
///
/// Codex sends notifications via OSC 9 (`\x1b]9;message\x07`) with
/// human-readable text. Since there's no way to distinguish notification types
/// from the raw text, all OSC 9 notifications are treated as `Stop` (success).
/// The notification body becomes the event's `query` so it surfaces as the
/// notification title in the UI.
struct Osc9FallbackSessionHandler {
    agent: CLIAgent,
    rich_seen: std::cell::Cell<bool>,
}

impl Osc9FallbackSessionHandler {
    /// Parse a plain-text OSC 9 notification body into a `CLIAgentEvent`.
    /// Returns `None` only for empty bodies.
    fn parse_osc9_text(agent: CLIAgent, body: &str) -> Option<CLIAgentEvent> {
        let body = body.trim();
        if body.is_empty() {
            return None;
        }

        Some(CLIAgentEvent {
            v: 1,
            agent,
            event: CLIAgentEventType::Stop,
            session_id: None,
            cwd: None,
            project: None,
            payload: CLIAgentEventPayload {
                query: Some(body.to_owned()),
                ..Default::default()
            },
        })
    }
}

impl CLIAgentSessionHandler for Osc9FallbackSessionHandler {
    /// Accept structured events for this agent, falling back to OSC 9 until the rich plugin is seen.
    fn try_parse(&self, title: Option<&str>, body: &str) -> Option<CLIAgentEvent> {
        // If the notification carries the structured sentinel, try the normal
        // JSON parser first (future-proofing in case Codex adds plugin
        // support later).
        if let Some(parsed) = parse_event(title, body) {
            if parsed.agent != self.agent {
                return None;
            }
            self.rich_seen.set(true);
            return Some(parsed);
        }
        // OSC 9 notifications have no title.
        if title.is_some() || self.rich_seen.get() {
            return None;
        }
        Self::parse_osc9_text(self.agent, body)
    }

    fn handle_event(&mut self, event: CLIAgentEvent) -> Option<CLIAgentEvent> {
        Some(event)
    }

    fn supports_rich_status(&self) -> bool {
        self.agent == CLIAgent::Grok
    }
}

/// Per-agent listener that subscribes to PTY events and forwards them to the
/// sessions model. Stored on [`super::CLIAgentSession`] so its lifetime is
/// tied to the session; dropping the handle cleans up the subscription.
pub struct CLIAgentSessionListener {
    terminal_view_id: EntityId,
    inner: Box<dyn CLIAgentSessionHandler>,
}

impl warpui::Entity for CLIAgentSessionListener {
    type Event = ();
}

impl CLIAgentSessionListener {
    pub fn new(
        terminal_view_id: EntityId,
        agent: CLIAgent,
        model_event_dispatcher: &ModelHandle<ModelEventDispatcher>,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        let handler =
            create_handler(&agent).expect("is_agent_supported must be checked before calling new");

        // Subscribe to subsequent OSC events from this terminal's PTY.
        // Parsing is delegated to the handler's `try_parse`; the handler's
        // `handle_event` then filters/transforms the result.
        ctx.subscribe_to_model(model_event_dispatcher, move |me, event, ctx| {
            if let ModelEvent::PluggableNotification { title, body } = event {
                // A rich session may have started before this listener was registered.
                if title.is_none()
                    && CLIAgentSessionsModel::as_ref(ctx)
                        .session(me.terminal_view_id)
                        .is_some_and(|session| {
                            matches!(session.agent, CLIAgent::Codex | CLIAgent::Grok)
                                && session.plugin_version.is_some()
                        })
                {
                    return;
                }
                let Some(parsed) = me.inner.try_parse(title.as_deref(), body) else {
                    return;
                };
                if let Some(event) = me.inner.handle_event(parsed) {
                    CLIAgentSessionsModel::handle(ctx).update(ctx, |sessions_model, ctx| {
                        sessions_model.update_from_event(me.terminal_view_id, &event, ctx);
                    });
                }
            }
        });

        Self {
            terminal_view_id,
            inner: handler,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal::cli_agent_sessions::event::CLIAgentEventType;

    #[test]
    fn codex_parses_any_text_as_stop() {
        let event =
            Osc9FallbackSessionHandler::parse_osc9_text(CLIAgent::Codex, "Agent turn complete")
                .unwrap();
        assert_eq!(event.event, CLIAgentEventType::Stop);
        assert_eq!(event.agent, CLIAgent::Codex);
        assert_eq!(event.payload.query.as_deref(), Some("Agent turn complete"));
    }

    #[test]
    fn codex_body_becomes_query() {
        let event = Osc9FallbackSessionHandler::parse_osc9_text(
            CLIAgent::Codex,
            "I've updated the README with the new instructions.",
        )
        .unwrap();
        assert_eq!(event.event, CLIAgentEventType::Stop);
        assert_eq!(
            event.payload.query.as_deref(),
            Some("I've updated the README with the new instructions.")
        );
    }

    #[test]
    fn codex_approval_text_still_becomes_stop() {
        let event = Osc9FallbackSessionHandler::parse_osc9_text(
            CLIAgent::Codex,
            "Approval requested: rm -rf /tmp/foo",
        )
        .unwrap();
        assert_eq!(event.event, CLIAgentEventType::Stop);
        assert_eq!(
            event.payload.query.as_deref(),
            Some("Approval requested: rm -rf /tmp/foo")
        );
    }

    #[test]
    fn codex_ignores_empty_body() {
        assert!(Osc9FallbackSessionHandler::parse_osc9_text(CLIAgent::Codex, "").is_none());
        assert!(Osc9FallbackSessionHandler::parse_osc9_text(CLIAgent::Codex, "   ").is_none());
    }

    #[test]
    fn codex_try_parse_ignores_titled_notifications() {
        let handler = Osc9FallbackSessionHandler {
            agent: CLIAgent::Codex,
            rich_seen: std::cell::Cell::new(false),
        };
        assert!(handler
            .try_parse(Some("some-title"), "Agent turn complete")
            .is_none());
    }

    #[test]
    fn codex_try_parse_handles_osc9() {
        let handler = Osc9FallbackSessionHandler {
            agent: CLIAgent::Codex,
            rich_seen: std::cell::Cell::new(false),
        };
        let event = handler.try_parse(None, "Agent turn complete").unwrap();
        assert_eq!(event.event, CLIAgentEventType::Stop);
    }

    #[test]
    fn auggie_is_supported() {
        assert!(is_agent_supported(&CLIAgent::Auggie));
    }

    #[test]
    fn auggie_uses_default_handler_with_rich_status() {
        assert!(agent_supports_rich_status(&CLIAgent::Auggie));
    }

    #[test]
    fn auggie_default_handler_skips_session_start() {
        let mut handler = DefaultSessionListener;
        let event = CLIAgentEvent {
            v: 1,
            agent: CLIAgent::Auggie,
            event: CLIAgentEventType::SessionStart,
            session_id: None,
            cwd: None,
            project: None,
            payload: CLIAgentEventPayload::default(),
        };
        assert!(handler.handle_event(event).is_none());
    }

    #[test]
    fn auggie_default_handler_forwards_stop() {
        let mut handler = DefaultSessionListener;
        let event = CLIAgentEvent {
            v: 1,
            agent: CLIAgent::Auggie,
            event: CLIAgentEventType::Stop,
            session_id: None,
            cwd: None,
            project: None,
            payload: CLIAgentEventPayload::default(),
        };
        assert!(handler.handle_event(event).is_some());
    }
}

#[cfg(test)]
mod grok_tests {
    use super::*;

    #[test]
    fn grok_osc9_fallback_has_grok_identity_and_stops_after_rich_plugin() {
        let handler = create_handler(&CLIAgent::Grok).unwrap();
        assert!(is_agent_supported(&CLIAgent::Grok));
        let event = handler.try_parse(None, "done").unwrap();
        assert_eq!(event.agent, CLIAgent::Grok);
        assert_eq!(event.event, CLIAgentEventType::Stop);
        handler
            .try_parse(
                Some("warp://cli-agent"),
                r#"{"v":1,"agent":"grok","event":"prompt_submit"}"#,
            )
            .unwrap();
        assert!(handler.try_parse(None, "done").is_none());
        assert!(handler.try_parse(None, "  ").is_none());
    }

    #[test]
    fn grok_rich_events_do_not_accept_another_agents_identity() {
        let handler = create_handler(&CLIAgent::Grok).unwrap();
        let title = Some("warp://cli-agent");
        let event = handler
            .try_parse(title, r#"{"v":1,"agent":"grok","event":"prompt_submit"}"#)
            .unwrap();
        assert_eq!(event.agent, CLIAgent::Grok);
        assert_eq!(event.event, CLIAgentEventType::PromptSubmit);
        assert!(handler
            .try_parse(title, r#"{"v":1,"agent":"codex","event":"stop"}"#)
            .is_none());
    }
}
