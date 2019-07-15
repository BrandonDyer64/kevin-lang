const YAML = require("yaml");
const Tracer = require("pegjs-backtrace");
const parser = require("./parser");
const { translate } = require("./translator");

const source = `
fn Test(int n) {
  let a = :apple;
  let b = :apple;
  let c = :orange;
  let d = :watermelon;
}
`;

const tracer = new Tracer(source, {});

console.log("\n\n##### PARSER GENERATED SUCCESSFULLY #####\n");

try {
  const ast = parser.parse(source, { tracer });
  console.log(YAML.stringify(ast));
  try {
    console.log(translate(ast).newAst.compiled);
  } catch (e) {
    throw `${e}\n  In file Test.kv`;
  }
} catch (e) {
  console.log(tracer.getBacktraceString());
  console.log(e);
}
