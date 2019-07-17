const indent = require("../util/indent");
const stmts = require("./statements");

const scope = (ast, state, utils) => {
  let innerVars = {};
  const statements = ast.map(stmt => {
    try {
      const out = statement(stmt, {...state, varTypes: {...state.varTypes, ...innerVars}}, utils);
      if (out.newVars) {
        innerVars = {...innerVars, ...out.newVars};
      }
      return out.compiled;
    } catch (e) {
      console.log(e)
      throw `${e}\n  In function ${state.functionName}`;
    }
  });
  return { compiled: statements.join("\n") };
};

function statement(ast, state, utils) {
  return stmts[ast.type](ast, state, {...utils, scope});
}

module.exports = scope;
