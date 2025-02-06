import test from "ava";

// import { WaveFormatStruct } from "../index.js";

test("sync function from native code", (t) => {
  const fixture = 42;
  t.deepEqual([1, 2], [1, 2]);
});

// const a = new WaveFormatStruct(1, 1, 1, 1, 1);

// // console.log(a.getDevice());
// a.init();
// a.start((val) => {
//   // console.log("start", val);
// });
// setTimeout(() => {
//   console.log("stop");
//   a.stop();
// }, 1000);
