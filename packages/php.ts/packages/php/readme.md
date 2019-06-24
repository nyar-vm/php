# @nyar/php

PHP language services for Node.js. The parser, lexer and highlighter come from Boost
`language.php` / `data.text.php`; this package only ships the npm surface and the
platform-specific native binding.

## Install

```bash
npm install @nyar/php
```

The matching `@nyar/php-<platform>` optional dependency is installed automatically.

## CLI

```bash
php version
php parse path/to/file.php
php parse path/to/file.php --json
php tokens path/to/file.php
php check src/*.php
php highlight path/to/file.php
```

## API

```ts
import { parse, tokens, check, highlight, version } from '@nyar/php';

const outcome = parse('<?php echo 1;');
console.log(outcome.ok, outcome.tokenCount);

for (const token of tokens('<?php function f() {}')) {
    console.log(token.kind, token.start, token.end);
}

const diagnostics = check('<?php class {');
const spans = highlight('<?php function f() {}');
```

## Native binding resolution

`PHP_NATIVE` overrides the `.node` path (useful when validating a fresh local build).
