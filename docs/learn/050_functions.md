---
sort_order: 50
previous: 040_boxes
next: 060_loops
---

# Functions

### Declarations

```
fn Add(a: Vector, b: Vector): Vector {
  Vector(a.x + b.x, a.y + b.y)
}
```

### Chains

```
import { Trim, AddHello, UpperCase } from "./string_functions"

// These are the same:
let my_str = "  Bob  " -> Trim -> AddHello -> Uppercase
let my_str = Uppercase(AddHello(Trim("  Bob  ")))

// my_str : "HELLO BOB"
```
