# @nyar/php-harness

Diagnostics and tuning harness for `@nyar/php`. It links the same native binding
through the platform optional dependencies, so a failing harness points at the same
addon the product CLI uses.

## Usage

```bash
php-harness version
php-harness check src/*.php
php-harness bench path/to/file.php --iterations 200
php-harness tokens path/to/file.php
```
