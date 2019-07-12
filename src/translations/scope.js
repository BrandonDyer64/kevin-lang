const indent = require("../util/indent");
const stmts = require("./statements");

module.exports = (ast, state) => {
  let innerVars = {};
  const statements = ast.map(stmt => {
    try {
      const out = statement(stmt, {...state, varTypes: {...state.varTypes, ...innerVars}});
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

function statement(ast, state) {
  return stmts[ast.type](ast, state);
}
