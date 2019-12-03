const fs = require("fs");

const input = fs.readFileSync("input.txt", { encoding: "utf8" });

const list = input
  .trim()
  .split(",")
  .map(index => parseInt(index, 10));

list[1] = 12;
list[2] = 2;

let i = 0;
while (true) {
  const [opcode, in1, in2, output] = list.slice(i, i + 4);
  let computed;
  if (opcode === 1) {
    computed = list[in1] + list[in2];
  } else if (opcode === 2) {
    computed = list[in1] * list[in2];
  } else if (opcode === 99) {
    break;
  } else {
    throw new Error("Unknown opcode: " + opcode);
  }

  list[output] = computed;
  i = i + 4;
}

console.log("output", list.join(","));
