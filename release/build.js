//@ts-check
"use strict";

const {
  $,
  $$,
  chalk,
  debug,
  header,
  info,
  sizeof,
  formatSize,
  formatPercent,
  detectOS,
  setVerbose,
} = require("@itchio/bob");
const { mkdirSync, readFileSync, writeFileSync } = require("fs");

/**
 * @typedef OsInfo
 * @type {{
 *   libName: string,
 *   architectures: {
 *     [key: string]: {
 *       triplet: string,
 *       prependPath?: string,
 *     }
 *   }
 * }}
 */

/**
 * @type {{[name: string]: OsInfo}}
 */
const OS_INFOS = {
  windows: {
    libName: "valet.dll",
    architectures: {
      i686: {
        triplet: "i686-pc-windows-gnu",
        prependPath: "/mingw32/bin",
      },
      x86_64: {
        triplet: "x86_64-pc-windows-gnu",
        prependPath: "/mingw64/bin",
      },
    },
  },
  linux: {
    libName: "libvalet.so",
    architectures: {
      x86_64: {
        triplet: "x86_64-unknown-linux-gnu",
      },
    },
  },
  darwin: {
    libName: "libvalet.dylib",
    architectures: {
      x86_64: {
        triplet: "x86_64-apple-darwin",
      },
    },
  },
};
const DEFAULT_ARCH = "x86_64";

/**
 * @param {string[]} args
 */
