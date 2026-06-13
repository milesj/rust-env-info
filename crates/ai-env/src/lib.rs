mod api;
mod augment;
mod claude;
mod codex;
mod cursor;
mod cursor_cli;
mod devin;
mod gemini;
mod opencode;
mod replit;

pub use api::{AiAgent, AiEnvironment};

use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Returns true if the process is being run by an AI coding agent.
pub fn is_ai_agent() -> bool {
    get_environment().is_some()
}

static AGENT: OnceLock<AiAgent> = OnceLock::new();

/// Detects the AI agent running the process. Returns `Unknown` when no known
/// agent is matched.
///
/// Note that an unrecognized agent may still be present via the `AI_AGENT`
/// self-identification variable; prefer [`is_ai_agent`] or [`get_environment`]
/// to detect that case.
pub fn detect_agent() -> AiAgent {
    *AGENT.get_or_init(|| {
        let vars = env::vars().collect::<Vec<_>>();

        detect_agent_from_vars(&vars)
    })
}

/// Detection markers in priority order, mirroring `@vercel/detect-agent`.
/// Cursor's editor terminal (`CURSOR_TRACE_ID`) ranks above the CLI markers,
/// matching that convention.
#[rustfmt::skip]
const AGENT_KEYS: &[(&str, AiAgent)] = &[
    ("CURSOR_TRACE_ID", AiAgent::Cursor),
    ("CURSOR_AGENT", AiAgent::CursorCli),
    ("GEMINI_CLI", AiAgent::Gemini),
    ("CODEX_SANDBOX", AiAgent::Codex),
    ("AUGMENT_AGENT", AiAgent::Augment),
    ("OPENCODE_CLIENT", AiAgent::OpenCode),
    ("OPENCODE", AiAgent::OpenCode),
    ("CLAUDECODE", AiAgent::Claude),
    ("CLAUDE_CODE", AiAgent::Claude),
    ("REPL_ID", AiAgent::Replit),
];

fn detect_agent_from_vars(vars: &[(String, String)]) -> AiAgent {
    let get = |key: &str| {
        vars.iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .filter(|v| !v.is_empty())
    };

    for (key, agent) in AGENT_KEYS {
        if get(key).is_some() {
            return *agent;
        }
    }

    // Fall back to the `AI_AGENT` self-identification string. Agents that only
    // self-identify (no boolean marker) are classified by their name prefix.
    if let Some(value) = get("AI_AGENT") {
        let agent = classify_self_id(value);

        if !matches!(agent, AiAgent::Unknown) {
            return agent;
        }
    }

    if PathBuf::from("/opt/.devin").exists() {
        return AiAgent::Devin;
    }

    AiAgent::Unknown
}

/// Maps an `AI_AGENT` self-identification string to a known agent by its name
/// prefix (e.g. `claude-code_2-1-170_agent` -> `Claude`).
fn classify_self_id(value: &str) -> AiAgent {
    let name = value.trim().to_ascii_lowercase();

    if name.starts_with("claude") {
        AiAgent::Claude
    } else if name.starts_with("codex") {
        AiAgent::Codex
    } else if name.starts_with("cursor") {
        AiAgent::Cursor
    } else if name.starts_with("gemini") {
        AiAgent::Gemini
    } else if name.starts_with("augment") {
        AiAgent::Augment
    } else if name.starts_with("opencode") {
        AiAgent::OpenCode
    } else if name.starts_with("devin") {
        AiAgent::Devin
    } else if name.starts_with("replit") {
        AiAgent::Replit
    } else {
        AiAgent::Unknown
    }
}

/// Returns metadata about the current AI agent environment, or `None` when no
/// agent is detected.
pub fn get_environment() -> Option<AiEnvironment> {
    let environment = match detect_agent() {
        AiAgent::Augment => augment::create_environment(),
        AiAgent::Claude => claude::create_environment(),
        AiAgent::Codex => codex::create_environment(),
        AiAgent::Cursor => cursor::create_environment(),
        AiAgent::CursorCli => cursor_cli::create_environment(),
        AiAgent::Devin => devin::create_environment(),
        AiAgent::Gemini => gemini::create_environment(),
        AiAgent::OpenCode => opencode::create_environment(),
        AiAgent::Replit => replit::create_environment(),
        AiAgent::Unknown => {
            // No typed agent matched. If something still self-identified via
            // `AI_AGENT`, surface it generically; otherwise no agent is present.
            return api::self_id().map(|id| AiEnvironment {
                agent: AiAgent::Unknown,
                env_prefix: None,
                id: Some(id),
                sandboxed: false,
                session_id: None,
            });
        }
    };

    Some(environment)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(list: &[(&str, &str)]) -> Vec<(String, String)> {
        list.iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect()
    }

    #[test]
    fn detects_claude_both_spellings() {
        assert_eq!(
            detect_agent_from_vars(&vars(&[("CLAUDECODE", "1")])),
            AiAgent::Claude
        );
        assert_eq!(
            detect_agent_from_vars(&vars(&[("CLAUDE_CODE", "1")])),
            AiAgent::Claude
        );
    }

    #[test]
    fn detects_codex_sandbox() {
        let env = vars(&[("CODEX_SANDBOX", "seatbelt")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Codex);
    }

    #[test]
    fn detects_cursor_editor_and_cli() {
        assert_eq!(
            detect_agent_from_vars(&vars(&[("CURSOR_TRACE_ID", "abc")])),
            AiAgent::Cursor
        );
        assert_eq!(
            detect_agent_from_vars(&vars(&[("CURSOR_AGENT", "1")])),
            AiAgent::CursorCli
        );
    }

    #[test]
    fn detects_gemini() {
        let env = vars(&[("GEMINI_CLI", "1")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Gemini);
    }

    #[test]
    fn detects_opencode_either_marker() {
        assert_eq!(
            detect_agent_from_vars(&vars(&[("OPENCODE", "1")])),
            AiAgent::OpenCode
        );
        assert_eq!(
            detect_agent_from_vars(&vars(&[("OPENCODE_CLIENT", "x")])),
            AiAgent::OpenCode
        );
    }

    #[test]
    fn classifies_self_id_when_no_marker() {
        let env = vars(&[("AI_AGENT", "claude-code_2-1-170_agent")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Claude);
    }

    #[test]
    fn boolean_marker_and_self_id_agree() {
        // The real Claude Code environment sets both
        let env = vars(&[
            ("CLAUDECODE", "1"),
            ("AI_AGENT", "claude-code_2-1-170_agent"),
        ]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Claude);
    }

    #[test]
    fn cursor_editor_outranks_claude() {
        let env = vars(&[("CURSOR_TRACE_ID", "abc"), ("CLAUDECODE", "1")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Cursor);
    }

    #[test]
    fn unrecognized_self_id_is_unknown() {
        // Still "present" — surfaced generically by get_environment — but not
        // a typed agent
        let env = vars(&[("AI_AGENT", "some-new-tool")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Unknown);
    }

    #[test]
    fn empty_values_are_ignored() {
        let env = vars(&[("CLAUDECODE", "")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Unknown);
    }

    #[test]
    fn no_agent() {
        let env = vars(&[("PATH", "/usr/bin")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Unknown);
    }
}
