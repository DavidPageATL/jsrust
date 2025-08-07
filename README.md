# JavaScript Parser in Rust

A comprehensive JavaScript parser built in Rust that can tokenize JavaScript source code and build an Abstract Syntax Tree (AST).

## Features

### Lexical Analysis (Tokenizer)
- **Literals**: Numbers, strings, booleans, null, undefined
- **Identifiers**: Variable and function names
- **Keywords**: `var`, `let`, `const`, `function`, `if`, `else`, `while`, `for`, `return`, `true`, `false`, `null`, `undefined`
- **Operators**: Arithmetic (`+`, `-`, `*`, `/`, `%`), comparison (`<`, `>`, `<=`, `>=`, `==`, `!=`, `===`, `!==`), logical (`&&`, `||`, `!`), assignment (`=`)
- **Punctuation**: Parentheses, braces, brackets, semicolons, commas, dots, colons
- **Comments**: Single-line (`//`) and multi-line (`/* */`) comments

### Syntax Analysis (Parser)
- **Variable declarations**: `var x = 42;`
- **Function declarations**: `function name(params) { body }`
- **Function expressions**: Anonymous functions in object literals
- **Control flow**: `if/else`, `while`, `for` loops
- **Expressions**: Binary operations, unary operations, function calls, member access
- **Literals**: Arrays `[1, 2, 3]`, objects `{key: value}`
- **Member access**: Dot notation (`obj.prop`) and bracket notation (`obj[prop]`)

## Project Structure

```
src/
├── main.rs          # Entry point and CLI interface
├── lexer.rs         # Tokenizer implementation
├── parser.rs        # Parser implementation
├── ast.rs           # Abstract Syntax Tree definitions
examples/
├── simple.js        # Simple JavaScript examples
├── complex.js       # Complex JavaScript examples
```

## Usage

### Building the Project
```bash
cargo build
```

### Running the Parser
```bash
# Parse a JavaScript file
cargo run examples/simple.js

# Parse any JavaScript file
cargo run path/to/your/file.js
```

### Example Output
The parser outputs:
1. **Tokens**: List of all tokens found in the source code
2. **AST**: Pretty-printed Abstract Syntax Tree

## Supported JavaScript Features

### Variables and Functions
```javascript
var x = 42;
var name = "Hello World";

function greet(name) {
    return "Hello, " + name;
}
```

### Control Flow
```javascript
if (x > 40) {
    result = result * 2;
} else {
    result = 0;
}

while (count < 5) {
    count = count + 1;
}

for (var i = 1; i <= 10; i = i + 1) {
    sum = sum + i;
}
```

### Objects and Arrays
```javascript
var numbers = [1, 2, 3, 4, 5];
var person = {
    name: "Alice",
    age: 30,
    add: function(a, b) {
        return a + b;
    }
};

var name = person.name;
var first = numbers[0];
```

### Expressions
```javascript
var result = x + y * z;
var isValid = (x > 0) && (y < 100);
var message = greet("World");
```

## Architecture

### Lexer (Tokenizer)
The lexer converts raw JavaScript source code into a stream of tokens. It handles:
- Character-by-character scanning
- Comment stripping
- String escape sequences
- Number parsing
- Keyword recognition

### Parser
The parser uses a recursive descent parsing approach to build an AST from tokens. It implements:
- Operator precedence
- Expression parsing with proper associativity
- Statement parsing
- Error recovery and reporting

### AST
The Abstract Syntax Tree represents the parsed JavaScript code in a structured format that can be:
- Pretty-printed for debugging
- Transformed for optimization
- Interpreted or compiled

## Error Handling

The parser provides detailed error messages for syntax errors:
```rust
Parse error: Expected ')' after function parameters
Parse error: Unexpected token
```

## Limitations

This is an educational parser that covers core JavaScript features but doesn't include:
- ES6+ features (arrow functions, classes, destructuring)
- Advanced operators (spread, optional chaining)
- Modules (import/export)
- Async/await
- Regular expressions
- Template literals

## Testing

Run the parser on the provided examples:
```bash
cargo run examples/simple.js    # Basic features
cargo run examples/complex.js   # Advanced features
```

## Development

To extend the parser:
1. Add new token types in `lexer.rs`
2. Update AST definitions in `ast.rs`
3. Implement parsing logic in `parser.rs`
4. Test with new JavaScript examples

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 David Page

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
