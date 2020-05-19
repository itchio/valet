//@ts-check
"use strict";

const { $, $bash, header, yellow, green, blue, info } = require("./common");
const { generateTypings } = require("./generate-typings");
const {
  statSync,
  readdirSync,
  readFileSync,
  mkdirSync,
  rmdirSync,
} = require("fs");

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

  header("Gathering information");
  $(`node --version`);

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

  rmdirSync("./artifacts/tmp.zip", { recursive: true });
  const targets = readdirSync("./artifacts");
  info(`Will upload targets: ${targets.map(yellow).join(", ")}`);

  generateTypings();
  $(`npm run ts`);

  if (process.env.DRY_RUN) {
    info("Dry run, bailing out now");
    return;
  }

  header("Uploading native addons...");
  mkdirSync("./release-tools", { recursive: true });

  let toolRepo = `https://github.com/github-release/github-release`;
  let toolTag = `v0.8.1`;
  let toolUrl = `${toolRepo}/releases/download/${toolTag}/linux-amd64-github-release.bz2`;
  let ghr = `./release-tools/github-release`;
  try {
    statSync(ghr);
    info(`Using existing ${yellow(ghr)}...`);
  } catch (e) {
    info(`Downloading ${yellow(ghr)}`);
    $bash(`curl --location "${toolUrl}" | bunzip2 > ${ghr}`);
  }
  $bash(`chmod +x ${ghr}`);
  $bash(`${ghr} --version`);

  process.env.GITHUB_USER = "itchio";
  process.env.GITHUB_REPO = "valet";
  if (!process.env.GITHUB_TOKEN) {
    throw new Error(
      `${yellow(
        "$GITHUB_TOKEN"
      )} is unset, are you running this script outside of CI?`
    );
  }

  try {
    $bash(`${ghr} delete --tag "${tag}"`);
    info(`Probably replacing release`);
  } catch (e) {
    info(`Probably not replacing release`);
  }
  $bash(`${ghr} release --tag "${tag}"`);

  for (const target of targets) {
    let label = `Native addon for ${target}`;
    $bash(
      `(cd ./artifacts; zip --display-dots --recurse-paths ./tmp.zip ./${target})`
    );
    // note: github-release says it can upload from stdin, but it can't
    $bash(
      [
        `${ghr} upload --tag "${tag}" --file "./artifacts/tmp.zip" --replace`,
        `--label "${label}" --name "${target}.zip"`,
      ].join(" ")
    );
    rmdirSync("./artifacts/tmp.zip", { recursive: true });
  }
}

main();
