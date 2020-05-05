//@ts-check
"use strict";

/**
 * @returns {string}
 */
function getOS() {
  switch (process.platform) {
    case "win32":
      return "windows";
    case "darwin":
      return "darwin";
    case "linux":
      return "linux";
    default:
      throw new Error(
        `valet: unsupported process.platform '${process.platform}'`
      );
  }
}

/**
 * @returns {string}
 */
function getArch() {
  switch (process.arch) {
    case "ia32":
      return "i686";
    case "x64":
      return "x86_64";
    default:
      throw new Error(`valet: unsupported process.arch '${process.arch}'`);
  }
}

let folder = `${getArch()}-${getOS()}`;
module.exports = require(`./artifacts/${folder}/index.node`);
