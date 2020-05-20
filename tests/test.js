//@ts-check
"use strict";

process.on("uncaughtException", (err, origin) => {
  console.error("Unknown exception at:", origin, "error:", err);
  process.exit(1);
});
process.on("unhandledRejection", (reason, promise) => {
  console.error("Unhandled Rejection at:", promise, "reason:", reason);
  process.exit(1);
});

const util = require("util");
const { default: valet, Client, createRequest } = require("..");

function logResponse(payload) {
  if (payload.result) {
    dump(payload.result);
  } else if (payload.error && payload.error.data && payload.error.data.stack) {
    console.log("Error: ");
    dump(payload.error);
    throw new Error("request error");
  } else if (payload.method === "Log") {
    console.log(payload.params.level, payload.params.message);
  } else {
    dump(payload);
  }
}

function dump(obj) {
  console.log(
    util.inspect(obj, {
      colors: true,
      showHidden: false,
      depth: null,
    })
  );
}

const requests = {
  VersionGet: createRequest("Version.Get"),
  TestDoubleTwice: createRequest("Test.DoubleTwice"),
};

async function main() {
  console.log("===========================");

  {
    const { major, minor, patch } = valet.version;
    console.log(`On valet ${major}.${minor}.${patch}`);
  }

  valet.initialize({
    dbPath: "./tmp/butler.db",
  });

  let client = new Client();
  console.log("Asking for version...");
  let res = await client.call(requests.VersionGet, {});
  console.log("Got result:");
  console.log(res);

  let numberToQuadruple = 256;

  // console.log(`Doing test request...`);
  // conn.send(
  //   JSON.stringify({
  //     jsonrpc: "2.0",
  //     id,
  //     method: "Test.DoubleTwice",
  //     params: {
  //       number: numberToQuadruple,
  //     },
  //   })
  // );
  // id++;

  // while (true) {
  //   let payload = JSON.parse(await conn.recv());
  //   if (typeof payload.id !== "undefined" && payload.method) {
  //     if (payload.method === "Test.Double") {
  //       conn.send(
  //         JSON.stringify({
  //           jsonrpc: "2.0",
  //           id: payload.id,
  //           result: {
  //             number: payload.params.number * 2,
  //           },
  //         })
  //       );
  //     } else {
  //       throw new Error(`Unknown client-side method: '${payload.method}'`);
  //     }
  //   }
  //   logResponse(payload);

  //   if (typeof payload.result !== "undefined") {
  //     break;
  //   }
  // }

  // console.log(`Test went fine!`);
}

main()
  .catch((e) => console.warn(e.stack))
  .then(() => {
    if (typeof process.versions["electron"] !== "undefined") {
      // @ts-ignore
      require("electron").app.exit(0);
    }
    process.exit(0);
  });
