#!/usr/bin/env node

const { execFileSync } = require("child_process");
const path = require("path");
const os = require("os");

const PLATFORMS = {
  "darwin-arm64": "gadscli-darwin-arm64",
  "darwin-x64": "gadscli-darwin-x64",
  "linux-x64": "gadscli-linux-x64-gnu",
  "linux-arm64": "gadscli-linux-arm64-gnu",
  "win32-x64": "gadscli-win32-x64-msvc",
};

const platform = os.platform();
const arch = os.arch();
const key = `${platform}-${arch}`;
const name = PLATFORMS[key];

if (!name) {
  console.error(
    `Unsupported platform: ${key}. Supported: ${Object.keys(PLATFORMS).join(", ")}`
  );
  process.exit(1);
}

const ext = platform === "win32" ? ".exe" : "";
const binPath = path.join(__dirname, "..", "bin", `${name}${ext}`);

try {
  execFileSync(binPath, process.argv.slice(2), {
    stdio: "inherit",
    env: process.env,
  });
} catch (e) {
  if (e.status !== undefined) {
    process.exit(e.status);
  }
  throw e;
}
