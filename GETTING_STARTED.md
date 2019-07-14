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

## Control Flow

### If / Else

```cpp
if is_happy {
  Console::Print("I am happy");
} else {
  Console::Print("I am not happy");
}

let am_i_happy = if is_happy {
  "I am happy";
} else {
  "I am not happy";
};
```

### Switch

```cpp
switch favorite_number {
  5, 8 => {
    Console::Print("Great favorite number!");
  },
  7, 9 => {
    Console::Print("Worst number ever...");
  },
  _ => {
    Console::Print("Meh.");
  }
}

let what_do_you_think = switch favorite_number {
  5, 8 => "Great!",
  7, 9 => "Bad...",
  _ => "Meh."
};
```

## Expressions

```cpp
let a = {
  let c = b - d;
  c + 2;
};
```

## Pointers

There are three kinds of pointer in Kevin.

| -       | -        |
| ------- | -------- |
| `Type`  | data     |
| `Type*` | safe     |
| `Type?` | nullable |
| `Type!` | unsafe   |

### Data Value

Raw data stored in a variable. You can use this without fear of null or any
unsafe shenanigans.

```rust
fn Fun(Vector vec) {
  Console::Print("X: ${vec.x}, Y: ${vec.y}");
}
```

### Safe Pointer

A safe pointer is a pointer to a location the compiler is confident contains
safe data of the correct type.

```rust
fn Fun(Vector* vec_ptr) {
  let vec = vec_ptr!;
}
```

### Nullable Pointer

A nullable pointer is a pointer that the compiler knows is either pointing to
safe data, or to null.

```rust
fn Fun(Vector? vec_ptr) {
  let vec = vec_ptr! eject Vector(1, 2);

  // OR

  let vec = vec_ptr! eject return;
}
```

### Unsafe Pointer

An unsafe pointer could be pointing anywhere. To get the data from one of these,
you'll need to use the `unsafe` keyword. This should really only be used when
interacting with functions written in C/C++. All pointers coming from unmarked
C++ functions is considered unsafe.

```rust
fn Fun(Vector! vec_ptr) {
  unsafe let vec = vec_ptr!;
}
```

## Promises

```rust
fn Promise<Vector> FetchVector(String name) {
  let url = "https://api.myapi.com/get/vector/${name}";
  return Fetch(url, { method: "GET" })
    .Then(res => {
      return Vector(res.x.ParseFloat(), res.y.ParseFloat());
    });
}

fn UpdateEnemy(mut Actor enemy) {
  FetchVector(enemy.name)
    .Then(vec => {
      enemy.position = vec;
    });
}
```

## Symbols

Kevin doesn't have any sort of enum system. Instead, we use symbols.
A symbol is a constant whose value is its own name.

```cpp
:apple == :apple  // : true
:apple == :orange // : false
```

Symbols whos names are `true`, `false`, or `null` will have the same value.

```cpp
true == :true   // : true
false == :false // : true
false == :true  // : false
null == :null   // : true
```

An example usage

```cpp
box Enemy {
  symbol type;
}

fn DealDamage(Enemy enemy, Actor actor) {
  actor.healt -= switch enemy.type {
    :goomba => 1,
    :koopa, :shell => 2,
    :bowser => 10,
    _ => 0
  };
}
```
