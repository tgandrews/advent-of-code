const fs = require("fs");

const file = fs.readFileSync("./input.txt", { encoding: "utf8" });
const lines = file.split("\n");

const result = lines
  .map(l => parseInt(l, 10))
  .reduce((sum, mass) => sum + (Math.floor(mass / 3) - 2), 0);

console.log("Result: ", result);
