// const valet = require("./artifacts/x86_64-windows/index.node");
const valet = require("./artifacts/x86_64-linux/index.node");
global.valet = valet;

console.log("===========================")
console.log("valet is: ")
console.log(valet);

for (let i = 0; i < 5; i++) {
    valet.set(10 * i);
    console.log(valet.get());
}
