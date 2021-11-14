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
*Comming soon...*

## Expression
Expression is a way to express a value by evaluating
[literal values](#literal-value) or values refered through [keys](#key), using
[operators](#operator).

### Literal Value
Literal value is a value expressed directly. Literal values are classified to
these types:
- [String](#string)
- [Integer](#integer)
- [Float](#float)
- [Boolean](#boolean)
- [Array](#array)
- [Inline Table](#inline-table)
- [Inline Function](#inline-function)

#### String
String is an array of characters. There are 4 ways to express strings.

##### Double Quoted String
Double quoted string starts with a quatation mark (`U+0022`), ends with a not
escaped quatation mark. Any Unicode characters can be used except quatation
mark and backslash (`U+005C`), and escape sequences starts with backslashes
can be used:

- `\n` ... linefeed (`U+000A`)
- `\r` ... cariage return (`U+000D`)
- `\t` ... horizontal tab (`U+0009`)
- `\"` ... quatation mark (`U+0022`)
- `\\` ... backslash (`U+005C`)
- `\xXX` ... 8 bit character (`U+00XX`)
- `\u{XXXX}` ... unicode character (`U+XXXX`)

Note that 8 bit characters are 2-digit hex values, and unicode characters are
hex values with any digits.

```toml
string = "The \"Double quated\"\r
\tString"
```

##### Single Quoted String
Single quoted string is a string surrounded by two apostrophes (`U+0027`).
No escape sequences are allowed, so apostrophes cannot be included in single
quoted strings.

```toml
string = '\\ServerX\admin$\system32\'
```

##### Double Quoted Unidented String
Double quoted unindented string is similar to double quoted string, but
surrounded with triple quotation marks (`"""`), and will be
[unindented](https://github.com/dtolnay/indoc/tree/master/unindent).
Quotation marks can be used in the strings.

```toml
# python = "def hello():\n    print('Hello, World!')\n\nhello()"
python = """
    def hello():
        print("Hello, world!")

    hello()
"""
```

##### Single Quoted Unindented String
Single quoted unindented string is similar to single quoted string, but
surrounded with triple apostrophes (`'''`), and will be
[unindented](https://github.com/dtolnay/indoc/tree/master/unindent).
Apostrophes can be used in the strings.

```toml
# python = "def hello():\n    print('Hello, World!')\n\nhello()"
python = '''
    def hello():
        print('Hello, world!')

    hello()
'''
```

#### Integer
Integer is a whole number. Decimals, hexadecimals (with prefix `0x`), octals 
(with prefix `0o`), and binaries (with prefix `0b`) are supported. In
hexadecimals, numbers from ten to fifty are expressed by `A-F` or `a-f`.
Leading zeros are not allowed.

```toml
decimal = 42
hex1 = 0xDEADBEEF
hex2 = 0xcafebabe
oct = 0o644
bin = 0b11010110
```

An integer prefixed with `+` will be treated as positive number, one prefixed
with `-` will be treated as negative number. If an integer does not have
neither `+` or `-`, it will be a positive number. `+0`, and `-0` are identical
to `0`.

```toml
pos1 = 42
pos1 = +1
zero1 = 0
zero2 = -0
neg1 = -5
neg2 = -0xcafebabe
```

Underscores between digits are allowed for readability.

```toml
int1 = 5_349_221
int1 = 1_2_3_4_5
int2 = 0b1101_0110
```

Accepted range is from `-2^63` to `2^63-1` (64bit signed integer).

#### Float
Float is a IEEE 754 binary64 value.

A float consists of integer part, fractional part and exponent part. An
integer part is required, and follows same rule as decimal [integer](#integer)
. A fractional part is prefixed with a full stop (`U+002E`), and consists of
one or more decimal digits. An exponent part is prefixed with `e` or `E`, and
consists of an integer, which follows same rule as decimal [integer](#integer)
but leading zeros are allowed. Either a fractional part or an exponent part
are required.

A float will be expressed by `(i + f) * (10 ** e)` where `i` is an integer
part, `f` is a fractional part prefixed with `0.` (if eliminated, `f` is 0),
and `e` is an exponent part (if eliminated, `e` is 1).

```toml
float1 = 3.14
float2 = +1.23           # same as 1.23
float3 = -0.001
float4 = 3e2             # same as 300.0
float5 = 1e-02           # same as 0.01
float6 = -2E+4           # same as -20000.0
float7 = 5_000.000_003   # same as 5000.000003
```

#### Boolean
Boolean is a value, either `true` or `false`.

```toml
bool1 = true
bool2 = false
```

#### Array

#### Inline Table

#### Inline Function
*Comming soon...*

### Key

#### Function Key
*Comming soon...*

### Operators
*Comming soon...*

#### Arithmetic Operators
*Comming soon...*

#### Logical Operators
*Comming soon...*

#### Comparison Operators
*Comming soon...*

#### Function Call
*Comming soon...*

## The terms
- "Whitespace" means tab (`U+0009`) or space (`U+0020`).
- "Newline" means line feed (`U+000A`) or carriage return (`U+000D`).
- "Parenthesis" means left and right of round brackets (`()`), curly brackets
  (`{}`), or square brackets (`[]`).
- "Table" means a collection consists of key/value pairs, also known as
  "dictionary" or "hash table".

## ABNF Grammar
*Comming soon...*
