export * from "./client";

export interface Config {
  /** path to DB file */
  dbPath: string;

  /** path to broth components */
  componentsDir: string;

  /** HTTP user-agent to use */
  userAgent?: string;
  /** itch.io API address, defaults to "https://itch.io" */
  address?: string;

  /** current app version */
  appVersion?: string;
  /** is the app canary channel or not? */
  isCanary?: boolean;
}

export interface Conn {
  send(payload: string): void;
  recv(): Promise<string>;
  close(): void;
}

export interface VersionObject {
  major: number;
  minor: number;
  patch: number;
}

export interface ValetStatic {
  version: VersionObject;
  initialize(opts: Config): void;
  newConn(): Conn;
  goPanic(): void;
  rustPanic(): void;
  selfUpdateCheck(): Promise<string>;
}

function getOS(): string {
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

function getArch(): string {
  switch (process.arch) {
    case "ia32":
      return "i686";
    case "x64":
      return "x86_64";
    default:
      throw new Error(`valet: unsupported process.arch '${process.arch}'`);
  }
}

(function () {
  //@ts-ignore
  if (typeof process !== "undefined" && process.type === "renderer") {
    throw new Error("Requiring valet from the renderer process is not supported right now.");
  }
})();

let platform = `${getOS()}-${getArch()}`;
let basePath = `./artifacts`;
{
  let envKey = "VALET_BINDINGS_BASE";
  let envBindingsBase = process.env[envKey];
  if (envBindingsBase) {
    console.log(
      `valet: bindings base overriden by $${envKey}=${JSON.stringify(
        envBindingsBase
      )}`
    );
    basePath = envBindingsBase;
  }
}

let bindingsPath = `${basePath}/${platform}/index.node`;
{
  let envKey = "VALET_BINDINGS_PATH";
  let envBindingsPath = process.env[envKey];
  if (envBindingsPath) {
    console.log(
      `valet: bindings path overriden by $${envKey}=${JSON.stringify(
        envBindingsPath
      )}`
    );
    bindingsPath = envBindingsPath;
  }
}
let valet: ValetStatic = require(bindingsPath);
export default valet;
