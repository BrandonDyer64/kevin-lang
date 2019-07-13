# Welcome to Kevin!

Kevin is a strongly typed, low-level, systems language that compiles directly to
C++. Think of it like the type-script of C++.

## Hello World

<table>
<tr><th>Kevin</th><th>C++</th></tr>
<tr>
<td>
<pre>
fn Main() {
  Console::Print("Hello, World!");
}
</pre>
</td>
<td>
<pre>
int main() {
  Console::Print(String(u8"Hello, World!"));
}
</pre>
</td>
</tr>
</table>

```
Hello, World!
```

## Template Strings

<table>
<tr><th>Kevin</th><th>C++</th></tr>
<tr>
<td>
<pre>
let hello = "Hello";
let who = "World";
let my_string = "${hello}, ${who}!";
</pre>
</td>
<td>
<pre>
String hello = String(u8"Hello");
String who = String(u8"Hello");
String my_string = hello + String(u8", ") + who + String(u8"!");
</pre>
</td>
</tr>
</table>

```
Hello, World!
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

<table>
<tr><th>Kevin</th><th>C++</th></tr>
<tr>
<td>
<pre>
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

</pre>
</td>
<td>
<pre>
int a = 1;

//
int b = 2;
b = 3;

//
String str = String(u8"Hello, World");

//
let mut vector = Vector2(1.2, 2.3);
vector.x = 6.8;

//
let watch vector_watching = vector;
vector.y = 3.1;

</pre>
</td>
</tr>
</table>

## Boxes

<table>
<tr><th>Kevin</th><th>C++</th></tr>
<tr>
<td>
<pre>
box Vector {
  // Fields
  float x;
  float y;

  // Factory
  (float x, float y) {
    return { x, y };
  }

}
</pre>
</td>
<td>
<pre>
struct Vector {
  //
  float x;
  float y;

  //
  Vector(float x, float y) {
    this->x = x;
    this->y = y;
  }
}
</pre>
</td>
</tr>
</table>

## Functions

### Declarations

<table>
<tr><th>Kevin</th><th>C++</th></tr>
<tr>
<td>
<pre>
fn Vector Add(Vector a, Vector b) {
  return Vector(a.x + b.x, a.y + b.y);
}
</pre>
</td>
<td>
<pre>
Vector Add(Vector a, Vector b) {
  return Vector(Add(a.x, b.x), Add(a.y, b.y));
}
</pre>
</td>
</tr>
</table>


### Chains

<table>
<tr><th>Kevin</th><th>C++</th></tr>
<tr>
<td>
<pre>
// These functions exist:
fn String Trim(String);
fn String AddHello(String);
fn String Uppercase(String);

// These are the same:
let my_str = "  Bob  ".Trim().AddHello().Uppercase();
let my_str = Uppercase(AddHello(Trim("  Bob  ")));

// : "HELLO BOB"
</pre>
</td>
<td>
<pre>
//
String Trim(String);
String AddHello(String);
String Uppercase(String);

//
String my_str = Uppercase(AddHello(Trim("  Bob  ")));

//
// : "HELLO BOB"
</pre>
</td>
</tr>
</table>
