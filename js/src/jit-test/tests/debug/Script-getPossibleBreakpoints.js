// simple ExpressionStatement
assertBreakpoints(`
  /*S*/a;
  /*S*/obj.prop;
`);

// ExpressionStatement with calls
assertBreakpoints(`
  /*S*/a/*B*/();
  /*S*/obj./*B*/prop();
`);

// ExpressionStatement with nested expression calls.
assertBreakpoints(`
  "45";
  /*S*/"45" + /*B*/a();
  /*S*/b/*B*/() + "45";

  /*S*/"45" + o./*B*/a();
  /*S*/o./*B*/b() + "45";
  /*S*/"45" + o./*B*/a() + o./*B*/b();
  /*S*/o./*B*/b() + "45" + o./*B*/a();
  /*S*/o./*B*/b() + o./*B*/a() + "45";
`);

// var VariableStatement initializers
assertBreakpoints(`
  var foo1 = /*S*/"" + o.a + "" + /*B*/b(),
      foo2 = /*S*/"45",
      foo3 = /*S*/"45" + /*B*/a(),
      foo4 = /*S*/b/*B*/() + "45",
      foo5 = /*S*/"45" + /*B*/a() + /*B*/b(),
      foo6 = /*S*/b/*B*/() + "45" + /*B*/a(),
      foo7 = /*S*/b/*B*/() + /*B*/a() + "45",
      foo8 = /*S*/"45" + o./*B*/a(),
      foo9 = /*S*/o./*B*/b() + "45",
      foo10 = /*S*/"45" + o./*B*/a() + o./*B*/b(),
      foo11 = /*S*/o./*B*/b() + "45" + o./*B*/a(),
      foo12 = /*S*/o./*B*/b() + o./*B*/a() + "45";
`);

// let VariableStatement initializers
assertBreakpoints(`
  let foo1 = /*S*/"" + o.a + "" + /*B*/b(),
      foo2 = /*S*/"45",
      foo3 = /*S*/"45" + /*B*/a(),
      foo4 = /*S*/b/*B*/() + "45",
      foo5 = /*S*/"45" + /*B*/a() + /*B*/b(),
      foo6 = /*S*/b/*B*/() + "45" + /*B*/a(),
      foo7 = /*S*/b/*B*/() + /*B*/a() + "45",
      foo8 = /*S*/"45" + o./*B*/a(),
      foo9 = /*S*/o./*B*/b() + "45",
      foo10 = /*S*/"45" + o./*B*/a() + o./*B*/b(),
      foo11 = /*S*/o./*B*/b() + "45" + o./*B*/a(),
      foo12 = /*S*/o./*B*/b() + o./*B*/a() + "45";
`);

// const VariableStatement initializers
assertBreakpoints(`
  const foo1 = /*S*/"" + o.a + "" + /*B*/b(),
        foo2 = /*S*/"45",
        foo3 = /*S*/"45" + /*B*/a(),
        foo4 = /*S*/b/*B*/() + "45",
        foo5 = /*S*/"45" + /*B*/a() + /*B*/b(),
        foo6 = /*S*/b/*B*/() + "45" + /*B*/a(),
        foo7 = /*S*/b/*B*/() + /*B*/a() + "45",
        foo8 = /*S*/"45" + o./*B*/a(),
        foo9 = /*S*/o./*B*/b() + "45",
        foo10 = /*S*/"45" + o./*B*/a() + o./*B*/b(),
        foo11 = /*S*/o./*B*/b() + "45" + o./*B*/a(),
        foo12 = /*S*/o./*B*/b() + o./*B*/a() + "45";
`);

// EmptyStatement
assertBreakpoints(`
  ;
  ;
  ;
  /*S*/a/*B*/();
`);

// IfStatement
assertBreakpoints(`
  if (/*S*/a) {}
  if (/*S*/a/*B*/()) {}
  if (/*S*/obj.prop) {}
  if (/*S*/obj./*B*/prop()) {}
  if (/*S*/"42" + a) {}
  if (/*S*/"42" + /*B*/a()) {}
  if (/*S*/"42" + obj.prop) {}
  if (/*S*/"42" + obj./*B*/prop()) {}
`);

// DoWhile
assertBreakpoints(`
  do {
    /*S*/fn/*B*/();
  } while(/*S*/a)
  do {
    /*S*/fn/*B*/();
  } while(/*S*/"42" + /*B*/a());
`);

