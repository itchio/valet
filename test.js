const valet = require("./artifacts/x86_64-windows/index.node");
global.valet = valet;

console.log("===========================")
console.log("valet is: ")
console.log(valet);

console.log("=====================");
valet.say_hi();
console.log("=====================");
valet.say_hi("ha");
console.log("=====================");
valet.say_hi(32);
console.log("=====================");
