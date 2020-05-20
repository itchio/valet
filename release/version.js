//@ts-check
"use strict";

const { yellow, info, $ } = require("./common");
const { readFileSync, writeFileSync } = require("fs");

/**
 * @param {string[]} args
 */
async function main(args) {
  const { version } = require("../package.json");
  info(`Bumped to version ${yellow(version)}`);

  info("Editing Cargo.toml...");
  let contents = readFileSync("Cargo.toml", { encoding: "utf8" });
  let lines = contents.split("\n");
  let foundVersion = false;
  for (let i = 0; i < lines.length; i++) {
    let line = lines[i];
    if (/version = ".*"/.test(line)) {
      lines[i] = `version = ${JSON.stringify(version)}`;
      foundVersion = true;
      break;
    }
  }
  if (!foundVersion) {
    throw new Error("Could not find version line in Cargo.toml!");
  }

  contents = lines.join("\n");
  writeFileSync("Cargo.toml", contents, { encoding: "utf8" });

  info("Cargo checking...");
  $(`cargo check`);

  info("Generating JSON-RPC typings...");
  $(`npm run generate-typings`);

  info("Compiling TypeScript code...");
  $(`npm run ts`);

  info("Adding files to git");
  $(`git add -A Cargo.toml Cargo.lock generated/version.rs ts/messages.ts`);
}

main(process.argv.slice(2)).catch((e) => {
  throw new Error(e.stack);
});
