#!/usr/bin/env node
import { runPhpHarnessCli } from '../dist/cli.js';

const code = await runPhpHarnessCli(process.argv.slice(2));
process.exit(code ?? 0);
