#!/usr/bin/env node
'use strict';

const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const VENDOR_DIR = path.join(__dirname, '..', 'vendor');
const binName = process.platform === 'win32' ? 'gitpixel.exe' : 'gitpixel';
const binaryPath = path.join(VENDOR_DIR, binName);

if (!fs.existsSync(binaryPath)) {
  console.error(
    'gitpixel binary not found at ' +
      binaryPath +
      '. Try `npm rebuild gitpixel` or check the install log for a download failure.'
  );
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
});

if (result.error) {
  console.error('Failed to run gitpixel:', result.error.message);
  process.exit(1);
}

process.exit(result.status === null ? 1 : result.status);
