# Nushell LSP Lapce Plugin

Nushell language server integration.

## Features

Supports all LSP features that the configured language server implementation supports.

- Syntax highlighting
- Error reporting
- Code completion

## Configure

Configure path to LSP server in Lapce `settings.toml`.

```toml
[nushell-lsp]
path = "/usr/bin/nu"
args = [ "--lsp" ]
```

By default, nu from `PATH` is used with the config looking like this.

```toml
[nushell-lsp]
path = "/usr/bin/env"
args = [ "nu", "--lsp", "--no-config-file" ]
```

## Technical

Uses [nushell's](https://github.com/nushell/nushell) [integrated LSP](https://github.com/nushell/nushell/tree/main/crates/nu-lsp) by default (`nu --lsp --no-config-file`).
