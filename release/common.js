//@ts-check
"use strict";

const fs = require("fs");
const https = require("https");

let verbose = false;

/**
 * @param {number} b An amount of bytes
 * @returns {string} A human-readable size
 */
function formatSize(b) {
  let KiB = 1024;
  let MiB = 1024 * KiB;

  if (b > MiB) {
    return `${(b / MiB).toFixed(2)} MiB`;
  } else if (b > KiB) {
    return `${(b / KiB).toFixed(0)} KiB`;
  } else {
    return `${b} B`;
  }
}

/**
 * @param {number} x A number in the [0, 1] range
 * @returns {string} That number formatted as a percentage
 */
function formatPercent(x) {
  return `${(x * 100).toFixed(2)}%`;
}

/**
 * Returns the size of a file in bytes
 * @param {string} path The path of the file
 * @returns {number} The size of `path` in bytes
 */
function sizeof(path) {
  const { statSync } = require("fs");
  const stats = statSync(path);
  return stats.size;
}

/**
 * @param {string} line
 */
function info(line) {
  console.log(blue(`💡 ${line}`));
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
  if (!verbose) {
    return;
  }
  // @ts-ignore
  console.log.apply(console, arguments);
}

const colors = {
  green: "\x1b[1;32;40m",
  yellow: "\x1b[1;33;40m",
  blue: "\x1b[1;34;40m",
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
 * @param {string} s
 * @return {string}
 */
function green(s) {
  return `${colors.green}${s}${colors.reset}`;
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
 * @param {string} cmd
 */
function $bash(cmd) {
  console.log(yellow(`📝 ${cmd}`));
  const cp = require("child_process");
  if (detectOS() === "windows") {
    cp.execSync("bash", {
      stdio: ["pipe", "inherit", "inherit"],
      input: cmd,
    });
  } else {
    cp.execSync(cmd, {
      stdio: "inherit",
    });
  }
}

/**
 * @param {string} cmd
 * @param {{silent?: bool}} opts
 * @returns {string} stdout
 */
function $$(cmd, opts) {
  if (!opts.silent) {
    console.log(yellow(`📜 ${cmd}`));
  }
  const cp = require("child_process");
  return cp.execSync(cmd, {
    stdio: ["inherit", "pipe", "inherit"],
    encoding: "utf8",
  });
}

/**
 * @returns {boolean}
 */
function isVerbose() {
  return verbose;
}

/**
 * @param {boolean} v
 */
function setVerbose(v) {
  verbose = v;
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

/**
 * @param {string} url
 * @param {fs.WriteStream} out
 * @returns {Promise<void>}
 */
async function downloadToStream(url, out) {
  let res = await new Promise((resolve, reject) => {
    let req = https.request(
      url,
      {
        method: "GET",
      },
      (res) => {
        resolve(res);
      }
    );

    req.on("error", (e) => {
      console.log(`Got error: ${e.stack}`);
      reject(e);
    });
    req.end();
  });

  let redirectURL = res.headers["location"];
  if (redirectURL) {
    let url = new URL(redirectURL);
    debug(`Redirected to ${yellow(url.hostname)}`);
    res.destroy();
    return await downloadToStream(redirectURL, out);
  }

  let contentLength = res.headers["content-length"] || "";
  let state = {
    doneSize: 0,
    totalSize: parseInt(contentLength, 10),
    currentDots: 0,
    totalDots: 100,
    prefix: ``,
  };

  let start = Date.now();

  let theme = {
    chunks: [" ", "▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"],
    start: "▐",
    end: "▌",
    filler: " ",
  };

  const showProgress = () => {
    let suffix = `${formatSize(state.doneSize)} / ${formatSize(
      state.totalSize
    )}`;
    process.stdout.write(`\r${theme.start}`);

    let units = state.currentDots;
    let remainWidth = Math.ceil(state.totalDots / 8);
    while (units > 0) {
      let chunk = units % 8;
      if (units >= 8) {
        chunk = 8;
      }
      let char = theme.chunks[chunk];
      process.stdout.write(char);
      units -= chunk;
      remainWidth--;
    }
    while (remainWidth > 0) {
      process.stdout.write(theme.filler);
      remainWidth--;
    }
    process.stdout.write(`${theme.end} ${suffix}`);
  };
  showProgress();

  /**
   * @param {Buffer} data
   */
  let onData = (data) => {
    state.doneSize += data.byteLength;
    let currentDots = Math.floor(
      (state.doneSize / state.totalSize) * state.totalDots
    );
    while (state.currentDots != currentDots) {
      state.currentDots = currentDots;
      showProgress();
    }
    out.write(data);
  };
  res.on("data", onData);
  res.on("close", () => {
    out.close();
  });

  await new Promise((resolve, reject) => {
    out.on("close", () => {
      resolve();
    });
    out.on("error", (e) => {
      console.warn(`I/O error: ${e.stack}`);
      reject(e);
    });
    res.on("aborted", () => {
      console.warn("Request aborted!");
      reject(new Error("Request aborted"));
    });
  });

  process.stdout.write(
    "\r                                                       \r"
  );
  let end = Date.now();

  let elapsedMS = end - start;
  let elapsedSeconds = elapsedMS / 1000;
  let bytesPerSec = state.totalSize / elapsedSeconds;

  let doneIn = `${elapsedSeconds.toFixed(1)}s`;
  let avgSpeed = `${formatSize(bytesPerSec)}/s`;
  debug(
    `Downloaded ${yellow(formatSize(state.totalSize))} in ${yellow(
      doneIn
    )}, average DL speed ${yellow(avgSpeed)}`
  );
}

module.exports = {
  $,
  $$,
  $bash,
  formatSize,
  formatPercent,
  sizeof,
  info,
  header,
  debug,
  yellow,
  blue,
  green,
  isVerbose,
  setVerbose,
  detectOS,
  downloadToStream,
};
