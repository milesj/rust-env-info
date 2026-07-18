use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// Google Gemini CLI — sets GEMINI_CLI=1 in shell commands it runs
// https://google-gemini.github.io/gemini-cli/docs/tools/shell.html
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Gemini,
        env_prefix: Some("GEMINI_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::Gemini),
        sandboxed: false,
        session_id: opt_var("GEMINI_SESSION_ID"),
    }
}
