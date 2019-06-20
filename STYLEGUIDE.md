# Kevin Styleguide

By example

## Naming

| thing | style |
| --- | --- |
| modules | `UpperCamelCase` |
| functions | `UpperCamelCase` |
| boxes | `UpperCamelCase` |
| box fields | `snake_case` |
| variables | `snake_case` |
| built-ins | `lowercase` |
| constants | `BIG_SNAKE_CASE` |

### Indentation

Indent everything between `{` and `}` with 2 spaces.

```
{
  // ...
  // ...
}
```

### Operator spacing

Math operators should have 1 space before and 1 space after.

```v = a + b * c```

### Functions

```
fn Type Name(ArgType arg_name_1, ArgType arg_name_2) {
  // ...
}
```

### Boxes

```
box Name {
  Type variable_1;
  Type variable_2;
  
  (Type value_1, Type value_2) {
    // ...
  }
}
```
