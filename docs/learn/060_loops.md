---
sort_order: 60
previous: 050_functions
---

# Loops

## Multi-threaded

### Map

```
Map(array) {
    // ...
    // `id` is the iteration
    // `it` is the array element
    // Expects: value
    // Returns: array
}
```

### Compute

```
Compute(where, data) {
    // ...
    // `id` is the iteration
    // `it` is the iteration
    // Expects: value
    // Returns: ComputeResult
}
```

### Filter

```
Filter(array) {
    // ...
    // `id` is the iteration
    // `it` is the array element
    // Expects: condition
    // Returns: array
}
```

## Single-threaded

### Loop

```
Loop(times) {
    // ...
    // `id` is the iteration
    // `it` is the iteration
}

Loop(array) {
    // ...
    // `id` is the iteration
    // `it` is the array element
}

Loop {
    // ...
    // `id` is the iteration
    // `it` is the iteration
    // `break` to exit
}
```

### Reduce

```
Reduce(array, start) {
    // ...
    // `id` is the iteration
    // `it` is the array element
    // `ac` is the accumulator
    // Expects: value
    // Returns: value
}
```