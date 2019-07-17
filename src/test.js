const YAML = require("yaml");
const Tracer = require("pegjs-backtrace");
const child_process = require("child_process");
const fs = require('fs');
const parser = require("./parser");
const { translate } = require("./translator");

const source = `
fn int Main() {
  let a = :apple;
  if a == :orange {
    Console::Print("a is an orange");
  } else {
    Console::Print("a is not an orange");
  }
}
`;

const tracer = new Tracer(source, {});

console.log("\n\n##### PARSER GENERATED SUCCESSFULLY #####\n");

try {
  const ast = parser.parse(source, { tracer });
  console.log(YAML.stringify(ast));
  try {
    const compiled = translate(ast).compiled;
    console.log(compiled);
    fs.writeFileSync('test.cpp', compiled);
    child_process.execSync('g++ test.cpp -o test --std=c++17');
    const out = child_process.execFileSync('./test', {encoding: 'utf8'});
    console.log(out);
  } catch (e) {
    throw `${e}\n  In file Test.kv`;
  }
} catch (e) {
  console.log(tracer.getBacktraceString());
  console.log(e);
}