function main(args) {
  header("Gathering configuration");

  /**
   * @type {{
   *   os?: "linux" | "windows" | "darwin";
   *   arch?: string;
   *   test: boolean;
   *   strip: boolean;
   *   "test-runtime"?: string;
   *   rust?: string;
   * }}
   */
  let opts = {
    test: false,
    strip: true,
  };
  if (process.env.CI) {
    info(`In CI, enabling testing`);
    opts.test = true;
  }

  let positional = [];
  for (let i = 0; i < args.length; i++) {
    let arg = args[i];

    let matches = /^--(.*)$/.exec(arg);
    if (matches) {
      let k = matches[1];
      if (k == "verbose") {
        setVerbose(true);
        continue;
      }

      if (k === "os" || k === "arch" || k === "test-runtime" || k === "rust") {
        i++;
        let v = args[i];
        if (k === "os") {
          if (v === "linux" || v === "windows" || v === "darwin") {
            opts[k] = v;
          } else {
            throw new Error(`Unsupported OS ${chalk.yellow(v)}`);
          }
        } else {
          opts[k] = v;
        }
      } else if (k == "test") {
        opts.test = true;
      } else if (k == "strip") {
        opts.strip = true;
      } else if (k == "no-strip") {
        opts.strip = false;
      } else {
        throw new Error(`Unknown long option: ${chalk.yellow("--" + k)}`);
      }
    } else if (arg === "-v") {
      setVerbose(true);
    } else if (arg.startsWith("-")) {
      throw new Error(`Unsupported flag: ${chalk.yellow(arg)}`);
    } else {
      positional.push(arg);
    }
  }
  debug({ opts, positional });

  if (!opts.os) {
    opts.os = detectOS();
    console.log(
      `Using detected OS ${chalk.yellow(opts.os)} (use ${chalk.yellow(
        "--os"
      )} to override)`
    );
  } else {
    console.log(`Using specified OS ${chalk.yellow(opts.os)}`);
  }

  let osInfo = OS_INFOS[opts.os];
  debug({ osInfo });
  if (!osInfo) {
    throw new Error(`Unsupported OS ${chalk.yellow(opts.os)}`);
  }

  if (!opts.arch) {
    opts.arch = DEFAULT_ARCH;
    console.log(
      `Using default arch ${chalk.yellow(opts.arch)} (use ${chalk.yellow(
        "--arch"
      )} to override)`
    );
  } else {
    console.log(`Using specified arch ${chalk.yellow(opts.arch)}`);
  }

  let testRuntime = opts["test-runtime"] || "electron";
  if (["node", "electron"].indexOf(testRuntime) === -1) {
    throw new Error(`Unrecognized test runtime ${chalk.yellow(testRuntime)}`);
  }
  console.log(`Will use test runtime ${chalk.yellow(testRuntime)}`);

  let archInfo = osInfo.architectures[opts.arch];
  debug({ archInfo });
  if (!archInfo) {
    throw new Error(`Unsupported arch '${opts.arch}' for os '${opts.os}'`);
  }

  if (archInfo.prependPath) {
    if (opts.os === "windows") {
      let prependPath = $$(`cygpath -w ${archInfo.prependPath}`).trim();
      console.log(
        `Prepending ${chalk.yellow(archInfo.prependPath)} (aka ${chalk.yellow(
          prependPath
        )}) to $PATH`
      );
      process.env.PATH = `${prependPath};${process.env.PATH}`;
    } else {
      console.log(`Prepending ${chalk.yellow(archInfo.prependPath)} to $PATH`);
      process.env.PATH = `${archInfo.prependPath}:${process.env.PATH}`;
    }
  }

  header("Showing tool versions");
  $(`node --version`);
  $(`go version`);
  $(`rustup -V`);

  let channel = opts.rust || "stable";
  let toolchain = `${channel}-${archInfo.triplet}`;

  $(`rustup toolchain install ${toolchain}`);

  header("Compiling native module");
  $(`cargo +${toolchain} build --release`);

  header("Gathering stats");
  let outPath = `./target/release/${osInfo.libName}`;
  info(`Artifact is ${chalk.yellow(formatSize(sizeof(outPath)))}`);
  let artifactDir = `./artifacts/${opts.os}-${opts.arch}`;
  mkdirSync(artifactDir, { recursive: true });
  let artifactPath = `${artifactDir}/index.node`;
  let artifactContents = readFileSync(outPath, { encoding: null });
  writeFileSync(artifactPath, artifactContents, { encoding: null });
  info(`Copied artifact to "${artifactPath}"`);
  switch (opts.os) {
    case "linux":
      $(`file "${artifactPath}"`);
      $(`ldd "${artifactPath}"`);
      showGlibcVersion(artifactPath);
      break;
    case "windows":
      break;
    case "darwin":
      $(`file "${artifactPath}"`);
      $(`otool -L "${artifactPath}"`);
      break;
  }

  if (opts.test) {
    process.env.RUST_BACKTRACE = "1";

    header("Testing generated bindings");
    info("Compiling TypeScript support code...");
    $(`npm run ts`);

    process.env.VALET_BINDINGS_PATH = `${artifactPath}`;

    if (testRuntime === "electron") {
      mkdirSync("test-rig", { recursive: true });
      process.chdir("test-rig");
      try {
        $(`npm init -y`);
        let old_npm_config_arch = process.env.npm_config_arch;
        if (opts.arch === "i686") {
          process.env.npm_config_arch = `ia32`;
        } else if (opts.arch === "x86_64") {
          process.env.npm_config_arch = `x64`;
        } else {
          throw new Error(
            `Unsupported architecture: ${chalk.yellow(opts.arch)}`
          );
        }
        console.log(
          `Set npm_config_arch to ${chalk.yellow(process.env.npm_config_arch)}`
        );
        $(`npm i --no-save --no-audit electron`);
        process.env.npm_config_arch = old_npm_config_arch;
        $(
          `"node_modules/.bin/electron" --js-flags=--expose-gc --no-sandbox ../tests/test.js`
        );
      } catch (e) {
        throw e;
      } finally {
        process.chdir("..");
      }
    } else if (testRuntime === "node") {
      $(`npm t`);
    } else {
      throw new Error(`Unknown test runtime: '${testRuntime}'`);
    }
  } else {
    info(`Skipping testing (enable with ${chalk.yellow("--test")})`);
  }

  if (opts.strip) {
    info(`Stripping debug symbols (disable with --no-strip)`);

    let before = sizeof(artifactPath);

    // note: Linux & Windows (mingw64) support '--strip-debug' but macOS has big
    // BSD energy and only supports '-S'
    $(`strip -S ${artifactPath}`);

    let after = sizeof(artifactPath);

    console.log(
      `Before: ${chalk.yellow(formatSize(before))} ` +
        `After: ${chalk.yellow(formatSize(after))} ` +
        `(${chalk.yellow(formatPercent((before - after) / before))} reduction)`
    );
  }

  info(`All done!`);
}

/**
 * @param {string} path ELF executable to show GLIBC version for
 */
function showGlibcVersion(path) {
  let glibcVersions = $$(`bash -c 'strings ${path} | grep ^GLIBC_'`)
    .split("\n")
    .map((x) => x.replace(/^GLIBC_/, "").trim())
    .filter((x) => x != "")
    .map((x) => {
      let tokens = x.split(".").map((x) => parseInt(x, 10));
      while (tokens.length < 3) {
        tokens.push(0);
      }
      return tokens;
    });
  glibcVersions.sort((a, b) => {
    let major = a[0] - b[0];
    if (major != 0) {
      return major;
    }
    let minor = a[1] - b[1];
    if (minor != 0) {
      return minor;
    }
    return a[2] - b[2];
  });
  let minGlibcVersion = glibcVersions[glibcVersions.length - 1];
  info(`Minimum GLIBC version: ${minGlibcVersion.join(".")}`);
}

main(process.argv.slice(2));
