# Welcome to Kevin!

Kevin is a strongly typed, low-level, systems language that compiles directly to
C++. Think of it like the type-script of C++.

## Hello World

```cpp
fn Main() {
  Console::Print("Hello, World!");
}
// : Hello, World!
```

## Template Strings

```rust
let hello = "Hello";
let who = "World";
let my_string = "${hello}, ${who}!";
// : Hello, World!
```

## Types

### Primitives

| -         | -                                  |
| --------- | ---------------------------------- |
| integer   | `i8` `i16` `i32` `i64` `i128` `i0` |
| unsigned  | `u8` `u16` `u32` `u64` `u128` `u0` |
| float     | `f32` `f64`                        |
| character | `char`                             |
| boolean   | `bool`                             |

### Compounds

| -     | -                     |
| ----- | --------------------- |
| array | `[a, b, c]`           |
| tuple | `(a, b, c)`           |
| box   | `{ a: a, b: b, c: c}` |

## Variables

```cpp 
// Unchangable
let a = 1;

// Changable
var b = 2;
b = 3;

// Typed
let String str = "Hello, World!";

// Mutable
let mut vector = Vector2(1.2, 2.3);
vector.x = 6.8;

// Watch
let watch vector_watching = vector;
vector.y = 3.1;
```

## Boxes

```cpp
box Vector {

  // Fields
  float x;
  float y;
  
  // Factory
  (float x, float y) {
    return { x, y };
  }
  
}
```

## Functions

### Declarations

```cpp
fn Vector Add(Vector a, Vector b) {
  return Vector(a.x + b.x, a.y + b.y);
}
```


### Chains

```cpp
// These functions exist:
fn String Trim(String);
fn String AddHello(String);
fn String Uppercase(String);

// These are the same:
let my_str = "  Bob  ".Trim().AddHello().Uppercase();
let my_str = Uppercase(AddHello(Trim("  Bob  ")));

// : "HELLO BOB"
```
