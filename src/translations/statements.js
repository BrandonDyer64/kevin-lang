const expression = require('./expression');

function assignment(ast, state) {
  let expr;
  try {
    expr = expression(ast.expr, state);
  } catch(e) {
    throw `${e}\n  Assigning ${ast.name}`;
  }

  if (!expr) throw `Could not parse expression ${ast.expr.type}`;

  let newVars = {};

  if (ast.dec) {
    newVars = {
      dec: ast.dec,
      mut: ast.mut,
      type: expr.type
    };
  } else {
    if (!(ast.name in state.varTypes)) {
      throw `${ast.name} hasn't been defined`;
    }
  }





  const type = '';
  return {
    newVars,
    compiled: `${type ? type + ' ': ''}${ast.name} = ${expr.compiled};`
  };
}

function returnSt(ast, state) {
  let expr;
  try {
    expr = expression(ast.expr, state);
  } catch (e) {
    throw `${e}\n  Returning ${ast.expr.type} ${ast.expr.name || ''}`;
  }
  if (expr.type !== state.returnType) {
    throw `${state.functionName} returns ${state.returnType}, not ${expr.type}`;
  }
  return {
    newVars: {},
    compiled: `return ${expr.compiled};`
  }
}

module.exports = {
  assignment,
  return: returnSt
};
