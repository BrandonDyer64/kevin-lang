# Kevin Specification

Kevin is my programming language.
Kevin is a low-level systems language.
Kevin is a memory safe alternative to C++ that isn't Rust.
Kevin's a girl.

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

### Boxes

```
box <Name> ['extends' [<Type> [',']]+] {
  [values]+
}
```

#### Example

```
box A {
  float x;
  float y;
  (float a, float b) {
    x = a;
    y = b;
  }
};
```

```
box D extends A, B, C {
  float z;
};
```

### Box Functions

```
fn [type] <Function>(<Box> self [, <arg>]*) {
  [statement]*
}
```

#### Example

```
fn float Sum(A self) {
  return self.a + self.b;
}
```

```
fn Main() {
  let a = new A(2, 3);
  let a_sum = a |> Sum;
}
```

## Memory safety

### Lend memory from parent

Nothing should ever go on the super heap (the heap provided by the OS).
Instead, we should put everything on the stack.

#### Example

```
fn Child() {
  let a = $(24);
}

fn Parent() {
  Child();
}
```

#### Translates to this

```
fn Child(int* a) {
  *a = 24;
}

fn Parent() {
  let int* a;
  Child(a);
}
```

### Passing farther

```
fn Grandchild() {
  let a = $[24];
}

fn Child() {
  $Grandchild();
}

fn Parent() {
  Child();
}
```

#### Translates to This

```
fn Grandchild(int* a) {
  *a = 24;
}

fn Child(int* a) {
  Grandchild(a);
}

fn Parent() {
  let int* a;
  Child(a);
}
```

## Built-in threading

### Array mapping

```
// Iterates over an array and maps its values.
// Because each value is independent of every other value
// we can run each iteration in its own thread.
fn Main() {
  let my_array = [ "Hello", "Goodbye", "Saluton al" ];
  let my_new_array = my_array |> Map([](String val, int i) {
    return val + " World!";
  })
  // [ "Hello World!", "Goodbye World!", "Saluton al World!" ]
}
```

### Async statements

```
fn Main() {
  // These two statements don't rely on each other.
  // We can run each in their own thread.
  async let a = SomethingExpensive();
  async let b = SomethingElseExpensive();

  // Wait until both are finished then continue
  Console::Print(a);
  Console::Print(b);
}
```

## Wiggles

Wiggley data gives bad guys looking at your memory a run for their money.

```
fn Main() {
  // Create a wiggley float with a value of 24
  let my_wiggle = Wiggley<float>(24);

  // Jiggle the wiggle
  Jiggle <| my_wiggle;
}
```

### Break down

```
fn Main() {

  // Memory: [ 00 00 00 00 ]

  let my_wiggle = Wiggley<float>(24);

  // Memory: [ 21 04 00 00 ]

  Jiggle <| my_wiggle;

  // Memory: [ 13 11 00 00 ]

  let my_other_wiggle = Wiggley<float>(6);

  // Memory: [ 13 11 02 04 ]

}
