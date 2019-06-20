# Kevin Specification

A lightweight transpiled language interoperable with C/C++.

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

A box is a structure that can only contain values and factories.

```
box A {
  // Values
  float x;
  float y;

  // Factory
  (float a, float b) {
    return {
      x: a,
      y: b
    };
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
  let a = $[24];
}

fn Parent() {
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
  // Declare the heap for a (24) here
  Child();
}
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

## Built-in threading

### Array mapping

```
// Iterates over an array and maps its values.
// Because each value is independent of every other value
// we can run each iteration in its own thread.
fn Main() {
  let my_array = [ "Hello", "Goodbye", "Saluton al" ];
  let my_new_array = my_array.Map([](String val, int i) {
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

## Compile-time

### If statements

```
fn Log(String out) {
  #if (debug) {
    Console::Print(out);
  } else {
    FS::WriteFile("log.txt", out);
  }
}
```

### Switches

```
fn String WhatPlatform() {
  #switch (platform) {
    "Linux" {
      return "Linux";
    }
    "MacOS" {
      return "MacOS";
    }
    "Windows" {
      return "Windows";
    }
    _ {
      return "¯\_(ツ)_/¯";
    }
  }
}
```

## Nullable

A nullable is a pointer that might equal null.

### Filtering

```
fn Main() {
  let int? my_ptr = GetPointer();
  let my_value = filter(my_ptr) return;
}
```

### if

```
fn Main() {
  let int? my_ptr = GetPointer();
  if let my_value = filter(my_ptr) {
    // pointer is valid
  } else {
    // pointer is null
  }
}
```

## Pointer types

```
int  -> raw data
int* -> safe pointer
int? -> safe, but possibly null, pointer
int! -> unsafe pointer
```

## Unsafe

### Unsafe pointers

```
fn Main() {
  let int! my_unsafe_ptr = GetUnsafePointer();

  // We don't know where my_unsafe_ptr is pointing, or if it's valid, but we'll
  // assume it's valid.

  unsafe let my_value = *my_unsafe_ptr;
}
```

### Pointer arithmetic

```
fn Main() {
  let int* my_safe_ptr = GetSafePointer();
  let int! my_unsafe_ptr = my_safe_ptr + 1;

  unsafe let my_value = *my_unsafe_ptr;
}
```

### Pointer casting

```
fn Main() {
  let int my_int = 24;
  let int* my_int_pointer = &my_int;
  let float! my_unsafe_ptr = cast[float; my_int_pointer];
  unsafe let float my_value = *my_unsafe_ptr;
}
```

## Frames

Frames are a sort of window into a part of the memory.
These can be used like an array, but are not safe.

```
// This is the memory pattern of a frame
box <T> Frame {
  T* start;
  int length;
}
```

```
fn Main() {
  // Create a frame of size 12
  let my_frame = [int; 12];

  let int! my_unsafe_ptr = my_frame[14];
  let int? my_safe_ptr = my_frame[11]!;
  let int my_value = filter(my_frame[11]!) return;
}
```

## Objects

Objects are high-level dynamic types that inteface directly with JavaScript.

```
fn Main() {
  let my_object = obj{
    float a: 12.6;
    int b: 6;
    String c: "Hi!";
  };
  let b = filter(my_object.b[int]) return;
  Console::Print(b);
}
```

Type must be known when recieved. Two values can have the same name if they have different types.

```
fn Main() {
  let my_object = obj{
    float value: 16.8;
    int value: 8;
  };
  my_object.value^String = "Hello, World!";
  let value = filter(my_object.value^String) return;
  Console::Print(value);
}
```

### JavaScript

```
#[JS_FUNCTION(require("a_module").someFunction)]
fn SomeJsFunction(Object)

fn Main() {
  let my_object = GetObject();
  SomeJsFunction(my_object);
}
```
