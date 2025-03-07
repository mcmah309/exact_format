// In tests/test_exact_format.rs

use exact_format::exact_format;

#[test]
fn test_basic_replacement() {
    let result = exact_format!("Hello {name}", "{name}" => "World");
    assert_eq!(result, "Hello World");
}

#[test]
fn test_no_replacements() {
    let result = exact_format!("Hello World", "NotFound" => "Replacement");
    assert_eq!(result, "Hello World");
}

#[test]
fn test_multiple_replacements() {
    let result = exact_format!("Hello {first} {last}",
                              "{first}" => "John",
                              "{last}" => "Doe");
    assert_eq!(result, "Hello John Doe");
}

#[test]
fn test_multiple_occurrences() {
    let result = exact_format!("{key} and {key} again", "{key}" => "value");
    assert_eq!(result, "value and value again");
}

#[test]
fn test_nested_replacements() {
    // First replace table, then id
    let result = exact_format!("SELECT * FROM TABLE WHERE id = ID",
                               "TABLE" => "users",
                               "ID" => "42");
    assert_eq!(result, "SELECT * FROM users WHERE id = 42");
}

#[test]
fn test_overlapping_keys() {
    // The key "Hello" contains "He", test that replacement order matters
    let result = exact_format!("Hello World",
                              "He" => "Hi",
                              "Hello" => "Greetings");
    // We find "He" first, so replace that
    assert_eq!(result, "Hillo World");

    // Opposite order: explicitly look for longer matches first
    let result = exact_format!("Hello World",
                              "Hello" => "Greetings",
                              "He" => "Hi");
    assert_eq!(result, "Greetings World");
}

#[test]
fn test_empty_replacements() {
    let result = exact_format!("Hello {name}", "{name}" => "");
    assert_eq!(result, "Hello ");
}

#[test]
fn test_expression_as_value() {
    let number = 42;
    let result = exact_format!("The answer is {answer}", "{answer}" => number.to_string());
    assert_eq!(result, "The answer is 42");
}

#[test]
fn test_escape_curly_braces() {
    // The original string has literal curly braces
    let result = exact_format!("Function call: function({param})", "{param}" => "value");
    assert_eq!(result, "Function call: function(value)");
}

#[test]
fn test_javascript_style_interpolation() {
    let user_id = 42;
    let user_name = "John";
    let result = exact_format!("const user = { id: USERID, name: 'USERNAME' };",
                               "USERID" => user_id.to_string(),
                               "USERNAME" => user_name);
    assert_eq!(result, "const user = { id: 42, name: 'John' };");

    // This test verifies that the expanded code is equivalent to:
    // format!("{}{}{}{}{}",
    //         "const user = { id: ",
    //         user_id.to_string(),
    //         ", name: '",
    //         user_name,
    //         "' };")
}

#[test]
fn test_replacement_order() {
    // Ensure we find the leftmost match first
    let result = exact_format!("abc abc", "a" => "X", "b" => "Y");
    assert_eq!(result, "XYc XYc");
}

#[test]
fn test_empty_string() {
    let result = exact_format!("", "anything" => "something");
    assert_eq!(result, "");
}

#[test]
fn test_character_by_character() {
    let result = exact_format!("abc", "a" => "1", "b" => "2", "c" => "3");
    assert_eq!(result, "123");
}

#[test]
fn test_positional_search() {
    // This test verifies that we scan from left to right for each replacement
    let result = exact_format!("Hello World", "o" => "X");
    assert_eq!(result, "HellX WXrld");
}
