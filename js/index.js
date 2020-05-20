"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !exports.hasOwnProperty(p)) __createBinding(exports, m, p);
};
exports.__esModule = true;
__exportStar(require("./support"), exports);
__exportStar(require("./client"), exports);
__exportStar(require("./conversation"), exports);
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
