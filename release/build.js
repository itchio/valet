//@ts-check
"use strict";

/**
 * @typedef OsInfo
 * @type {{
 *   libName: string,
 *   architectures: {
 *     [key: string]: {
 *       toolchain: string,
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
        toolchain: "stable-i686-pc-windows-gnu",
      },
      x86_64: {
        toolchain: "stable-x86_64-pc-windows-gnu",
      },
    },
  },
  linux: {
    libName: "libvalet.so",
    architectures: {
      x86_64: {
        toolchain: "stable-x86_64-unknown-linux-gnu",
      },
    },
  },
  darwin: {
    libName: "libvalet.dylib",
    architectures: {
      x86_64: {
        toolchain: "stable-x86_64-apple-darwin",
      },
    },
  },
};
const DEFAULT_ARCH = "x86_64";

let isVerbose = false;

/**
 * @param {string[]} args
 */
function main(args) {
  header("Gathering configuration");

  /**
   * @type {{
   *   os?: string;
   *   arch?: string;
   * }}
   */
  let opts = {};
  let positional = [];
  for (let i = 0; i < args.length; i++) {
    let arg = args[i];

    let matches = /^--(.*)$/.exec(arg);
    if (matches) {
      let k = matches[1];
      if (k == "verbose") {
        isVerbose = true;
        continue;
      }

      i++;
      let v = args[i];
      opts[k] = v;
    } else if (arg === "-v") {
      isVerbose = true;
    } else if (arg.startsWith("-")) {
      throw new Error(`Unsupported flag: ${yellow(arg)}`);
    } else {
      positional.push(arg);
    }
  }
  debug({ opts, positional });

  if (!opts.os) {
    opts.os = detectOS();
    console.log(
      `Using detected OS ${yellow(opts.os)} (use ${yellow("--os")} to override)`
    );
  }

  let osInfo = OS_INFOS[opts.os];
  debug({ osInfo });
  if (!osInfo) {
    throw new Error(`Unsupported OS ${yellow(opts.os)}`);
  }

  if (!opts.arch) {
    opts.arch = DEFAULT_ARCH;
    console.log(
      `Using default arch ${yellow(opts.arch)} (use ${yellow(
        "--arch"
      )} to override)`
    );
  }

  let archInfo = osInfo.architectures[opts.arch];
  debug({ archInfo });
  if (!archInfo) {
    throw new Error(`Unsupported arch '${opts.arch}' for os '${opts.os}'`);
  }

  header("Showing tool versions");
  $(`node --version`);
  $(`go version`);
  $(`rustup -V`);

  $(`rustup toolchain install ${archInfo.toolchain}`);
}

/**
 * @param {string} line
 */
function header(line) {
  let bar = "―".repeat(line.length + 2);

  console.log();
  console.log(blue(bar));
  console.log(blue(` ${line} `));
  console.log(blue(bar));
  console.log();
}

function debug() {
  if (!isVerbose) {
    return;
  }
  console.log.apply(console, arguments);
}

const colors = {
  blue: "\x1b[1;34;40m",
  yellow: "\x1b[1;33;40m",
  reset: "\x1b[0;0;0m",
};

/**
 * @param {string} s
 * @return {string}
 */
function yellow(s) {
  return `${colors.yellow}${s}${colors.reset}`;
}

/**
 * @param {string} s
 * @return {string}
 */
function blue(s) {
  return `${colors.blue}${s}${colors.reset}`;
}

/**
 * @param {string} cmd
 */
function $(cmd) {
  console.log(yellow(`📜 ${cmd}`));
  const cp = require("child_process");
  cp.execSync(cmd, {
    stdio: "inherit",
  });
}

/**
 * @returns {string}
 */
function detectOS() {
  switch (process.platform) {
    case "win32":
      return "windows";
    case "darwin":
      return "darwin";
    case "linux":
      return "linux";
    default:
      throw new Error(`Unsupported process.platform: ${process.platform}`);
  }
}

main(process.argv.slice(2));
