//@ts-check
"use strict";

const {
  $,
  $bash,
  header,
  yellow,
  green,
  blue,
  info,
  detectOS,
} = require("./common");
const { readdirSync, readFileSync, mkdirSync } = require("fs");

function main() {
  try {
    let manifest = JSON.parse(
      readFileSync("./package.json", { encoding: "utf8" })
    );
    if (manifest.name !== "valet") {
      throw new Error("Unexpected manifest name");
    }
  } catch (e) {
    throw new Error(
      `Script must be invoked as 'node release/release.js', from the root repository folder.\n` +
        `Was invoked from ${yellow(process.cwd())} instead.`
    );
  }

  header("Showing tool versions");
  $(`node --version`);
  $(`node --version`);

  header("Gathering information");

  let tag = process.env.CI_COMMIT_TAG;
  if (!tag) {
    throw new Error(`$CI_COMMIT_TAG not set, bailing out`);
  }

  let matches = /v([0-9]+)\.([0-9]+)\.([0-9]+)/.exec(tag);
  if (!matches) {
    throw new Error(
      `Could not parse version ${yellow(tag)} - is it missing the 'v' prefix?`
    );
  }
  let [, major, minor, patch] = matches;
  info(`Releasing version ${yellow(`${major}.${minor}.${patch}`)}`);

  const targets = readdirSync("./artifacts");
  info(`Will upload targets: ${targets.map(yellow).join(", ")}`);

  header("Downloading tool");
  mkdirSync("./release-tools", { recursive: true });

  let toolRepo = `https://github.com/github-release/github-release`;
  let toolTag = `v0.8.1`;
  let toolUrl = `${toolRepo}/releases/download/${toolTag}/linux-amd64-github-release.bz2`;
  $bash(`curl -L "${toolUrl}" | bunzip2 > ./release-tools/github-release`);
  $bash(`chmod +x ./release-tools/github-release`);
  $bash(`release-tools/github-release -v`);
}

main();
