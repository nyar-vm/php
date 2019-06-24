# php-napi

Node-API export layer for PHP. The parser, lexer and highlighter all come from
Boost `language.php` / `data.text.php`; this crate only maps pure Rust results into
host objects.

## Exports

- `phpVersion()` — binding version.
- `phpParse(source)` — syntax tree (node kind + byte span).
- `phpTokens(source)` — lexical leaves.
- `phpCheck(source)` — diagnostics.
- `phpHighlight(source)` — keyword/string/number/comment/identifier spans.

## Build

```bash
cargo build -p php-napi
node scripts/build.mjs napi
```
