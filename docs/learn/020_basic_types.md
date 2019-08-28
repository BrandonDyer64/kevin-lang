---
sort_order: 20
previous: 010_hello_world
next: 030_variables
---

# Basic Types

## Literals

| integer | `1` `2` `3` |
| float | `1.2` `2.3` `4.5` |
| character | `'a'` `'b'` `'c'` |
| string | `"abcdefg"` |
| boolean | `true` `false` |
| symbol | `:abc` `:defg` `:hi` |

## Primitives

| integer | `i1` `i2` `i4` `i8` `i16` `i32` `i64` `i128` `i256` ... |
| unsigned | `u1` `u2` `u4` `u8` `u16` `u32` `u64` `u128` `u256` ... |
| float | `f32` `f64` |
| char | `char` |
| boolean | `bool` |
| symbol | `symbol` |

## Compounds

### Arrays

```
// All of these make the same array
let array = [1, 2, 3]
let array = [i32; 3] { it }
let array = [
    , 1
    , 2
    , 3
]
```

```
              ┌──────────── array type
              │   ┌──────── array length
              │   │    ┌─── iterator index
let array = [i32; 3] { it }
```

An inner array

```
let mut outer: i32[] = [1, 2, 3, 4, 5]
let mut inner: i32[] = outer[1; 3]

// outer : 1, 2, 3, 4, 5
// inner :    2, 3, 4

outer[2] = 8;

// outer : 1, 2, 8, 4, 5
// inner :    2, 8, 4

```

```
      ┌───── starting position
      │  ┌── length of inner array
outer[1; 3];
```

### Multidimensional Arrays

Multidimensional arrays are stored in contiguous memory as a single array, but
are accessed with multiple values.

#### 2D

```
let mut array: i32[,] = [i32; 4, 4] { it_x * it_y }

// array:
// 0 0 0 0
// 0 1 2 3
// 0 2 4 6
// 0 3 6 9

// Modify the array
array[2, 3] = 6
```

#### 3D

```
let mut [i32,,] array = [i32; 4, 4, 4] => it_x * it_y * it_z

// Modify the array
array[2, 3, 1] = 4
```

You can have as many dimensions as you like, but remember that each dimension
consumes exponentially more memory.

```
let [Type] array_1d     = [i32; x] { value }             array_1d[x]
let [Type,] array_2d    = [i32; x, y] { value }          array_2d[x, y]
let [Type,,] array_3d   = [i32; x, y, z] { value }       array_3d[x, y, z]
let [Type,,,] array_4d  = [i32; x, y, z, w] { value }    array_4d[x, y, z, w]
let [Type,,,,] array_5d = [i32; x, y, z, w, h] { value } array_5d[x, y, z, w, h]
...
```

### Jagged Arrays

Jagged arrays are arrays of arrays and are not stored in contiguous memory.
These are slower, and have less memory efficiency than a multidimensional array,
but have the advantage of being able to change an entire row in one instruction.

```
let mut array = [i32[]; 4] (it_outer) { [i32; 4] { it_outer * it } }
let row = [i32; 4] { it + 2 }
array[2] = row
array[1][3] = 6
```

### Tuples

```
fn GetTuple(tuple: (i32, string)): (i32, string) {
    let (num, str) = tuple
    return (num + 1, str + " pizzas")
}

let tuple = (15, "Krusty Krab")
let new_tuple = GetTuple(tuple)
let (num, str) = new_tuple
Console::Print("I have ${num} ${str}")
```

### Boxes

TODO.