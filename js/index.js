"use strict";
function __export(m) {
    for (var p in m) if (!exports.hasOwnProperty(p)) exports[p] = m[p];
}
exports.__esModule = true;
__export(require("./support"));
__export(require("./client"));
__export(require("./conversation"));
function getOS() {
    switch (process.platform) {
        case "win32":
            return "windows";
        case "darwin":
            return "darwin";
        case "linux":
            return "linux";
        default:
            throw new Error("valet: unsupported process.platform '" + process.platform + "'");
    }
}
function getArch() {
    switch (process.arch) {
        case "ia32":
            return "i686";
        case "x64":
            return "x86_64";
        default:
            throw new Error("valet: unsupported process.arch '" + process.arch + "'");
    }
}
var folder = getArch() + "-" + getOS();
var bindingsPath = "../artifacts/" + folder + "/index.node";
var envKey = "VALET_BINDINGS_PATH";
var envBindingsPath = process.env[envKey];
if (envBindingsPath) {
    console.log("valet: bindings path overriden by $" + envKey + "=" + JSON.stringify(envBindingsPath));
    bindingsPath = envBindingsPath;
}
var valet = require(bindingsPath);
exports["default"] = valet;
