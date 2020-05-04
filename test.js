const util = require("util");

// const valet = require("./artifacts/x86_64-windows/index.node");
const valet = require("./artifacts/x86_64-linux/index.node");
global.valet = valet;

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

  let s = valet.new_server();
  let id = 1;

  console.log(`Looking for game...`);
  s.send(
    JSON.stringify({
      jsonrpc: "2.0",
      id,
      method: "Fetch.Caves",
      params: {
        limit: 1,
        filters: {
          gameId: 447005,
        },
      },
    })
  );
  id++;

  let res = JSON.parse(s.recv());
  logResponse(res);

  let caveId = res.result.items[0].id;
  console.log(`Launching cave ${caveId} in 2 seconds...`);

  await new Promise((rs, rj) => setTimeout(rs, 2000));

  s.send(
    JSON.stringify({
      jsonrpc: "2.0",
      method: "Launch",
      id,
      params: {
        caveId,
        prereqsDir: "/home/amos/.config/itch/prereqs",
      },
    })
  );
  id++;

  console.log("Now dumping responses...");
  while (true) {
    let res = JSON.parse(s.recv());
    logResponse(res);
  }

  // setTimeout(() => {
  //   console.log("Bye now!");
  // }, 1000);
}
