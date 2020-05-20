//@ts-check
"use strict";

const { $bash, info, header, $ } = require("./common");

/**
 * @param {string[]} args
 */
async function main(args) {
  info("Showing tool versions");
  $(`node --version`);
  $(`go version`);

  info("Compiling and running generous...");

  let oldCwd = process.cwd();
  try {
    process.chdir("./libbutler");
    $bash(`go build github.com/itchio/butler/butlerd/generous`);
    $bash(`./generous ts --support-path "./support" ../ts/messages.ts`);
  } catch (e) {
    throw e;
  } finally {
    process.chdir(oldCwd);
  }

  info(`Typescript typings generated!`);
}

main(process.argv.slice(2)).catch((e) => {
  throw new Error(e.stack);
});
