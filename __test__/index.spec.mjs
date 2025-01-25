import test from "ava";

import { WaveFormatStruct } from "../index.js";

// console.log(WaveFormatStruct.getStatic());
const a = new WaveFormatStruct(1, 1, 1, 1, 1);

// console.log(a.getDevice());
a.init();
a.start((val) => {
  // console.log("start", val);
});
// setTimeout(() => {
//   a.setStatus(0);
// }, 1000);
