const fs = require("fs");

const X = 0;
const Y = 1;

const input = fs.readFileSync("input.txt", { encoding: "utf8" });

const lines = input
  .split("\n")
  .filter(Boolean)
  .map(line =>
    line.split(",").map(([direction, ...digits]) => ({
      direction,
      amount: parseInt(digits.join(""), 10)
    }))
  );

const grid = {};

const crossingPoints = [];

lines.forEach((line, lineIdx) => {
  let coords = [0, 0];
  line.forEach(({ direction, amount }, commandIdx) => {
    let axis, step;
    switch (direction) {
      case "U":
        axis = Y;
        step = +1;
        break;
      case "D":
        axis = Y;
        step = -1;
        break;
      case "L":
        axis = X;
        step = -1;
        break;
      case "R":
        axis = X;
        step = +1;
    }

    for (let i = 1; i <= amount; ++i) {
      coords[axis] += step;
      const gridIdx = `${coords[X]},${coords[Y]}`;
      const value = grid[gridIdx] || {};
      value[lineIdx] = true;
      grid[gridIdx] = value;
      if (Object.keys(value).length === lines.length) {
        crossingPoints.push({ x: coords[X], y: coords[Y] });
      }
    }
  });
});

console.log(crossingPoints);

const smallestDistance = crossingPoints.reduce((smallest, coord) => {
  const distance = Math.abs(coord.x) + Math.abs(coord.y);
  if (distance < smallest) {
    return distance;
  }
  return smallest;
}, Number.POSITIVE_INFINITY);

console.log("Smallest distance", smallestDistance);
