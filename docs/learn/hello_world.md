---
sort_order: 10
next: basic_types
---

# Hello World

A simple program to get started.

```
import Console from "console"

fn Main(args: string[]): symbol {

    Console::Print("Hello, World!")

    return :ok
}

export Main
```

We can create a binary by running the compiler.

```
kevin hello.kev
```

This makes a `hello` binary we can run in our terminal.

```
$ ./hello
Hello, World!
```
