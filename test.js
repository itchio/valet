//@ts-check
"use strict";

const util = require("util");
const valet = require(".");

main();

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

async function main() {
  console.log("===========================");

  let s = valet.newServer({
    dbPath: "./tmp/butler.db",
  });
  let id = 1;

  let numberToQuadruple = 256;

  console.log(`Doing test request...`);
  s.send(
    JSON.stringify({
      jsonrpc: "2.0",
      id,
      method: "Test.DoubleTwice",
      params: {
        number: numberToQuadruple,
      },
    })
  );
  id++;

  while (true) {
    let payload = JSON.parse(s.recv());
    if (typeof payload.id !== "undefined" && payload.method) {
      if (payload.method === "Test.Double") {
        s.send(
          JSON.stringify({
            jsonrpc: "2.0",
            id: payload.id,
            result: {
              number: payload.params.number * 2,
            },
          })
        );
      } else {
        throw new Error(`Unknown client-side method: '${payload.method}'`);
      }
    }
    logResponse(payload);

    if (typeof payload.result !== "undefined") {
      break;
    }
  }

  console.log(`Test went fine!`);
}
