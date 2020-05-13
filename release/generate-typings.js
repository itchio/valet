//@ts-check
"use strict";

const { $bash, info } = require("./common");

function generateTypings() {
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

module.exports = { generateTypings };
