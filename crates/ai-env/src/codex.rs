use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// OpenAI Codex — sets CODEX_SANDBOX (e.g. `seatbelt`) when sandboxing commands,
// and CODEX_THREAD_ID for the active conversation thread. Sandboxed commands
// with network access turned off also set CODEX_SANDBOX_NETWORK_DISABLED,
// which `detect_network_policy` reports as `Disabled`.
// https://developers.openai.com/codex/environment-variables
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Codex,
        env_prefix: Some("CODEX_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::Codex),
        sandboxed: opt_var("CODEX_SANDBOX").is_some(),
        session_id: opt_var("CODEX_THREAD_ID"),
    }
}
