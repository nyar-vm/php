import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import type { Cli, ParsedOptions } from '@vmz/commander';
import { createCli } from '@vmz/commander';

import { check, highlight, parse, tokens, version } from '../native.ts';

const localesRoot = path.join(path.dirname(fileURLToPath(import.meta.url)), '../../locales');

function positional(options: ParsedOptions): string | undefined {
    const first = options._[0];
    return typeof first === 'string' && first.length > 0 ? first : undefined;
}

function sourcePath(options: ParsedOptions): string | undefined {
    const value = positional(options);
    return value ? path.resolve(value) : undefined;
}

function buildPhpCli(): Cli {
    const cli = createCli('php').intro('cli.intro');
    try {
        cli.locales(localesRoot);
    } catch {
        /* locales optional in dev */
    }

    cli.command('version', 'cli.cmd.version').action(() => {
        console.log(version());
        return 0;
    });

    cli.command('parse', 'cli.cmd.parse')
        .option('-j, --json', 'cli.opt.json')
        .action((options: ParsedOptions) => {
            const file = sourcePath(options);
            if (!file) {
                console.error('missing <file>');
                return 2;
            }
            const outcome = parse(readFileSync(file, 'utf8'));
            if (options.json) {
                console.log(JSON.stringify(outcome, null, 2));
            } else if (outcome.ok) {
                console.log(`parsed ${file} (${outcome.tokenCount} token(s), ${outcome.diagnostics.length} diagnostic(s))`);
            } else {
                console.error(outcome.error ?? 'parse failed');
            }
            return outcome.ok ? 0 : 1;
        });

    cli.command('tokens', 'cli.cmd.tokens')
        .option('-j, --json', 'cli.opt.json')
        .action((options: ParsedOptions) => {
            const file = sourcePath(options);
            if (!file) {
                console.error('missing <file>');
                return 2;
            }
            const list = tokens(readFileSync(file, 'utf8'));
            if (options.json) {
                console.log(JSON.stringify(list, null, 2));
            } else {
                for (const token of list) console.log(`${token.kind}\t${token.start}\t${token.end}`);
            }
            return 0;
        });

    cli.command('check', 'cli.cmd.check').action((options: ParsedOptions) => {
        const files = options._.filter((value): value is string => typeof value === 'string');
        if (files.length === 0) {
            console.error('missing <file...>');
            return 2;
        }
        let failed = 0;
        for (const file of files) {
            const resolved = path.resolve(file);
            const diagnostics = check(readFileSync(resolved, 'utf8'));
            if (diagnostics.length === 0) {
                console.log(`${file}: ok`);
                continue;
            }
            failed++;
            console.error(`${file}: ${diagnostics.length} diagnostic(s)`);
            for (const diagnostic of diagnostics) console.error(`  ${diagnostic}`);
        }
        return failed === 0 ? 0 : 1;
    });

    cli.command('highlight', 'cli.cmd.highlight')
        .option('-j, --json', 'cli.opt.json')
        .action((options: ParsedOptions) => {
            const file = sourcePath(options);
            if (!file) {
                console.error('missing <file>');
                return 2;
            }
            const spans = highlight(readFileSync(file, 'utf8'));
            if (options.json) {
                console.log(JSON.stringify(spans, null, 2));
            } else {
                for (const span of spans) console.log(`${span.kind}\t${span.start}\t${span.end}`);
            }
            return 0;
        });

    return cli;
}

/** Parse argv and run the PHP CLI (@vmz/commander; capabilities via php-napi). */
export async function runPhpCli(argv: string[] = process.argv.slice(2)): Promise<number> {
    return buildPhpCli().parse(argv);
}
