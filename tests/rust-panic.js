#!/usr/bin/env node
process.env.RUST_BACKTRACE = 1;
const valet = require("..").default;
valet.rustPanic();
