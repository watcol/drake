# Walnut v0.0.1-pre

This book is the reference for the Walnut configuration language, describes
syntax and semantics.

## File Format
- A Walnut file must be encoded in UTF-8.
- A Walnut file is described as an sequence of [statements](#statement), and
  expresses a [table](#table) by [rendering](#terms).
- A Walnut file should use the extension `.wal`.
- The appropriate MIME type for Walnut files is `application/walnut`.

### Supported File Types
Walnut files can be transpiled to these file types:

*Comming soon...*

## Statement
Statement is a base unit of Walnut, categorized into these types:
- [Value Binding](#value-binding)
- [Table Header](#table-header)
- [Function Definition](#function-definition)

Statements basically consist of one line (separated with [newline](#terms)
characters), except these cases:
- Statement with a line ends with `\` (without considering
  [whitespaces](#terms) or [comments](#comment)), will be continued to the
  next line (and `\` will be ignored).
- Newline characters inside [parentheses](#terms) will be ignored.
- Newline characters inside [strings](#string) will be treated by the specific
  way, determined by the kind of strings, mentioned in the section
  [Expression/String](#string).
Note that [whitespaces](#terms) in the beginning or the end of a line and a
[comment](#comment) in the end of a line will be ignored (= is equivalent to
an empty text).

```toml
# All of these are statements.
stmt = "foo"
  stmt2 = "bar"
stmt3 = \
# Empty lines in line continuouses are allowed.
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
[string](#string), (inside [parentheses](#terms) are allowed) and continues
until the appearance of a [newline](#terms) character (not included). Newline
characters are not permitted in comments.

```toml
# This is an comment
key = "value" # This is also a comment
key2 = "# This is not a comment"
```

### Value Binding
Value Binding is a statement which registers key/value pairs to current
[scope](#scope) using [patterns](#pattern). Patterns are on the left of the
equals sign (`U+003D`), and [expressions](#expression) are on the right.
[Whitespaces](#terms) around the euqals sign are ignored.

```toml
pattern = "expression"
```

### Table Header
Table header is a statement to declare the beginning of [table](#table),
formed by a [pattern](#pattern) surrounded by a pair of
[square brackets](#terms). [Whitespaces](#terms) around the brackets are
ignored.

A table header stores an empty table using the pattern (except
[array](#array-destruction) and [table](#table-destruction) destructions)
using [root scope](#root-scope), and starts a new [sub scope](#sub-scope)
continues to the next table header or the end of file.

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

Instead of an empty table, an expression after the closing bracket can be used
as an initial tables.

```toml
# `table` is `{ foo = "baz", bar = "baz", baz = 1 }`.
[table] { foo = "foo", bar = "bar" }
bar = "baz"   # Overwriting a value.
baz = 1       # Appending a value.

 # It is useful when overwriting an imported table.
[dependencies] import("dependencies.wal")
```

#### Array of Tables
By using double [square brackets](#square-brackets), the header creates an
empty array to [root scope](#root-scope) (in the first header), appends a
table to the array, and starts a new [sub scope](#sub-scope).

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

In the first header, an initial array can be used instead of an empty array.

```toml
# `array` is `[ { foo = "foo" }, "str", { bar = "bar" }, { baz = "baz" } ]`.
[[array]] [{ foo = "foo" }, "str"]
bar = "bar"

[[array]]
baz = "baz"
```

### Function Definition
*Comming soon...*

#### Expression Function
*Comming soon...*

#### Table Function
*Comming soon...*

## Expression
Expression is a way to express a value by evaluating
[literal values](#literal-value) or values refered through
[keys](#key-reference), using [operators](#operator).

### Literal Value
Literal value is a value expressed directly. Literal values are classified to
these types:
- [Character](#character)
- [String](#string)
- [Integer](#integer)
- [Float](#float)
- [Boolean](#boolean)
- [Array](#array)
- [Inline Table](#inline-table)
- [Inline Function](#inline-function)

#### Character
Character is a value expresses Unicode code point. characters must be
surrounded with a apostrophes (`U+0027`). Note that characters will be
rendered as one-character [strings](#string).

```toml
char1 = 'a'
```

In apostrophes, all Unicode characters are allowed, except an apostrophe, a
backslash (`U+005C`), a line feed (`U+000A`), a carriage return (`U+000D`) and
these escape sequences are available:
- `\n` ... linefeed (`U+000A`)
- `\r` ... carriage return (`U+000D`)
- `\t` ... horizontal tab (`U+0009`)
- `\'` ... apostrophe (`U+0027`)
- `\\` ... backslash (`U+005C`)
- `\xXX` ... 8 bit character (`U+00XX`)
- `\u{XXXX}` ... unicode character (`U+XXXX`)

Note that 8 bit characters are 2-digit hex values, and unicode characters are
hex values with 2~8 digits.

```toml
char2 = '\n'
char3 = '\u{A0}'
```

#### String
String is a sequence of [characters](#character) which starts with a quatation
mark (`U+0022`) and ends with a not escaped quatation mark. Escape sequences
similar to characters are available, but instead of `\'`, `\"` can be used to
escape quotation marks. Note that a line feed, a carriage return, or a pair of
carriage return and line feed are all normalized to a line feed, and if there
is an backslash before them, they will be ignored.

```toml
string1 = "\
Normal\u{A0}Strings\n"
```

##### Raw String
Raw String is a string with no escapes, surrounded by three or more quotation
marks. Consecutive quotation marks are allowed if their number is less than
that of enclosures. Newline characters are normalized as escaped strings.

```toml
string2 = """\ServerX\admin$\system32\"""
```

#### Integer
Integer is a whole number. Decimals, hexadecimals (with prefix `0x`), octals 
(with prefix `0o`), and binaries (with prefix `0b`) are supported. In
hexadecimals, numbers from ten to fifty are expressed by `A-F` or `a-f`.
Leading zeros are not allowed in decimals.

```toml
decimal = 42
hex1 = 0xDEADBEEF
hex2 = 0xcafebabe
oct = 0o644
bin = 0b01010110
```

Underscores between digits are allowed for readability.

```toml
int1 = 5_349_221
int1 = 1_2_3_4_5
int2 = 0b1101_0110
```

Accepted range is from `-2^63` to `2^64-1` (64bit signed/unsigned integer).

#### Float
Float is a IEEE 754 binary64 value.

A float consists of integer part, fractional part and exponent part. An
integer part is required, and follows same rule as decimal
[integer](#integer). A fractional part is prefixed with a full stop
(`U+002E`), and consists of one or more decimal digits. An exponent part is
prefixed with `e` or `E`, and consists of an integer, which follows same rule
as decimal [integer](#integer) but leading zeros are allowed. Either a
fractional part or an exponent part are required.

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

Use `@inf` and `@nan` to express positive infinity and "Not a number." See
also [Built-in Key](#built-in-key).

```toml
float8 = @inf    # Positive Infinity
float9 = -@inf   # Negative Infinity
float10 = @nan   # Not a number.
```

#### Boolean
Boolean is a value, either "true" or "false". Use `@true` and `@false`
to express them. (`true` and `false` are keys.) See also
[Built-in Key](#built-in-key).

```toml
bool1 = @true
bool2 = @false
```

#### Array
Array is a collection of values. An array is surrounded by a pair of
[square brackets](#square-brackets), and values are separated with commas
(`U+002C`). A comma after the last value is allowed, and
[whitespaces](#terms) around square brackets or commas will be ignored. See
also [Array of Tables](#array-of-tables).

```toml
array1 = [1, 2, 3, 4]
array2 = []
array3 = ["foo", true, 2.3]
array4 = [
  "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod",
  "tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim",
  "veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea",
  "commodo consequat. Duis aute irure dolor in reprehenderit in voluptate",
  "velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint",
  "occaecat cupidatat non proident, sunt in culpa qui officia deserunt",
  "mollit anim id est laborum."
]
```

#### Inline Table
Inline table is another way to express a [table](#table). A inline table
consists of a list of pairs of a [bare](#bare-key) or [raw](#raw-key) key and
an [expression](#expression) with a equals sign separater, separated with
commas, surrounded by [curly brackets](#terms). A comma after the last value
is allowed, and [whitespaces](#terms) around square brackets or commas will be
ignored.

```toml
table1 = { foo = "foo", bar = "bar" }
table2 = {}
table3 = {
  string = "foo",
  integer = 42,
  float = 1.0,
  boolean = false,
  array = [0, 1, 2],
  table = { name = "Tom", id = 23235278 },
}
```

#### Inline Function
*Comming soon...*

### Key Reference
*Comming soon...*

### Operators
*Comming soon...*

#### Arithmetic Operators
*Comming soon...*

#### Logical Operators
*Comming soon...*

#### Comparison Operators
*Comming soon...*

#### Accessing Values
*Comming soon...*

#### Function Call
*Comming soon...*

## Tables and Keys
### Table
Table is a collection consists of key/value pairs, also known as "dictionary"
or "hash map". There are two types to express tables:

- [Table Header](#table-header)
- [Inline Table](#inline-table)

```toml
[table]
item1 = "item1"
item2 = "item2"

.inline_table = {item1 = "item1", item2 = "item2"}
```

### Key
Key is an identifier refers to a specific value on the table. There are five
types of keys:

- [Bare Key](#bare-key)
- [Raw Key](#raw-key)
- [Local Key](#local-key)
- [Root Key](#root-key)
- [Built-in Key](#built-in-key)

#### Bare Key
Bare key is a basic way to express key, and a bare key starts with a 
english letter (`U+0041-U+005A`, `U+0061-U+007A`), and rest characters are
consists of english letters, decimal digits (`U+0030-U+0039`), or low lines
(`U+005F`).

```toml
barekey = "This is a bare key."
bare_key2 = "Digits and underscores can be used."
```

#### Raw Key
Raw Key is a way to express key contains characters which can't express with
[bare keys](#bare-key). A raw key starts with a dollar sign (`U+0024`), and
surrounded by a pair of [curly brackets](#terms). This key can contain any key
except backslashes (`U+005C`) and right curly brackets (`U+007D`). Escape
patterns similar to [strings](#string) is available, but instead of `\"`, `\}`
can be used to escape right curly brackets, and newline characters will be
normalized as [strings](#string).

Note that bare keys and raw keys consists of same characters (for
instance `key` and `${key}`) are identical and will conflict.

```toml
${raw key} = "This is a raw key."
${\\{All\u{00A0}characters\ncan be used.\}} = true
```

#### Local Key
Local key is a [bare](#bare-key) or [raw](#raw-key) key prefixed with a low
line (`U+005F`). Local keys can't be accessed from external scope, and won't
be [rendered](#terms) by the compiler. [Whitespaces](#terms) after the `_`
prefix are ignored.

This is useful when you want to create commonly used in the file, but is not
needed to be visible from out of a scope.

```toml
base = table._base   # Not allowed
foo = table.foo      # Allowed.

[table]
_base = 5
foo = _base + 5
bar = _base + 10

[table2]
foo = ._base         # Not allowed

_${Raw Local} = "Raw Local Key"
${_Not Local} = "Normal Raw Key"
```

Refering local keys of outer scopes are allowed.

```toml
_root_local = "root"
root1 = _root_local              # Allowed
root2 = sub._sub_local           # Not allowed
root3 = sub.sub._sub_sub_local   # Not allowed

[sub]
_sub_local = "sub"
sub1 = ._root_local              # Allowed
sub2 = _sub_local                # Allowed
sub3 = sub._sub_sub_local        # Not allowed

[sub.sub]
_sub_sub_local = "subsub"
subsub1 = ._root_local           # Allowed
subsub2 = .sub._sub_local        # Allowed
subsub3 = _sub_sub_local         # Allowed
```

#### Root Key
Root key is a [bare](#bare-key), [raw](#raw-key), key prefixed with a period,
and access value from the [root scope](#root-scope) instead of current scope.
[Whitespaces](#terms) after `.` prefix are ignored.

```toml
value = "root"

[table]
value = "table"
foo = value          # "table"
bar = .value         # "root"
bar = .table.value   # "table"

.${Raw Root} = "Raw Root Key"
${.Not Root} = "Normal Raw Key"
```

#### Built-in Key
Built-in key is a key starts with a commercial at (`@`, `U+0040`), integrated
with the transpiler.

- `@output` ... A [string](#string) to specify destination path to output a
                transpiled source. Write only.
- `@type`   ... A [string](#string) to specify file type to transpile.
                Supported file types are described
                [here](#supported-file-types). Write only. (Normally infered
                from `@output` and not needed.)
- `@nan`    ... A [float](#float) expresses quiet "Not a Number". Read only.
- `@inf`    ... A [float](#float) expresses positive infinity. Read only.
- `@true`   ... A [boolean](#boolean) expresses true. Read only.
- `@false`  ... A [boolean](#boolean) expresses false. Read only.

Note that built-in keys are file-specific, and independent from the root
scope.

### Scope
Scope is a special table, used as the base point when refering keys. There are
three types of scopes:

#### Root Scope
Root scope is a scope which exists only one per a file, and used as the entry
point when [rendering](#terms) the file. Root scopes will be created as
an empty table when a file is started.

#### Sub Scope
Sub scope is a scope created by [table headers](#table-header), and continues
to the next table header or the end of file.

#### Function Scope
*Comming soon...*

## Pattern
Pattern is a way to express destinations of [value bindings](#value-binding)
or [table headers](#table-header).

### Key Pattern
Key pattern is a pattern that just stores a key to current [scope](#scope).

```toml
key = "foo"
```

### Dotted Key Pattern
*Comming soon...*

### Array Destruction
*Comming soon...*

### Table Destruction
*Comming soon...*

## Terms
- "Whitespace" means tab (`U+0009`) or space (`U+0020`).
- "Newline" means a string sequence starts with line feed (`U+000A`) or
  carriage return (`U+000D`), and contains only tabs, spaces, line feeds,
  carriage returns or comments.
- "Parenthesis" means left and right of round brackets (`()`), curly brackets
  (`{}`), or square brackets (`[]`).
- "Render" means processing and converting the walnut file to other data
  notations.

## ABNF Grammar
*Comming soon...*
