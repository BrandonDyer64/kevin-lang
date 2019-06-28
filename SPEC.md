# Kevin Specification

A lightweight transpiled language interoperable with C/C++.

## Variables

```
let name = expression; // Immutable variable value
var name = expression; // Mutable variable value
```

## Functions

### Function Definition

```
fn Type Name(Type arg1, Type arg2) {
  // ...
}
```

### Function Chaining

```
FunC(FunB(FunA(5)));

// Can be written as
5.FunA().FunB().FunC();
```

## Boxes

```
box Name {
  Type val1;
  Type val2;
}
```

## Memory safety

### Lend memory from parent

Nothing should ever go on the super heap (the heap provided by the OS).
Instead, we should put everything on the stack.

```
fn Child() {
  // 24 goes in the parent stack-frame
  let a = $[24];
}

fn Parent() {
  // Parent makes room for 24 here
  Child();
}
```

### Passing farther

```
fn Grandchild() {
  // Pass 24 to the parent
  let a = $[24];
}

fn Child() {
  // Pass the heap signature to the parent
  $Grandchild();
}

fn Parent() {
  // Parent makes room for 24 here
  Child();
}
```

## Mutablility

### Variable definitions

```
let a = ABox(6);
a.b = 4;     // Illegal
a = ABox(5); // Illegal

var a = ABox(6);
a.b = 4;     // Illegal
a = ABox(5); // Legal

let mut a = ABox(6);
a.b = 4;     // Legal
a = ABox(5); // Illegal

var mut a = ABox(6);
a.b = 4;     // Legal
a = ABox(5); // Legal
```

### Function parameters

```
fn (ABox a) {
  a.b = 4;     // Illegal
  a = ABox(5); // Legal
}

fn (mut ABox a) {
  a.b = 4;     // Legal
  a = ABox(5); // Legal
}
```

### Passing mutability

```
let a = ABox(6);
let mut b = a; // Illegal

let a = ABox(6);
let mut b = a.&;

let mut a = ABox(6);
let b = a; // Legal

let mut a = ABox(6);
let mut b = a; // Legal
```

```
fn Fun(mut ABox a) {}
let a = ABox(6);
Fun(a); // Illegal

fn Fun(mut ABox a) {}
let a = mut ABox(6);
Fun(a); // Legal
```

## Operators

| Op | Does | C++ equivalent |
| --- | --- | --- |
| + | .Add | + |
| - | .Sub | - |
| * | .Mult | * |
| / | .Div | / |
| % | .Mod | % |
| . | `a.b` = `a.b` | . / -> |
| . | `a.b()` = `b(a)` | |
| -> | | . |
| ->> | | -> |

## Expression logic

### If expression

```
let my_value = if (some_bool) {
  17;
} else {
  18;
}
```

### Switch expression

```
let my_value = switch (some_int) {
  6: "bad";
  7, 8, 9: "good";
  _: "¯\_(ツ)_/¯";
}
```

## Built-in threading

### Async statements

```
// These two statements don't rely on each other.
// We can run each in their own thread.
async let a = SomethingExpensive();
async let b = SomethingElseExpensive();

// Wait until both are finished then continue
Console::Print(a);
Console::Print(b);
```

```
async {
  let a = SomethingExpensive();
  let b = SomethingElseExpensive();
}
```

## Pointer types

```
int* -> safe
int? -> nullable
int! -> unsafe
```

### Safe

A safe pointer is a pointer who is guaranteed to be valid data.

```
let int a = 5;
let int* a_ptr = &a;

Console::Print(*a); // : 5
```

### Nullable

A nullable is a pointer who might either equal null or valid data.

```
let int? my_ptr = GetNullablePointer();

// Early return
let my_value = filter(my_ptr) return;

// -- OR --

// Default value
let my_value = filter(my_ptr) 17;
```

```
let int? my_ptr = GetPointer();
if let my_value = filter(my_ptr) {
  // pointer is valid
} else {
  // pointer is null
}
```

### Unsafe

An unsafe pointer is a pointer whos value may not be valid.
To dereference an unsafe pointer we must use the `unsafe` keyword.
Pointer validity is left to the discression of the writer when using `unsafe`.

#### Unsafe source

```
let int! my_unsafe_ptr = GetUnsafePointer();

unsafe let my_value = *my_unsafe_ptr;
```

#### Pointer arithmetic

```
let int* my_safe_ptr = GetSafePointer();
let int! my_unsafe_ptr = my_safe_ptr + 1;

unsafe let my_value = *my_unsafe_ptr;
```

#### Pointer casting

```
let int my_int = 24;
let int* my_int_ptr = &my_int;
let float! my_unsafe_ptr = Cast<float>(my_int_ptr);
unsafe let float my_value = *my_unsafe_ptr;
```

## Frames

Frames are a sort of window into a part of the memory.
These can be used like an array, but are not safe.

```
// This is the memory pattern of a frame
box <T> Frame {
  T* start;
  int length;
  
  (int length, T fill)
  (int length, fn(int) => T)
}
```

```
// Create a frame of size 12
let my_frame = Frame<int>(12, (i) => int { i });

let int! my_unsafe_ptr = my_frame[14];
let int? my_safe_ptr = my_frame[11]!;
let int my_value = filter(my_frame[11]!) return;
```

## Anonymous box

Sometimes it's nice to just pass data without the need of a dedicated box.

```
fn box GetABox(int a) {
  let b = 2;
  let c = 3;
  return { a, b, c };
}

fn Main() {
  let { a, b, c } = GetABox(1);
}
```
