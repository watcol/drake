# Walnut v0.1.0-pre

This book is the specification for the Walnut configuration language that
speifies syntax and semantics. Parser and interpreter implementations must
follow this specification.

## The terms
- "Whitespace" means tab (`U+0009`) or space (`U+0020`).
- "Newline" means line feed (`U+000A`) or carriage return (`U+000D`).
- "Parenthesis" means left and right of round brackets (`()`), curly brackets
  (`{}`), or square brackts (`[]`).

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
Line is the string between two [newline](#the-term) characters (or the
beginning of a file), expcept newlines escaped by `\`, newlines surrounded by
[parentheses](#the-terms) or newlines in [strings](#string). The former two
cases are treated as a [whitespace](#the-term), but in the latter case, the
behavior is decided by the kind of string. See also:
[Expression/String](#string)

[Whitespaces](#the-terms) in the beginning or the end of a line and a
[comment](#comment) in the middle, or the end of a line will be ignored (= is
equivalent to empty text). Therefore, a comment in the middle of a line will
be interpreted as a whitespace, and one in the end of a line will be ignored.

### Comment
Comment is a text starts with hash symbol (`#`, `U+0023`) outside a
[string](#string), (inside [parentheses](#the-term) are allowed) and
continues until the appearance of a [newline](#the-term) character (not
included). Newline characters are not permitted in comments.

```
# This is an comment
key = "value" # This is also a comment
key2 = "# This is not a comment"
```

## ABNF Grammar
*Comming soon...*
