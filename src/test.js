const YAML = require("yaml");
const Tracer = require("pegjs-backtrace");
const parser = require("./parser");
const { translate } = require("./translator");

const source = `
fn int Factorial(int n) {
   if n == 0 || n == 1
      return 1;
   else
      return n * Factorial(n - 1);
}
`;

const tracer = new Tracer(source, {});

console.log("\n\n##### PARSER GENERATED SUCCESSFULLY #####\n");

try {
  const ast = parser.parse(source, { tracer });
  console.log(YAML.stringify(ast));
  try {
    console.log(YAML.stringify(translate(ast)));
  } catch (e) {
    throw `${e}\n  In file Test.kv`;
  }
} catch (e) {
  console.log(tracer.getBacktraceString());
  console.log(e);
}
