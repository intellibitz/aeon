# aeon

![AEON Version](https://img.shields.io/badge/version-v0.1.2022738-blue.svg) ![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)

**aeon** is a Rust-based local-first AI execution engine. It provides a sub-2ms CLI launcher that proxies commands to a persistent background daemon for zero-latency tool execution and agent orchestration.

## Architecture

* **Binary Core**: Immutable system rules and agent definitions are compiled directly into the binary.
* **Model Routing**: Prioritizes local runtimes (Candle tensor substrates) before falling back to cloud inference.
* **Hardware-Aware Selection**: Automatically profiles system RAM, VRAM, and GPU acceleration to identify and default to the best-suited local model.
* **Capabilities**: MCP (Model Context Protocol) integration, file I/O, system command execution, and autonomous task looping.

## Installation

### Platform-Independent One-Liner (Linux / macOS / Windows / WSL)
```bash
curl -sSfL https://raw.githubusercontent.com/intellibitz/aeon/main/install.sh | sh
```

## Configuration

`aeon` is designed to be zero-config, but you can customize its behavior via environment variables or a local `.env` file in `~/.aeon/env`.

* **AEON_REPO**: Custom GitHub repository for binary updates (defaults to `intellibitz/aeon`).

## Usage

Execute commands or natural language tasks directly:
```bash
aeon "read BUILD.md and agents.md"
aeon "list installed Android Studio versions"
aeon status
aeon mcp
```

## Maintainers

* IntelliBitz
* Muthu Ramadoss
* Gemini (Google AI)

## License

[Apache License 2.0](LICENSE)
