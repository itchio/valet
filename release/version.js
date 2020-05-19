//@ts-check
"use strict";

const { info, $ } = require("./common");
const { generateTypings } = require("./generate-typings");

/**
 * @param {string[]} args
 */
async function main(args) {
  info("Generating JSON-RPC typings...");
  generateTypings();
  info("Compiling TypeScript code...");
  $(`npm run ts`);
}

main(process.argv.slice(2)).catch((e) => {
  throw new Error(e.stack);
});
