//@ts-check
"use strict";

const {
  yellow,
  detectOS,
  formatSize,
  sizeof,
  downloadToStream,
  debug,
  setVerbose,
} = require("./common");
const { createWriteStream } = require("fs");

/**
 * @param {string[]} args
 */
async function main(args) {
  let start = Date.now();

  for (const arg of args) {
    if (arg === "-v" || arg === "--verbose") {
      setVerbose(true);
    } else {
      throw new Error(`Unknown argument ${arg}`);
    }
  }

  let { version } = require("../package.json");

  let os = detectOS();
  let arch = "x86_64";
  if (process.arch === "ia32") {
    arch = "i686";
  }
  let platform = `${arch}-${os}`;
  console.log(`valet ${yellow(version)} on ${yellow(platform)}`);

  let tag = `v${version}`;
  let url = `https://github.com/itchio/valet/releases/download/${tag}/${platform}.zip`;

  let output = `./artifacts/tmp.zip`;
  let out = createWriteStream(output, { autoClose: true });

  debug(`Downloading from ${yellow(url)}`);
  await downloadToStream(url, out);

  const extract = require("extract-zip");
  const { resolve } = require("path");
  await extract(output, { dir: resolve("./artifacts") });

  let bindingsPath = `artifacts/${platform}/index.node`;
  let bindingsSize = sizeof(bindingsPath);
  debug(
    `Bindings ${yellow(bindingsPath)} is ${yellow(formatSize(bindingsSize))}`
  );

  let end = Date.now();
  let totalTime = `${((end - start) / 1000).toFixed(1)}s`;
  debug(`Total time: ${yellow(totalTime)}`);
}

main(process.argv.slice(2)).catch((e) => {
  throw new Error(e.stack);
});
