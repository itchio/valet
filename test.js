const valet = require("./artifacts/x86_64-windows/index.node");
global.valet = valet;

console.log("===========================")
console.log("valet is: ")
console.log(valet);

for (let i = 0; i < 5; i++) {
    console.log(valet.say_hi());
}