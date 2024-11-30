const { readFileSync } = require("fs");

const file = "./file.txt";
function partOne(file) {
  let totalSum = 0;
  const lines = readFileSync(file, "utf-8").trim().split("\n");
  let sum = 0;
  const cardCopyTracker = lines.map(() => 1);

  lines.map((line, idx) => {
    const [_, ...rest] = line.split(":").map((item) => item.trim());
    const [[winNumber, myNumber]] = rest.map((item) =>
      item.split("|").map((item) => item.trim().match(/\d+/g))
    );
    let found = 0;
    for (let i = 0; i < myNumber.length; i++) {
      if (myNumber.includes(winNumber[i])) {
        found++;

        if (found === 1) {
          sum++;
        } else {
          sum = sum * 2;
        }
      }
    }

    const currentCardCopies = cardCopyTracker[idx];
    const startIdx = idx + 1;

    const endIdx = Math.min(lines.length - 1, idx + found);

    for (let i = startIdx; i <= endIdx; i++) {
      cardCopyTracker[i] += currentCardCopies;
    }

    // console.log(sum);
    totalSum += sum;
    sum = 0;
  });
  const result = cardCopyTracker.reduce((acc, count) => (acc += count), 0);
  console.log(result);

}
console.log(partOne(file));
