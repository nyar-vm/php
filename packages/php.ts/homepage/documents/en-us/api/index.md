# API

All offsets are byte offsets into the source string.

## Parse

```ts
import { parse } from '@nyar/php';

const outcome = parse('<?php echo 1;');

outcome.ok;          // true when a tree was produced
outcome.error;       // fatal error message, or null
outcome.diagnostics; // non-fatal messages
outcome.tokenCount;  // number of leaves
outcome.root;        // SyntaxNode | null
```

`SyntaxNode`:

```ts
type SyntaxNode = {
    kind: string;
    start: number;
    end: number;
    children: SyntaxNode[];
};
```

## Tokens

```ts
import { tokens } from '@nyar/php';

for (const token of tokens('<?php function f() {}')) {
    console.log(token.kind, token.start, token.end);
}
```

## Diagnostics

```ts
import { check } from '@nyar/php';

const messages = check('<?php class {');
for (const message of messages) console.error(message);
```

A fatal error, when present, is the first entry.

## Highlighting

```ts
import { highlight } from '@nyar/php';

const spans = highlight('<?php function f() {}');
// [{ start, end, kind: 'keyword' | 'string' | 'number' | 'comment' | 'identifier' }]
```

## Version

```ts
import { version } from '@nyar/php';
console.log(version());
```
