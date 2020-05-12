//@ts-check
"use strict";

let verbose = false;

/**
 * @param {number} b The size in bytes
 * @returns {string} A human-readable size
 */
function fileSize(b) {
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
 * @returns {string} stdout
 */
function $$(cmd) {
  console.log(yellow(`📜 ${cmd}`));
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

module.exports = {
  $,
  $$,
  $bash,
  fileSize,
  info,
  header,
  debug,
  yellow,
  blue,
  green,
  isVerbose,
  setVerbose,
  detectOS,
};
