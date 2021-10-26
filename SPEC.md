# Walnut v0.1.0-pre

This book is the specification for the Walnut configuration language that
speifies syntax and semantics. Parser and interpreter implementations must
follow this specification.

## Table of Contents
- [File Format](#file-format)
- [Sentence](#sentence)
  - [Comment](#comment)
  - [Empty Sentence](#empty-sentence)
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
  - Function Call
- [The terms](#the-terms)
- [ABNF Grammar](#abnf-grammar)

## File Format
- A Walnut file must be encoded in UTF-8.
- A Walnut file is described as an sequence of [sentences](#sentence).
- A Walnut file should use the extension `.wal`.
- The appropriate MIME type for Walnut files is `application/walnut`.

## Sentence
Sentence is a base unit of Walnut, categorized into these types:
- [Empty Sentence](#empty-sentence)
- Key/Expression Pair
- Table Header
- Function Definition
- Import Declaration

Sentences basically consist of one line (separated with [newline](#the-terms)
characters), except these cases:
- Sentence with a line ends with `\` (without considering
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
line = "foo" # This is a line.
```

### Comment
Comment is a text starts with hash symbol (`#`, `U+0023`) outside a
[string](#string), (inside [parentheses](#the-term) are allowed) and
continues until the appearance of a [newline](#the-term) character (not
included). Newline characters are not permitted in comments.

```toml
# This is an comment
key = "value" # This is also a comment
key2 = "# This is not a comment"
```

### Empty sentence
Empty sentence is a sentence with nothing but [whitespaces](#whitespaces).
Empty sentence has no effects to the semantics.

## The terms
- "Whitespace" means tab (`U+0009`) or space (`U+0020`).
- "Newline" means line feed (`U+000A`) or carriage return (`U+000D`).
- "Parenthesis" means left and right of round brackets (`()`), curly brackets
  (`{}`), or square brackts (`[]`).

## ABNF Grammar
*Comming soon...*
