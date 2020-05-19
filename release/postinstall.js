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
  $$,
} = require("./common");
const { resolve } = require("path");
const { createWriteStream, rmdirSync, mkdirSync, existsSync } = require("fs");

/**
 * @param {string[]} args
 */
async function main(args) {
  let start = Date.now();

  let opts = {
    os: detectOS(),
    arch: process.arch === "ia32" ? "i686" : "x86_64",
    force: false,
    userSpecifiedOS: false,
    userSpecifiedArch: false,
  };

  for (let i = 0; i < args.length; i++) {
    let arg = args[i];

    if (arg === "-v" || arg === "--verbose") {
      setVerbose(true);
    } else if (arg === "--arch") {
      i++;
      let v = args[i];
      opts.arch = v;
      opts.userSpecifiedArch = true;
    } else if (arg === "--os") {
      i++;
      let v = args[i];
      opts.os = v;
      opts.userSpecifiedOS = true;
    } else if (arg === "--force") {
      opts.force = true;
    } else {
      throw new Error(`Unknown argument ${arg}`);
    }
  }

  if (existsSync("Cargo.toml") && !opts.force) {
    console.log(
      `In development (${yellow(
        `Cargo.toml`
      )} found), skipping postinstall (Use ${yellow("--force")} to force)`
    );
    return;
  }

  let { version } = require("../package.json");

  if (opts.userSpecifiedOS) {
    console.log(`Using user-specified os ${yellow(opts.os)}`);
  } else {
    debug(`Using detected os ${yellow(opts.os)}`);
  }

  if (opts.userSpecifiedArch) {
    console.log(`Using user-specified arch ${yellow(opts.arch)}`);
  } else {
    debug(`Using detected arch ${yellow(opts.arch)}`);
  }

  let platform = `${opts.arch}-${opts.os}`;
  console.log(`valet ${yellow(version)} on ${yellow(platform)}`);

  let bindingsPath = `./artifacts/${platform}/index.node`;

  if (!(opts.userSpecifiedArch || opts.userSpecifiedOS)) {
    debug(`Platform is fully autodetected, probing ${yellow(bindingsPath)}`);
    if (shouldSkipDownload({ bindingsPath, version })) {
      debug(`Nothing to do`);
      return;
    }
  }

  let tag = `v${version}`;
  let url = `https://github.com/itchio/valet/releases/download/${tag}/${platform}.zip`;

  let output = `./artifacts/tmp.zip`;
  mkdirSync(resolve(output, ".."), { recursive: true });
  let out = createWriteStream(output, { autoClose: true });

  debug(`Downloading from ${yellow(url)}`);
  await downloadToStream(url, out);

  const extract = require("extract-zip");
  await extract(output, { dir: resolve("./artifacts") });

  let bindingsSize = sizeof(bindingsPath);
  debug(
    `Bindings ${yellow(bindingsPath)} is ${yellow(formatSize(bindingsSize))}`
  );

  rmdirSync("./artifacts/tmp.zip", { recursive: true });

  let end = Date.now();
  let totalTime = `${((end - start) / 1000).toFixed(1)}s`;
  debug(`Total time: ${yellow(totalTime)}`);
}

/**
 * @param {{ bindingsPath: string; version: string; }} opts
 * @returns {boolean}
 */
function shouldSkipDownload(opts) {
  const { bindingsPath, version } = opts;

  if (!existsSync(bindingsPath)) {
    debug(`Bindings don't exist on disk yet`);
    return false;
  }

  let bindingsVersionObject;
  try {
    let evalSource = `
      (function() {
        let bindings = require('${bindingsPath}');
        console.log(JSON.stringify(bindings.version));
      })()
    `;
    evalSource = evalSource.replace(/[\n\t ]+/g, " ");
    bindingsVersionObject = JSON.parse($$(`node -e "${evalSource}"`));
  } catch (e) {
    debug(`Bindings exist on disk but can't require: ${e}`);
    return false;
  }

  let { major, minor, patch } = bindingsVersionObject;
  let bindingsVersion = `${major}.${minor}.${patch}`;
  debug(`Bindings on disk have version ${yellow(bindingsVersion)}`);

  if (bindingsVersion !== version) {
    debug(`Bindings on disk are the wrong version`);
    return false;
  }

  return true;
}

main(process.argv.slice(2)).catch((e) => {
  throw new Error(e.stack);
});
