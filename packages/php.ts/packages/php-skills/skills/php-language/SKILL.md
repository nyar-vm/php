---
name: php-language
description: Use @nyar/php and php-harness to parse PHP, inspect tokens, triage diagnostics, and tune native parse throughput.
---

# PHP Language Services

Use this skill when a user asks to parse PHP, list tokens, collect parser
diagnostics, highlight PHP source, or benchmark the native binding.

## Ownership boundary

- The parser, lexer and highlighter live in Boost `language.php` / `data.text.php`.
- `@nyar/php` is only an npm surface over the `php-napi` Node-API addon.
- Do not add parsing fallbacks in JavaScript. If a capability is missing, extend
  `php-napi` and rebuild the platform binding.

## Commands

| Command | Purpose |
|---------|---------|
| `php version` | Binding version |
| `php parse <file> [--json]` | Syntax tree with byte spans |
| `php tokens <file> [--json]` | Token kinds and spans |
| `php check <file...>` | Parser diagnostics |
| `php highlight <file> [--json]` | Keyword/string/number/comment/identifier spans |
| `php-harness bench <file> -n 200` | Repeated parse latency |
| `php-harness tokens <file>` | Token counts by kind |

## API

```ts
import { parse, tokens, check, highlight } from '@nyar/php';

const outcome = parse('<?php echo 1;');
if (!outcome.ok) console.error(outcome.error, outcome.diagnostics);
```

## Native resolution

`@nyar/php` loads `@nyar/php-<platform>/lib/php.<triple>.node`. Set `PHP_NATIVE`
to validate a freshly built addon without reinstalling platform packages.

## Rebuild

```bash
node scripts/build.mjs napi
node scripts/build.mjs napi --all
```
