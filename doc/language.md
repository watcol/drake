# Walnut v0.1.0-pre

This book is the reference for the Walnut configuration language, describes
syntax and semantics.

## File Format
- A Walnut file must be encoded in UTF-8.
- A Walnut file is described as an sequence of [statements](#statement), and
  expresses a table whose keys and values are described in
  [key/expression pair](#keyexpression-pair) statements.
- A Walnut file should use the extension `.wal`.
- The appropriate MIME type for Walnut files is `application/walnut`.

## Statement
Statement is a base unit of Walnut, categorized into these types:
- [Empty Statement](#empty-statement)
- [Key/Expression Pair](#keyexpression-pair)
- [Table Header](#table-header)
- [Function Definition](#function-definition)
- [Import Declaration](#import-declaration)

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

### Table Header
Table Header is a statement to declare the beginning of table, formed by a
[key](#key) surrounded by a pair of [square brackets](#the-terms).
[Whitespaces](#the-terms) around the brackets are ignored.

[Key/Expression pairs](#keyexpression-pair) below headers are regarded as
elements of an [table](#the-terms) indicated by the [key](#key), until the
next table header or the end of file.

```toml
[table1]
key = "value"

  [
  table2
  ]
```

Overwriting exsiting tables are prohibited.

```toml
table = { foo = "foo" }

[table] # Error!
```

But an expression after the closing bracket is used as an initial table and
values are freely overwrited or appended.

```toml
# `table` is `{ foo = "baz", bar = "baz", baz = 1 }`.
[table] { foo = "foo", bar = "bar" }
bar = "baz"   # Overwriting a value.
baz = 1       # Appending a value.

 # It is useful when overwriting an imported table.
[dependencies] import("dependencies.wal")
```

#### Array of Tables
Table header with double square brackets expresses that the value indicated by
the key is an array and the following key/expression pairs form a table which
is an element of the array. The table below the first header is the first
element, and headers with same key indicates following elements.

```toml
[[users]]
name = "Alice"
id = 42362465

[[users]]   # Empty table

[[users]]
name = "Bob"
id = 63328257

# Equivalent to:
users = [
  { name = "Alice", id = 42362465 },
  {},
  { name = "Bob", id = 63328257 },
]
```

Initial arrays of tables can be assigned by putting an expression after the
last closing bracket of the first element. Following tables will be appended
to the initial array.

```toml
# `array` is `[ { foo = "foo" }, { bar = "bar" }, { baz = "baz" } ]`.
[[array]] [{ foo = "foo" }]
bar = "bar"

[[array]]
baz = "baz"
```

### Function Definition

## Expression

### String

### Integer

### Float

### Boolean

### Array

### Inline Table

### Inline Function

### Key

### Operators

#### Arithmetic Operators

#### Logical Operators

#### Comparison Operators

### Function

#### Function Call

## The terms
- "Whitespace" means tab (`U+0009`) or space (`U+0020`).
- "Newline" means line feed (`U+000A`) or carriage return (`U+000D`).
- "Parenthesis" means left and right of round brackets (`()`), curly brackets
  (`{}`), or square brackets (`[]`).
- "Table" means a collection consists of key/value pairs, also known as
  "dictionary" or "hash table".

## ABNF Grammar
*Comming soon...*