// While
assertBreakpoints(`
  while(/*S*/a) {
    /*S*/fn/*B*/();
  }
  while(/*S*/"42" + /*B*/a()) {
    /*S*/fn/*B*/();
  }
`);

// ForExpr
assertBreakpoints(`
  for (/*S*/b = 42; /*S*/c; /*S*/d) /*S*/fn/*B*/();
`);

// ForVar
assertBreakpoints(`
  for (var b = /*S*/42; /*S*/c; /*S*/d) /*S*/fn/*B*/();
`);

// ForLet
assertBreakpoints(`
  for (let b = /*S*/42; /*S*/c; /*S*/d) /*S*/fn/*B*/();
`);

// ForConst
assertBreakpoints(`
  for (const b = /*S*/42; /*S*/c; /*S*/d) /*S*/fn/*B*/();
`);

// ForInExpr
assertBreakpoints(`
  for (b in /*S*/d) /*S*/fn/*B*/();
`);
// ForInVar
assertBreakpoints(`
  for (var b in /*S*/d) /*S*/fn/*B*/();
`);
// ForInLet
assertBreakpoints(`
  for (let b in /*S*/d) /*S*/fn/*B*/();
`);
// ForInConst
assertBreakpoints(`
  for (const b in /*S*/d) /*S*/fn/*B*/();
`);

// ForOfExpr
assertBreakpoints(`
  for (b of /*S*/d) /*S*/fn/*B*/();
`);
// ForOfVar
assertBreakpoints(`
  for (var b of /*S*/d) /*S*/fn/*B*/();
`);
// ForOfLet
assertBreakpoints(`
  for (let b of /*S*/d) /*S*/fn/*B*/();
`);
// ForOfConst
assertBreakpoints(`
  for (const b of /*S*/d) /*S*/fn/*B*/();
`);

// SwitchStatement
assertBreakpoints(`
  switch (/*S*/d) {
    case 42:
      /*S*/fn/*B*/();
  }
`);

// ContinueStatement
assertBreakpoints(`
  while (/*S*/a) {
    /*S*/continue;
  }
`);

// BreakStatement
assertBreakpoints(`
  while (/*S*/a) {
    /*S*/break;
  }
`);

// ReturnStatement
assertBreakpoints(`
  /*S*/return a + /*B*/b();
`);

// WithStatement
assertBreakpoints(`
  with (/*S*/a) {
    /*S*/fn/*B*/();
  }
`);

// ThrowStatement
assertBreakpoints(`
  /*S*/throw /*B*/fn();
  /*S*/throw "42" + /*B*/fn();
`);

// DebuggerStatement
assertBreakpoints(`
  /*S*/debugger;
  /*S*/debugger;
`);

// BlockStatent wrapper
assertBreakpoints(`
  {
    /*S*/a/*B*/();
  }
`);

// ClassDeclaration
assertBreakpoints(`
  class Foo2 {}
  /*S*/class Foo extends ("" + o.a + /*B*/a() + /*B*/b()) { }
`);

