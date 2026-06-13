use serde::{Deserialize, Serialize};
use std::{env, fmt};

/// List of detectable AI coding agents.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
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
    #[serde(rename = "opencode")]
    OpenCode,
    Replit,
    V0,
    #[default]
    Unknown,
}

impl fmt::Display for AiAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = match self {
            AiAgent::Antigravity => "antigravity",
            AiAgent::Augment => "augment",
            AiAgent::Claude => "claude",
            AiAgent::ClaudeCowork => "claude-cowork",
            AiAgent::Codex => "codex",
            AiAgent::Cursor => "cursor",
            AiAgent::CursorCli => "cursor-cli",
            AiAgent::Devin => "devin",
            AiAgent::Gemini => "gemini",
            AiAgent::GithubCopilot => "github-copilot",
            AiAgent::OpenCode => "opencode",
            AiAgent::Replit => "replit",
            AiAgent::V0 => "v0",
            AiAgent::Unknown => "unknown",
        };

        f.write_str(id)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::{Error as DeError, StrDeserializer};
    use serde::de::IntoDeserializer;

    fn from_id(id: &str) -> Result<AiAgent, DeError> {
        let de: StrDeserializer<DeError> = id.into_deserializer();
        AiAgent::deserialize(de)
    }

    #[test]
    fn display_uses_kebab_case_ids() {
        assert_eq!(AiAgent::Claude.to_string(), "claude");
        assert_eq!(AiAgent::ClaudeCowork.to_string(), "claude-cowork");
        assert_eq!(AiAgent::GithubCopilot.to_string(), "github-copilot");
        assert_eq!(AiAgent::Unknown.to_string(), "unknown");
    }

    #[test]
    fn serde_round_trips_display_ids() {
        // The serde wire format must match `Display` exactly, including the
        // `opencode` override where serde's mechanical kebab-case differs.
        for agent in [
            AiAgent::ClaudeCowork,
            AiAgent::GithubCopilot,
            AiAgent::OpenCode,
            AiAgent::Unknown,
        ] {
            let id = agent.to_string();
            assert_eq!(from_id(&id).unwrap(), agent, "id {id:?}");
        }
    }
}
