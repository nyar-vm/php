#!/usr/bin/env node
import { runPhpCli } from '../dist/cli/php-cli.js';

const code = await runPhpCli(process.argv.slice(2));
process.exit(code ?? 0);
