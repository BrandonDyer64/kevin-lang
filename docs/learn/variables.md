---
sort_order: 30
previous: basic_types
next: boxes
---

# Variables

```
let a = 1;

// With a specific type
let i16 a = 1;

// Changeable
var x;

// Mutable
let mut x;

// Both
var mut x;
```

### Changeability

The changeability of a variable describes if a variable can be assigned a new value.

```
let a = 1;
a = 2; //  ×  |  a cannot be redefined

var a = 1;
a = 2; //  ✓  |  a can be redefined
```

### Mutability

The mutability of a variable describes if the contents within a variable can change.

#### Immutable

```
let vector = Vector2(1.2, 2.3);
vector.x = 4;          //  ×  |  The value inside of vector cannot change
```

#### Mutable

```
let mut vector = Vector2(1.2, 2.3);
vector.x = 6.8;        //  ✓  |  The value inside of vector can change
```

#### Watch

Assuming `vector` is mutable, the value inside of vector can change, but the
value can't be changed by the watching variable.

```
let watch vector_watching = vector;
vector.y = 4.2;        //  ✓  |  can change
vector_watching.y = 3; //  ×  |  cannot change
```