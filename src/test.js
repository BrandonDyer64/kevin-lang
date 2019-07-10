const YAML = require('yaml');
const Tracer = require('pegjs-backtrace');

const source = `
import Console from "console";

fn int Main() {
  43 + 2 + 3;
}

export Main;
`;

const tracer = new Tracer(source, {
});

const parser = require('./parser');

console.log('\n\n##### PARSER GENERATED SUCCESSFULLY #####\n');

try {
  console.log(YAML.stringify(parser.parse(source, {tracer})));
} catch(e) {
  console.log(tracer.getBacktraceString());
}
