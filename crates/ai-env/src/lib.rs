mod antigravity;
mod api;
mod augment;
mod claude;
mod claude_cowork;
mod codex;
mod cursor;
mod cursor_cli;
mod devin;
mod gemini;
mod github_copilot;
mod opencode;
mod replit;
mod v0;

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
///
/// Unlike the `ci_env`/`cd_env` crates, detection isn't a flat key table:
/// several agents need value checks (`CURSOR_EXTENSION_HOST_ROLE`), multiple
/// alternative markers (Codex, Copilot), or a sub-mode gate (Claude vs its
/// Cowork mode), so the checks are spelled out explicitly. Markers are checked
/// before the `AI_AGENT` self-id so the Cowork sub-mode is detected even when
/// `AI_AGENT` is also set to the generic Claude Code string.
fn detect_agent_from_vars(vars: &[(String, String)]) -> AiAgent {
    let get = |key: &str| {
        vars.iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .filter(|v| !v.is_empty())
    };
    let any = |keys: &[&str]| keys.iter().any(|key| get(key).is_some());

    // Cursor editor integrated terminal
    if get("CURSOR_TRACE_ID").is_some() {
        return AiAgent::Cursor;
    }

    // Cursor CLI agent (env marker or the agent-exec extension host role)
    if get("CURSOR_AGENT").is_some() || get("CURSOR_EXTENSION_HOST_ROLE") == Some("agent-exec") {
        return AiAgent::CursorCli;
    }

    if get("GEMINI_CLI").is_some() {
        return AiAgent::Gemini;
    }

    // Codex — sandboxed exec, CI, or an interactive thread
    if any(&["CODEX_SANDBOX", "CODEX_CI", "CODEX_THREAD_ID"]) {
        return AiAgent::Codex;
    }

    if get("ANTIGRAVITY_AGENT").is_some() {
        return AiAgent::Antigravity;
    }

    if get("AUGMENT_AGENT").is_some() {
        return AiAgent::Augment;
    }

    if any(&["OPENCODE_CLIENT", "OPENCODE"]) {
        return AiAgent::OpenCode;
    }

    // Claude Code, or its Cowork sub-mode
    if any(&["CLAUDECODE", "CLAUDE_CODE"]) {
        return if get("CLAUDE_CODE_IS_COWORK").is_some() {
            AiAgent::ClaudeCowork
        } else {
            AiAgent::Claude
        };
    }

    if get("REPL_ID").is_some() {
        return AiAgent::Replit;
    }

    // GitHub Copilot CLI
    if any(&["COPILOT_MODEL", "COPILOT_ALLOW_ALL", "COPILOT_GITHUB_TOKEN"]) {
        return AiAgent::GithubCopilot;
    }

    // Fall back to the `AI_AGENT` self-identification string. Agents that only
    // self-identify (no boolean marker, e.g. v0) are classified by name.
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

/// Maps an `AI_AGENT` self-identification string to a known agent. A few names
/// are matched exactly (`v0`, `github-copilot`); the rest by name prefix
/// (e.g. `claude-code_2-1-170_agent` -> `Claude`).
fn classify_self_id(value: &str) -> AiAgent {
    let name = value.trim().to_ascii_lowercase();

    if name == "v0" {
        AiAgent::V0
    } else if name == "github-copilot" || name == "github-copilot-cli" {
        AiAgent::GithubCopilot
    } else if name.starts_with("claude") {
        AiAgent::Claude
    } else if name.starts_with("cowork") {
        AiAgent::ClaudeCowork
    } else if name.starts_with("codex") {
        AiAgent::Codex
    } else if name.starts_with("cursor") {
        AiAgent::Cursor
    } else if name.starts_with("gemini") {
        AiAgent::Gemini
    } else if name.starts_with("antigravity") {
        AiAgent::Antigravity
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
        AiAgent::Antigravity => antigravity::create_environment(),
        AiAgent::Augment => augment::create_environment(),
        AiAgent::Claude => claude::create_environment(),
        AiAgent::ClaudeCowork => claude_cowork::create_environment(),
        AiAgent::Codex => codex::create_environment(),
        AiAgent::Cursor => cursor::create_environment(),
        AiAgent::CursorCli => cursor_cli::create_environment(),
        AiAgent::Devin => devin::create_environment(),
        AiAgent::Gemini => gemini::create_environment(),
        AiAgent::GithubCopilot => github_copilot::create_environment(),
        AiAgent::OpenCode => opencode::create_environment(),
        AiAgent::Replit => replit::create_environment(),
        AiAgent::V0 => v0::create_environment(),
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
    fn cowork_mode_distinguished_from_claude() {
        let env = vars(&[("CLAUDECODE", "1"), ("CLAUDE_CODE_IS_COWORK", "1")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::ClaudeCowork);
    }

    #[test]
    fn detects_codex_via_sandbox_or_thread() {
        assert_eq!(
            detect_agent_from_vars(&vars(&[("CODEX_SANDBOX", "seatbelt")])),
            AiAgent::Codex
        );
        // CODEX_THREAD_ID marks Codex even outside a sandbox
        assert_eq!(
            detect_agent_from_vars(&vars(&[("CODEX_THREAD_ID", "th_123")])),
            AiAgent::Codex
        );
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
    fn cursor_cli_via_extension_host_role() {
        // Only the `agent-exec` role counts, not the editor's other roles
        assert_eq!(
            detect_agent_from_vars(&vars(&[("CURSOR_EXTENSION_HOST_ROLE", "agent-exec")])),
            AiAgent::CursorCli
        );
        assert_eq!(
            detect_agent_from_vars(&vars(&[("CURSOR_EXTENSION_HOST_ROLE", "extension")])),
            AiAgent::Unknown
        );
    }

    #[test]
    fn detects_gemini() {
        let env = vars(&[("GEMINI_CLI", "1")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Gemini);
    }

    #[test]
    fn detects_antigravity() {
        let env = vars(&[("ANTIGRAVITY_AGENT", "1")]);

        assert_eq!(detect_agent_from_vars(&env), AiAgent::Antigravity);
    }

    #[test]
    fn detects_github_copilot() {
        assert_eq!(
            detect_agent_from_vars(&vars(&[("COPILOT_MODEL", "gpt-5")])),
            AiAgent::GithubCopilot
        );
        assert_eq!(
            detect_agent_from_vars(&vars(&[("COPILOT_GITHUB_TOKEN", "x")])),
            AiAgent::GithubCopilot
        );
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
        assert_eq!(
            detect_agent_from_vars(&vars(&[("AI_AGENT", "claude-code_2-1-170_agent")])),
            AiAgent::Claude
        );
        // v0 only ever identifies through AI_AGENT
        assert_eq!(
            detect_agent_from_vars(&vars(&[("AI_AGENT", "v0")])),
            AiAgent::V0
        );
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
