# CLI

The `php` command is a thin wrapper over the native binding.

## Commands

| Command | Purpose |
|---------|---------|
| `php version` | Print the binding version |
| `php parse <file> [--json]` | Parse a file and report the syntax tree |
| `php tokens <file> [--json]` | List tokens with byte spans |
| `php check <file...>` | Report diagnostics for one or more files |
| `php highlight <file> [--json]` | List highlight spans |

## Examples

```bash
php version
php parse src/Controller.php
php parse src/Controller.php --json
php tokens src/Controller.php
php check src/*.php
php highlight src/Controller.php
```

## Output

- `parse` prints `parsed <file> (<n> token(s), <m> diagnostic(s))`, or the fatal
  error. With `--json` it prints the full `ParseOutcome`.
- `tokens` prints `kind`, `start`, `end` per line. With `--json` it prints
  `TokenSpan[]`.
- `check` prints `ok` or the diagnostics per file and a summary line.
- `highlight` prints `kind`, `start`, `end` per line. With `--json` it prints
  `HighlightSpan[]`.

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | success |
| 1 | parse failed or diagnostics were reported by `check` |
| 2 | missing or unreadable input |
