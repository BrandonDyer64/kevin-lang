# Kevin Specification

### Function Definition

```
fn [type] <Name>([arg]*) {
  [statement]+
}
```

#### Example

```
fn String Trim(String str) {
  // Do string trim
}

fn NoReturnType() {
  // This has no params or return
}
```

### Structs

```
struct <Name> ['extends' [<Type> [',']]+] {
  [values]+
}
```

#### Example

```
struct A {
  float x;
  float y;
  (float a, float b) {
    x = a;
    y = b;
  }
};
```

```
struct D extends A, B, C {
  float z;
};
```

### Struct Functions

```
fn [type] <Struct>::<Function>([arg]*) {
  [statement]*
}
```

#### Example

```
fn float A::Sum() {
  return self.a + self.b;
}
```

```
fn Main() {
  let a = new A(2, 3);
  let a_sum = a |> Sum;
}
```
