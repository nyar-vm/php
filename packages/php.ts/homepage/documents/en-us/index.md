# PHP language services

`@nyar/php` brings PHP parsing to Node.js through a native Node-API addon. It is
read-only: you give it source text, it gives you syntax trees, tokens, diagnostics
and highlight spans.

## What you get

| Export | Returns |
|--------|---------|
| `parse(source)` | `{ ok, error, diagnostics, root, tokenCount }` |
| `tokens(source)` | `TokenSpan[]` with byte offsets |
| `check(source)` | `string[]` diagnostics |
| `highlight(source)` | keyword/string/number/comment/identifier spans |
| `version()` | native binding version |

## Install

```bash
npm install @nyar/php
```

The matching platform package is installed automatically.

## Next

- [Install](./install.md)
- [CLI](./cli.md)
- [API](./api.md)
- [Platforms and troubleshooting](./native.md)
