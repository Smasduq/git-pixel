'use strict';

// Downloads the prebuilt gitpixel binary for the current platform/arch
// from GitHub Releases and stores it under ./vendor/.
const { execFileSync } = require('child_process');
const fs = require('fs');
const https = require('https');
const path = require('path');
const zlib = require('zlib');

const REPO = 'Smasduq/git-pixel';
const PACKAGE_VERSION = require('../package.json').version;
const TAG = `v${PACKAGE_VERSION}`;

function mapTarget() {
  const p = process.platform;
  const a = process.arch;
  if (p === 'linux' && a === 'x64') return 'x86_64-unknown-linux-gnu';
  if (p === 'linux' && a === 'arm64') return 'aarch64-unknown-linux-gnu';
  if (p === 'darwin' && a === 'arm64') return 'aarch64-apple-darwin';
  if (p === 'darwin' && a === 'x64') return 'x86_64-apple-darwin';
  if (p === 'win32' && a === 'x64') return 'x86_64-pc-windows-msvc';
  throw new Error(`Unsupported platform/arch: ${p}/${a}`);
}

const TARGET = mapTarget();
const archive = TARGET.endsWith('.exe') ? null : TARGET === 'x86_64-pc-windows-msvc' ? `${TARGET}.zip` : `${TARGET}.tar.gz`;
const FILE_NAME = archive || `${TARGET}.zip`;
const URL = `https://github.com/${REPO}/releases/download/${TAG}/${FILE_NAME}`;

const VENDOR_DIR = path.join(__dirname, '..', 'vendor');
const BIN_DIR = path.join(VENDOR_DIR, TARGET);
const BIN_NAME = process.platform === 'win32' ? 'gitpixel.exe' : 'gitpixel';
const FINAL_BIN = path.join(VENDOR_DIR, BIN_NAME);

function download(url) {
  return new Promise((resolve, reject) => {
    https
      .get(url, { headers: { 'User-Agent': 'gitpixel-npm' } }, (res) => {
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          download(res.headers.location).then(resolve, reject);
          res.resume();
          return;
        }
        if (res.statusCode !== 200) {
          reject(new Error(`HTTP ${res.statusCode} for ${url}`));
          return;
        }
        const chunks = [];
        res.on('data', (d) => chunks.push(d));
        res.on('end', () => resolve(Buffer.concat(chunks)));
      })
      .on('error', reject);
  });
}

function extractArchive(buf) {
  fs.mkdirSync(BIN_DIR, { recursive: true });
  if (FILE_NAME.endsWith('.zip')) {
    // Use unzip via a child process.
    const zipPath = path.join(VENDOR_DIR, FILE_NAME);
    fs.writeFileSync(zipPath, buf);
    execFileSync('unzip', ['-o', zipPath, '-d', BIN_DIR], { stdio: 'inherit' });
    fs.unlinkSync(zipPath);
  } else {
    const tarPath = path.join(VENDOR_DIR, FILE_NAME);
    fs.writeFileSync(tarPath, buf);
    execFileSync('tar', ['-xzf', tarPath, '-C', BIN_DIR], { stdio: 'inherit' });
    fs.unlinkSync(tarPath);
  }
}

async function main() {
  // If already installed and populated, skip the download.
  if (fs.existsSync(FINAL_BIN)) {
    console.log('gitpixel binary already installed, skipping download.');
    return;
  }

  fs.mkdirSync(VENDOR_DIR, { recursive: true });
  console.log(`Downloading gitpixel ${PACKAGE_VERSION} for ${TARGET}...`);
  const buf = await download(URL);
  extractArchive(buf);

  const src = path.join(BIN_DIR, BIN_NAME);
  if (!fs.existsSync(src)) {
    throw new Error(`expected binary not found in archive: ${BIN_NAME}`);
  }
  fs.renameSync(src, FINAL_BIN);
  if (process.platform !== 'win32') {
    fs.chmodSync(FINAL_BIN, 0o755);
  }
  console.log(`Installed gitpixel to ${FINAL_BIN}`);
}

main().catch((err) => {
  console.error('Failed to install gitpixel binary:', err.message);
  if (err.message.includes('HTTP') || err.message.includes('ENOTFOUND')) {
    console.error(
      `Make sure ${URL} exists — release assets are built by the GitHub Actions ` +
        'release workflow and tagged with the package version.'
    );
  }
  process.exit(1);
});
