# Scanning

We're going to write a compiler for the Oberon-0 language, with support only for ascii
characters (Full unicode support would complicate things a lot.)

## Tokens

The first task is to scan tokens of the language.
Tokens will be either:
- "single character" contructs (like '(', '+', etc...)
- "two-characters" contructs (like '<=', ':=', etc...)
- numbers
- identifiers (a group of characters)

```rust
// @?scanner/tokens.loc
@@scanner/tokens
```

While scanning the file, we'll want to keep track of the file name, line and column number,
to be able to report errors nicely.

```rust
// @?scanner/scan.loc
@@scanner/scan
```

## Scanner

A `Scanner` struct will keep track of iterating over a file.
We'll be doing the iteration rather "manually" ; so we'll mostly have a `Chars` iterator.
Since we want to keep positions of things, we'll use a `CharIndices` iterator ;
and we'll make it `Peekable` to allow looking for more that the next chars.

The `Scanner` will live as long as the string we're scanning, so we have to introduce lifetimes.

```rust
// @?scanner/scanner.loc
@@scanner/scanner
```

## Scanning whitespace



\pagebreak