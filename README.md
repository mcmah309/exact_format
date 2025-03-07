# exact_format

A procedural macro for compile time string replacement without using the standard format placeholder syntax (`{}`). This is most useful when dealing with strings that contain `{ }` blocks you do no wish to interpolate e.g. writing javascript.

## Usage

The `exact_format!` macro allows you to replace exact string matches within a template:

```rust
use exact_format::exact_format;

// Basic replacement
let result = exact_format!("Hello {name}", "{name}" => "World");
assert_eq!(result, "Hello World");

// JavaScript-style template string replacement
let user_id = 42;
let user_name = "John";
let result = exact_format!("const user = { id: USERID, name: 'USERNAME' };",
                         "USERID" => user_id.to_string(),
                         "USERNAME" => user_name);
assert_eq!(result, "const user = { id: 42, name: 'John' };");
```

## How It Works

The macro expands to a `format!` call that handles the replacements. For example:

```rust,ignore
exact_format!("const user = { id: USERID, name: 'USERNAME' };",
             "USERID" => user_id.to_string(),
             "USERNAME" => user_name);
```

Expands to something like:

```rust,ignore
{
    let __value_0 = user_id.to_string();
    let __value_1 = user_name;
    format!("{}{}{}{}{}",
            "const user = { id: ",
            __value0__,
            ", name: '",
            __value1__,
            "' };"
        )
}
```

## Rules

1. The first argument must be a string literal
2. Each replacement key must be a string literal
3. Replacement values can be any expression that can be formatted with `{}`
4. The order of replacements matters when keys overlap