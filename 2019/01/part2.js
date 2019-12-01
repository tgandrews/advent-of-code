const fs = require("fs");

const file = fs.readFileSync("./input.txt", { encoding: "utf8" });
const lines = file.split("\n");

const massToFuel = mass => {
  const fuel = Math.floor(mass / 3) - 2;
  if (fuel <= 0) {
    return 0;
  }
  return fuel + massToFuel(fuel);
};

const result = lines
  .map(l => parseInt(l, 10))
  .reduce((sum, mass) => sum + massToFuel(mass), 0);

console.log("Result: ", result);
