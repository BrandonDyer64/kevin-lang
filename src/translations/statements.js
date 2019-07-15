const expression = require("./expression");

function assignment(ast, state) {
  let expr;
  try {
    expr = expression(ast.expr, state);
  } catch (e) {
    throw `${e}\n  Assigning ${ast.name}`;
  }

  if (!expr) throw `Could not parse expression ${ast.expr.type}`;
  if (expr.type === "void") throw `Cannot set a variable to type 'void'`;
  if (ast.staticType === "void") throw `Variable can't be a 'void'`;
  if (ast.dec && ast.name in state.varTypes)
    throw `${ast.name} has already been declared`;
  if (!(ast.dec || ast.name in state.varTypes))
    throw `${ast.name} hasn't been defined`;
  if (ast.name in state.varTypes && state.varTypes[ast.name].dec === "let")
    throw `${ast.name} is immutable`;
  if (ast.name in state.varTypes && state.varTypes[ast.name].type !== expr.type)
    throw `${ast.name} is ${state.varTypes[ast.name].type}, not ${expr.type}`;
  if (ast.dec && ast.staticType && ast.staticType !== expr.type)
    throw `${ast.name} is ${ast.staticType}, not ${expr.type}`;

  let newVars = {};
  let type = ast.staticType || expr.type;

  if (ast.dec) {
    newVars = {
      [ast.name]: {
        dec: ast.dec,
        mut: ast.mut,
        type: expr.type
      }
    };
  } else {
  }

  return {
    newVars,
    compiled: `${ast.dec ? type + " " : ""}${ast.name} = ${expr.compiled};`
  };
}

function returnSt(ast, state) {
  let expr;
  try {
    expr = expression(ast.expr, state);
  } catch (e) {
    throw `${e}\n  Returning ${ast.expr.type} ${ast.expr.name || ""}`;
  }
  if (expr.type !== state.returnType) {
    throw `${state.functionName} returns ${state.returnType}, not ${expr.type}`;
  }
  return {
    newVars: {},
    compiled: `return ${expr.compiled};`
  };
}

module.exports = {
  assignment,
  return: returnSt
};