// Misc examples
assertBreakpoints(`
  /*S*/void /*B*/a();
`);
assertBreakpoints(`
  /*S*/a/*B*/() + /*B*/b();
`);
assertBreakpoints(`
  for (
    var i = /*S*/0;
    /*S*/i < n;  // 4
    /*S*/++i
  ) {
    /*S*/console./*B*/log("omg");
  }
`);
assertBreakpoints(`
  function * gen(){
    var foo = (
      (/*S*/console./*B*/log('before', /*B*/a())),
      (yield console./*B*/log('mid', /*B*/b())),
      (console./*B*/log('after', /*B*/a()))
    );
    var foo2 = /*S*/a/*B*/() + /*B*/b();
    /*S*/console./*B*/log(foo);
  /*B*/}
  var i = /*S*/0;
  for (var foo of /*S*/gen/*B*/()) {
    /*S*/console./*B*/log(i++);
  }
`);
assertBreakpoints(`
  var fn = /*S*/() => {
    /*S*/console./*B*/log("fn");
    /*S*/return /*B*/new Proxy({ prop: 42 }, {
      deleteProperty() {
        /*S*/console./*B*/log("delete");
      /*B*/}
    });
  /*B*/};
`);
assertBreakpoints(`
  var fn = /*S*/async (arg) => {
    /*S*/console./*B*/log("fn");
  /*B*/};
`);
assertBreakpoints(`
  var fn = /*S*/arg => {
    /*S*/console./*B*/log("fn");
  /*B*/};
`);
assertBreakpoints(`
  var fn = /*S*/async arg => {
    /*S*/console./*B*/log("fn");
  /*B*/};
`);
assertBreakpoints(`
  var fn = /*S*/(arg) => /*S*/console./*B*/log("fn");
  var fn = /*S*/async (arg) => /*S*/console./*B*/log("fn");
  var fn = /*S*/arg => /*S*/console./*B*/log("fn");
  var fn = /*S*/async arg => /*S*/console./*B*/log("fn");
`);
assertBreakpoints(`
  if ((/*S*/delete /*B*/fn().prop) + /*B*/b()) {
    /*S*/console./*B*/log("foo");
  }
`);
assertBreakpoints(`
  for (var j = /*S*/0; (/*S*/o.a) < 3; (/*S*/j++, /*B*/a(), /*B*/b())) {
    /*S*/console./*B*/log(i);
  }
`);
assertBreakpoints(`
  function fn2(
    [a, b] = (/*B*/a(), /*B*/b())
  ) {
    /*S*/a/*B*/();
    /*S*/b/*B*/();
  /*B*/}

  ({ a, b } = (/*S*/a/*B*/(), /*B*/b()));
`);
assertBreakpoints(`
  /*S*/o.a + "42" + /*B*/a() + /*B*/b();
`);
assertBreakpoints(`
  /*S*/a/*B*/();
  /*S*/o./*B*/a(/*B*/b());
`);
assertBreakpoints(`
  (/*S*/{}[obj.a] = 42 + /*B*/a());
`);
assertBreakpoints(`
  var {
    foo = o.a
  } = /*S*/{};
`);
assertBreakpoints(`
  var ack = /*S*/[
    o.a,
    o.b,
    /*B*/a(),
    /*B*/a(),
    /*B*/a(),
    /*B*/a(),
    /*B*/a(),
    /*B*/a(),
    /*B*/a(),
  ];
`);

function assertBreakpoints(expected) {
  const input = expected.replace(/\/\*[BS]\*\//g, "");

  var global = newGlobal({ newCompartment: true });
  var dbg = Debugger(global);
  dbg.onDebuggerStatement = function(frame) {
    const fScript = frame.environment.parent.getVariable("f").script;

    let positions = [];
    (function recurse(script) {
      const bps = script.getPossibleBreakpoints();
      const offsets = script.getPossibleBreakpointOffsets();

      assertEq(offsets.length, bps.length);
      for (let i = 0; i < bps.length; i++) {
        assertEq(offsets[i], bps[i].offset);
      }

      positions = positions.concat(bps);
      script.getChildScripts().forEach(recurse);
    })(fScript);

    const result = annotateOffsets(input, positions);
    assertEq(result, expected + "/*B*/");
  };

  global.eval(`function f(){${input}} debugger;`);
}

function annotateOffsets(code, positions) {
  const offsetLookup = createOffsetLookup(code);

  positions = positions.slice();
  positions.sort((a, b) => {
    const lineDiff = a.lineNumber - b.lineNumber;
    return lineDiff === 0 ? a.columnNumber - b.columnNumber : lineDiff;
  });
  positions.reverse();

  let output = "";
  let last = code.length;
  for (const { lineNumber, columnNumber, isStepStart } of positions) {
    const offset = offsetLookup(lineNumber, columnNumber);

    output =
      "/*" +
      (isStepStart ? "S" : "B") +
      "*/" +
      code.slice(offset, last) +
      output;
    last = offset;
  }
  return code.slice(0, last) + output;
}

function createOffsetLookup(code) {
  const lines = code.split(/(\r?\n|\r|\u2028|\u2029)/g);
  const lineOffsets = [];

  let count = 0;
  for (const [i, str] of lines.entries()) {
    if (i % 2 === 0) {
      lineOffsets[i / 2] = count;
    }
    count += str.length;
  }

  return function(line, column) {
    // Lines from getAllColumnOffsets are 1-based.
    line = line - 1;

    if (!lineOffsets.hasOwnProperty(line)) {
      throw new Error("Unknown line " + line + " column " + column);
    }
    return lineOffsets[line] + column;
  };
}
