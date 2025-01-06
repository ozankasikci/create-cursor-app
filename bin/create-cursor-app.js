#!/usr/bin/env node

// Make sure the script runs in ES modules mode
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Run the CLI
import('../src/cli/index.js').then(cli => cli.run()).catch(error => {
    console.error('Error:', error);
    process.exit(1);
}); 