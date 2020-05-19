//@ts-check
"use strict";

const { $ } = require("./common");
const { generateTypings } = require("./generate-typings");

/**
 * @param {string[]} args
 */
async function main(args) {
  generateTypings();
  $(`npm run ts`);
}

main(process.argv.slice(2)).catch((e) => {
  throw new Error(e.stack);
});
