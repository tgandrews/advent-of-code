const fs = require("fs");

const input = fs.readFileSync("input.txt", { encoding: "utf8" });

const initialList = input
  .trim()
  .split(",")
  .map(index => parseInt(index, 10));

for (let noun = 0; noun < 100; noun++) {
  for (let verb = 0; verb < 100; verb++) {
    const list = initialList.slice(0);
    list[1] = noun;
    list[2] = verb;

    try {
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
    } catch (e) {
      continue;
    }

    if (list[0] === 19690720) {
      console.log("!!! DONE !!!");
      console.log({ noun, verb });
      process.exit(0);
    }
  }
}
