# Walnut v0.1.0-pre

This book is the specification for the Walnut configuration language that
speifies syntax and semantics. Parser and interpreter implementations must
follow this specification.

Note that I used [TOML v1.0.0](https://toml.io/en/v1.0.0) as a sample to write
this specification. I'm grateful to the authors and contributors.

## The terms
- "Whitespace" means tab (`U+0009`) or space (`U+0020`).
- "Newline" means line feed (`U+000A`) or carriage return (`U+000D`).

## Table of Contents
- File Format
- Line
  - Comment
  - Empty Line
  - Key/Expression Pair
  - Table Header
    - Array of Tables
  - Function Definition
  - Import Declaration
- Expression
  - String
  - Integer
  - Float
  - Boolean
  - Array
  - Inline Table
  - Inline Function
  - Key
  - Function
  - Operators
    - Arithmetic Operators
    - Logical Operators
    - Comparison Operators
    - If Operator
  - Function Call
- ABNF Grammar

## File Format
- A Walnut file must be encoded in UTF-8.
- A Walnut file is described as an sequence of [lines](#line).
- A Walnut file should use the extension `.wal`.
- The appropriate MIME type for Walnut files is `application/walnut`.

## Line
### Comment
Comment is an string starts with hash symbol (`#`, `U+0023`) outside a string,
and ends with line feed (`U+000A`) or carriage return (`U+000D`). Line feeds
and carriage returns are not permitted in comments. Comments are allowed in the end of [lines](#line) (before newline character).

```
# This is an comment
key = "value" # This is also a comment
key2 = "# This is not a comment"
```

## ABNF Grammar
*Comming soon...*
