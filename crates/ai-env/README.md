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
