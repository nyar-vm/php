# Architecture

## One pipeline

```
Boost language.php / data.text.php
        │  pure Rust parsing, lexing, highlighting
        ▼
packages/php.rs/php-npai  (crate php-napi, cdylib + rlib)
        │  N-API objects: parse tree, token spans, diagnostics, highlight spans
        ▼
packages/php.ts/platforms/php-<platform>/lib/php.<triple>.node
        │  prebuilt addon selected by host OS/arch
        ▼
@nyar/php (native.ts → CLI + API) · @nyar/php-harness
```

The homepage (`@nyar/php-homepage`) is a separate VMZ application. Its user docs
are Markdown under `packages/php.ts/homepage/documents/` and mount at `/d/`.

## Ownership boundary

| Layer | Owner | May change |
|-------|-------|-----------|
| PHP grammar, lexer, highlighter | Boost `language.php` / `data.text.php` | No (bump the revision) |
| N-API object shape and exports | `php-napi` | Yes |
| Host binding resolution and CLI | `@nyar/php` | Yes |
| Diagnostics and latency loop | `@nyar/php-harness` | Yes |
| User docs | homepage `documents/` | Yes (all locales) |

`php-napi` must stay a projection layer. If a capability is missing, add it to
Boost and raise the pinned revision instead of reimplementing it here.

## Parse data flow

1. `phpParse(source)` builds a `PhpLanguage`, `PhpParser`, `PhpLexer` and a
   `ParseSession`, then calls `data_text_core::parser::parse`.
2. The green tree is walked once. Nodes become `SyntaxNode`; leaves become both
   `SyntaxNode` children and `TokenSpan` rows. Offsets are byte offsets.
3. `phpCheck` returns the session diagnostics; a fatal error is placed first.
4. `phpTokens` and `phpHighlight` reuse the same entry points so spans stay
   consistent across commands.

## Binding resolution

`native.ts` resolves the addon in this order:

1. `PHP_NATIVE` when set;
2. `@nyar/php-<short>/lib/php.<triple>.node` via `require.resolve`;
3. repository-relative `platforms/php-<short>/lib/...` (local development);
4. `node_modules/@nyar/php-<short>/...`.

If none exist the error lists every path tried. Rebuild with
`node scripts/build.mjs napi`.

## Deliberate non-goals

- No JavaScript parser fallback.
- No per-language code in the CLI.
- No committed `.node` binaries: platform packages are populated at release time.
