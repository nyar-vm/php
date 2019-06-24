import { createRequire } from 'node:module';

/**
 * `@nyar/php` — npm wrapper for PHP language services.
 *
 * This package declares commands and package identity only; parsing, lexing and
 * highlighting are delegated to `php-napi`, whose Rust implementation is owned by
 * Boost `language.php`.
 */

const require = createRequire(import.meta.url);
const pkg = require('../package.json') as { name: string; version: string };

/** npm package name (matches `package.json` `name`). */
export const PACKAGE_NAME = pkg.name;

/** Wrapper version (matches `package.json` `version`). */
export const PACKAGE_VERSION = pkg.version;

/** Commands provided after installation. */
export const COMMANDS = ['php'] as const;

/** Command name. */
export type Command = (typeof COMMANDS)[number];

export {
    check,
    highlight,
    loadNative,
    nativeBinaryName,
    parse,
    platformShort,
    platformTriple,
    resolveNativePath,
    tokens,
    version,
    type HighlightSpan,
    type ParseOutcome,
    type SyntaxNode,
    type TokenSpan,
} from './native.ts';

export { runPhpCli } from './cli/php-cli.ts';
