---
sort_order: 70
previous: 060_loops
---

# Nullables

```
fn DoAThing(nullable: Type?): Type? {
    let value = nullable?
    // If dereference fails the function will return `null`
}

fn DoAThing(nullable: Type?) {
    let value = nullable?
    // If dereference fails the function will return
}

fn DoAThing(nullable: Type?) {
    let value = nullable? eject Type()
    // If dereference fails `value` will be set to `Type()`
}

fn DoAThing(nullable: Type?) {
    if let value = nullable? {
        // Dereference succeeded
    } else {
        // Dereference failed
    }
    // If dereference fails the condition will be considered falsy
}

fn DoAThing(nullable: Type?): Type eject Type() {
    let value = nullable?
    // If dereference fails the function will return `Type()`
}

fn DoAThing(nullable_1: Type?, nullable_2: Type?): Type {
    eject Type(1)
    let value_1 = nullable_1?
    // If dereference fails the function will return `Type(1)`

    eject Type(2) // Change ejection
    let value_2 = nullable_2?
    // if dereference fails the function will return `Type(2)`
}
```