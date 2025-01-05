#!/usr/bin/env node

const { run } = require('../src/cli');

run().catch(error => {
    console.error('Error:', error);
    process.exit(1);
}); 