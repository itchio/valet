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

const { default: valet, Client, createRequest } = require("..");

const requests = {
  VersionGet: createRequest("Version.Get"),
  TestDouble: createRequest("Test.Double"),
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
  let version = await client.call(requests.VersionGet, {});
  console.log(version);

  console.log("Quadrupling number...");
  let numberToQuadruple = 256;
  let doubleTwiceRes = await client.call(
    requests.TestDoubleTwice,
    {
      number: numberToQuadruple,
    },
    (convo) => {
      convo.onRequest(requests.TestDouble, async ({ number }) => {
        return { number: number * 2 };
      });
    }
  );
  console.log(`Result: ${numberToQuadruple} => ${doubleTwiceRes.number}`);

  console.log(`Test went fine!`);
}

main()
  .catch((e) => console.warn(e.stack))
  .then(() => {
    if (typeof global.gc !== "undefined") {
      console.log("Triggering a few GC rounds..");
      for (let i = 0; i < 5; i++) {
        global.gc();
      }
    } else {
      console.log("GC not exposed :(");
    }
    return new Promise((resolve, reject) => {
      console.log("Waiting 500ms...");
      setTimeout(resolve, 500);
    });
  })
  .then(() => {
    console.log("Exiting!");
    if (typeof process.versions["electron"] !== "undefined") {
      // @ts-ignore
      require("electron").app.exit(0);
    }
    process.exit(0);
  });
