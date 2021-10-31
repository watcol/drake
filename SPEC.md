# Walnut v0.1.0-pre

This book is the specification for the Walnut configuration language that
speifies syntax and semantics. Parser and interpreter implementations must
follow this specification.

## Table of Contents
- [File Format](#file-format)
- [Statement](#statement)
  - [Comment](#comment)
  - [Empty Statement](#empty-statement)
  - [Key/Expression Pair](#key-expression-pair)
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
  - Function Call
- [The terms](#the-terms)
- [ABNF Grammar](#abnf-grammar)

## File Format
- A Walnut file must be encoded in UTF-8.
- A Walnut file is described as an sequence of [statements](#statement), and
  expresses a table whose keys and values are described in
  [key/expression pair](#key-expression-pair) statements.
- A Walnut file should use the extension `.wal`.
- The appropriate MIME type for Walnut files is `application/walnut`.

## Statement
Statement is a base unit of Walnut, categorized into these types:
- [Empty Statement](#empty-statement)
- [Key/Expression Pair](#key-expression-pair)
- Table Header
- Function Definition
- Import Declaration

Statements basically consist of one line (separated with [newline](#the-terms)
characters), except these cases:
- Statement with a line ends with `\` (without considering
  [whitespaces](#the-terms) or [comments](#comment)), will be continued to the
  next line (and `\` will be ignored).
- Newline characters inside [parentheses](#the-terms) will be ignored.
- Newline characters inside [strings](#string) will be treated by the specific
  way, determined by the kind of strings, mentioned in the section
  [Expression/String](#string).

Note that [whitespaces](#the-terms) in the beginning or the end of a line and
a [comment](#comment) in the end of a line will be ignored (= is equivalent to
an empty text).

```toml
# All of these are statements.
stmt = "foo"
  stmt2 = "bar"
stmt3 =
  "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do" + \
  "eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad" + \
  "minim veniam, quis nostrud exercitation ullamco laboris nisi ut" + \
  "aliquip ex ea commodo consequat. Duis aute irure dolor in" + \
  "reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla" + \
  "pariatur. Excepteur sint occaecat cupidatat non proident, sunt in" + \
  "culpa qui officia deserunt mollit anim id est laborum."
stmt4 = [
  "elem1",
  "elem2"
]
```

### Comment
Comment is a text starts with hash symbol (`#`, `U+0023`) outside a
[string](#string), (inside [parentheses](#the-terms) are allowed) and
continues until the appearance of a [newline](#the-terms) character (not
included). Newline characters are not permitted in comments.

```toml
# This is an comment
key = "value" # This is also a comment
key2 = "# This is not a comment"
```

### Empty statement
Empty statement is a statement with nothing but [whitespaces](#the-terms).
Empty statement has no effects to the semantics.

### Key/Expression Pair
Key/Expression pair is a statement registers a value to a key. [Keys](#key)
are on the left of the equals sign (`=`, `U+003D`), and
[expressions](#expression) are on the right. [Whitespaces](#the-terms) around
the euqals sign are ignored.

```toml
key = "expression"
```

## The terms
- "Whitespace" means tab (`U+0009`) or space (`U+0020`).
- "Newline" means line feed (`U+000A`) or carriage return (`U+000D`).
- "Parenthesis" means left and right of round brackets (`()`), curly brackets
  (`{}`), or square brackts (`[]`).

## ABNF Grammar
*Comming soon...*
