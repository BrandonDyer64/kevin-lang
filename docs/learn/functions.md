---
sort_order: 50
previous: boxes
---

# Functions

### Declarations

```
fn Vector Add(Vector a, Vector b) {
  return Vector(a.x + b.x, a.y + b.y);
}
```

### Chains

```
import { Trim, AddHello, UpperCase } from "./string_functions"

// These are the same:
let my_str = "  Bob  "->Trim()->AddHello()->Uppercase();
let my_str = Uppercase(AddHello(Trim("  Bob  ")));

// my_str : "HELLO BOB"
```