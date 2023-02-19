# Anatomy of Denim Syntax

```
TODO(skeswa): build out a tree here based on Rust syntax.
```

Denim is _primarily_ an expression language. This means that most forms of
value-producing or effect-causing evaluation are directed by the uniform syntax
category of _expressions_. Each kind of expression can typically _nest_ within
each other kind of expression, and rules for evaluation of expressions involve
specifying both the value produced by the expression and the order in which its
sub-expressions are themselves evaluated.

In contrast, statements serve _mostly_ to contain and explicitly sequence
expression evaluation.

## Statements and Expressions

- Statements
  - **Declaration statement**\
    A _declaration statement_ is one that introduces one or more _names_ into the
    enclosing statement block.
    - **Item declaration statement**\
      An _item declaration statement_ has a syntactic form identical to an [item declaration](#items)
      within a package.
    - **`let` statement**\
      A _`let` statement_ introduces a new set of [variables], given by a [pattern].
      ```rust
      let u = "Hello World!";
      let (v!, w) = ([1, 2, 3], 42); // The bindings may be mutable (!)
      let {end: {x!, y}} = Line {
        end: Point { x: 2.0, y: 4.0 },
        start: Point { x: 1.0, y: 2.0 },
      };
      ```
  - **Expression statement**\
    An _expression statement_ is one that evaluates an [expression] and ignores its
    result.
    - **Expression with block**\
      An expression that consists of only a [block expression][block] or control flow expression, if used in a context where a statement is permitted, can omit the trailing semicolon. This can cause an ambiguity between it being parsed as a standalone statement and as a part of another expression; in this case, it is parsed as a statement. The type of [_ExpressionWithBlock_][expression] expressions when used as statements must be `void`.
      ```rust
      let mut v = vec![1, 2, 3];
      v.pop();          // Ignore the element returned from pop
      if v.is_empty() {
          v.push(5);
      } else {
          v.remove(0);
      }                 // Semicolon can be omitted.
      [1];              // Separate expression statement, not an indexing expression.
      ```

## Items

An _item_ is a component of a package. There are several kinds of items:

- `from...show` declarations
- `from...use` declarations
- function definitions
- `type` definitions
- `struct` definitions
- `enum` definitions
- `union` definitions
- `trait` definitions
- implementations
- `extern` blocks
