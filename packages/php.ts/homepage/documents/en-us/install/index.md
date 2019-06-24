# Install

## Requirements

- Node.js 22.18 or newer.
- A supported host: Windows x64, Linux x64, macOS x64 or macOS arm64.

## npm

```bash
npm install @nyar/php
```

`@nyar/php` declares one optional dependency per host
(`@nyar/php-win32-x64`, `@nyar/php-linux-x64`, `@nyar/php-darwin-x64`,
`@nyar/php-darwin-arm64`). npm installs only the one matching your OS and CPU.

## Verify

```bash
php version
php parse example.php
```

If the addon cannot be found, `php version` prints every path it tried. See
[Platforms and troubleshooting](../native.md).

## Using a custom addon

Point `PHP_NATIVE` at a `.node` file to bypass platform resolution:

```bash
PHP_NATIVE=/absolute/path/php.win32-x64-msvc.node php version
```

This is useful when testing a local build before publishing a platform package.
