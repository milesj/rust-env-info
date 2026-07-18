# ai_env

![Crates.io](https://img.shields.io/crates/v/ai_env) ![Crates.io](https://img.shields.io/crates/d/ai_env)

Detects which AI coding agent (Claude Code, Codex, Cursor, Gemini, etc.) is running the current process, by inspecting agent-specific environment variables and filesystem markers.

### Usage

To start, detect if an AI agent is running the process.

```rust
ai_env::is_ai_agent();
```

Or detect which agent is being used.

```rust
ai_env::detect_agent(); // Claude
```

And extract information about the agent environment.

```rust
use ai_env::get_environment;

if let Some(ai) = get_environment() {
	println!("Agent: {:?}", ai.agent);

	if let Some(id) = ai.id {
		println!("Self-reported: {}", id);
	}

	if ai.sandboxed {
		println!("Running in a sandbox");
	}
}
```

Additionally, detect whether the environment restricts network egress, e.g. the
GitHub Copilot cloud agent firewall, Codex sandboxes with network access turned
off, or local filtering proxies like the Claude Code sandbox.

```rust
use ai_env::{detect_agent, detect_network_policy, AiNetworkPolicy};

match detect_network_policy(detect_agent()) {
	AiNetworkPolicy::Disabled => println!("No network access"),
	AiNetworkPolicy::Filtered => println!("Egress restricted to an allowlist"),
	AiNetworkPolicy::Open => println!("Egress explicitly unrestricted"),
	AiNetworkPolicy::Unknown => println!("No signal"),
}
```

This is derived from environment variables only — no network I/O is performed.
A `Filtered` network may still allow the hosts your program needs, so verify
reachability of those hosts before acting on the signal.
