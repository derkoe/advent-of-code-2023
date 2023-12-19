// run with deno run --allow-read src/day19.ts

import fs from "node:fs";

type Part = {
  x: number;
  m: number;
  a: number;
  s: number;
};

type Instruction = {
  field: string;
  operation: string;
  value: number;
  target: string;
};

type Workflow = {
  id: string;
  instructions: Instruction[];
  fallback: string;
};

type System = {
  workflows: {
    [key: string]: Workflow;
  };
  parts: Part[];
};

function parseInput(input: string): System {
  const [workflowsStr, partsStr] = input.split("\n\n");
  const workflows: System["workflows"] = {};
  workflowsStr.split("\n").forEach((w) => {
    const id = w.split("{")[0];
    const instr = w.split("{")[1].replace("}", "").split(",");
    const instructions = instr
      .filter((i) => i.includes(":"))
      .map((i) => {
        return {
          field: i[0],
          operation: i[1],
          value: Number.parseInt(i.slice(2).split(":")[0]),
          target: i.split(":")[1],
        } as Instruction;
      });
    workflows[id] = {
      id,
      instructions,
      fallback: instr[instr.length - 1],
    };
  });
  const parts = partsStr
    .split("\n")
    .filter((partsStr) => partsStr.trim().length > 0)
    .map((partStr) => {
      const part = {};
      partStr
        .replace("{", "")
        .replace("}", "")
        .split(",")
        .forEach((field) => {
          if (field.includes("=")) {
            const split = field.split("=");
            part[split[0]] = Number.parseInt(split[1]);
          }
        });
      return part as Part;
    });
  return { workflows, parts };
}

function runWorkflow(
  workflows: {
    [key: string]: Workflow;
  },
  part: Part
): boolean {
  let workflow: Workflow | null = workflows["in"];
  while (workflow != null) {
    let target;
    for (let instruction of workflow.instructions) {
      if (instruction.field) {
        if (instruction.operation === "<") {
          if (part[instruction.field] < instruction.value!) {
            target = instruction.target;
            break;
          }
        } else if (instruction.operation === ">") {
          if (part[instruction.field] > instruction.value!) {
            target = instruction.target;
            break;
          }
        }
      }
    }
    target = target || workflow.fallback;

    if (target === "A") {
      return true;
    }
    if (target === "R") {
      return false;
    }
    if (target) {
      workflow = workflows[target];
    }
  }
  return false;
}

const fileContents = fs.readFileSync("./input/2023/day19.txt").toString();
const system = parseInput(fileContents);

let part1 = system.parts
  .filter((part) => runWorkflow(system.workflows, part))
  .map((part) => part.x + part.m + part.a + part.s)
  .reduce((a, b) => a + b, 0);
console.log("Part 1:", part1);

type Range = {
  from: Part;
  to: Part;
};

function findAcceptedRanges(
  workflows: {
    [key: string]: Workflow;
  },
  currentWorkflow: string,
  ranges: Range[]
): Range[] {
  if (currentWorkflow === "A") {
    return ranges;
  }
  if (currentWorkflow === "R") {
    return [];
  }

  let workflow = workflows[currentWorkflow];
  let acceptedRanges: Range[] = [];
  while (ranges.length > 0) {
    for (let instruction of workflow.instructions) {
      if (ranges.length === 0) {
        break;
      }
      const range = ranges.pop()!;
      const subRanges: Range[] = [];
      if (instruction.operation === "<") {
        if (range.to[instruction.field] < instruction.value) {
          subRanges.push(range);
        } else if (range.from[instruction.field] < instruction.value) {
          subRanges.push({
            from: range.from,
            to: {
              ...range.to,
              [instruction.field]: instruction.value - 1,
            },
          });
          ranges.push({
            from: {
              ...range.from,
              [instruction.field]: instruction.value,
            },
            to: range.to,
          });
        }
      } else if (instruction.operation === ">") {
        if (range.from[instruction.field] > instruction.value) {
          subRanges.push(range);
        } else if (range.to[instruction.field] > instruction.value) {
          subRanges.push({
            from: {
              ...range.from,
              [instruction.field]: instruction.value + 1,
            },
            to: range.to,
          });
          ranges.push({
            from: range.from,
            to: {
              ...range.to,
              [instruction.field]: instruction.value,
            },
          });
        }
      }
      if (subRanges.length > 0) {
        acceptedRanges = acceptedRanges.concat(
          findAcceptedRanges(workflows, instruction.target, subRanges)
        );
      }
    }
    acceptedRanges = acceptedRanges.concat(
      findAcceptedRanges(workflows, workflow.fallback, ranges)
    );
  }
  return acceptedRanges;
}

let ranges = findAcceptedRanges(system.workflows, "in", [
  {
    from: { x: 1, m: 1, a: 1, s: 1 },
    to: { x: 4000, m: 4000, a: 4000, s: 4000 },
  },
]);

console.log(
  "Part 2:",
  ranges.reduce(
    (sum, range) =>
      sum +
      (range.to.a - range.from.a + 1) *
        (range.to.m - range.from.m + 1) *
        (range.to.s - range.from.s + 1) *
        (range.to.x - range.from.x + 1),
    0
  )
);
