use serde::{Deserialize, Serialize};
use std::env;

/// List of detectable AI coding agents.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum AiAgent {
    Antigravity,
    Augment,
    Claude,
    ClaudeCowork,
    Codex,
    Cursor,
    CursorCli,
    Devin,
    Gemini,
    GithubCopilot,
    OpenCode,
    Replit,
    V0,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AiEnvironment {
    /// The detected AI agent.
    pub agent: AiAgent,

    /// Prefix that the agent's environment variables use.
    pub env_prefix: Option<String>,

    /// Raw self-identification reported via `AI_AGENT`, when provided. Often
    /// encodes the agent name and version (e.g. `claude-code_2-1-170_agent`).
    pub id: Option<String>,

    /// Whether the agent runs the process inside a sandbox.
    pub sandboxed: bool,

    /// Unique ID of the current agent session, when exposed.
    pub session_id: Option<String>,
}

pub fn opt_var(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(value) if value.is_empty() => None,
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

/// Returns the raw `AI_AGENT` self-identification string, if set. This is the
/// emerging convention for agents to announce themselves to child processes.
pub fn self_id() -> Option<String> {
    opt_var("AI_AGENT")
}
