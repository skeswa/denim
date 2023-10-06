# Denim's Language Design Takes

Programming language design opinions sorted more or less from hottest (most
unique/surprising) to coldest (Least unique/surprising).

## Hot Takes

### Concurrency

In a typical concurrency regime, like `async/await`, programs can "opt in" to
asynchrony in specific spots. Additionally, special functionality like
`Promise.all(...)` is necessary to express the idea that an operation should
wait for multiple prior operations to complete before it can begin.

Denim takes a very different approach: in Denim, concurrency is a first-class
syntactic concern.

Firstly, all operations in Denim are "awaited" automatically. When you don't
need to wait for something to complete, you can let in complete without blocking
using the `async` keyword.

```rust
quick_operation_1();

// We wait for `quick_operation_1` to complete before starting
// `quick_operation_2` without having to use a keyword - just works.
quick_operation_2();

let eventual_value = super_slow_operation().async;
thing_that_needs_to_happen_right_away();

print("it's ready now: ${eventual_value.await}");
```

Secondly, and perhaps most importantly, Denim allows the programmer to
ergonomically stipulate which things should happen at the same time, and which
should happen in sequence.

Statements separated a blank line are executed in sequence.

```rust
let greeting = "hello";

// Performs a network call to figure out what planet we are on.
let planet = get_current_planet();

// Waits for a 3 second time to elapse.
wait_for_3_seconds();

// Nothing is printed to the screen until 3 seconds **after** `planet` is
// fetched.
print("$greeting, $planet!");
```

Statements **not** separated by a blank line are executed concurrently.

```rust
let greeting = "hello";
let planet = get_current_planet();
wait_for_3_seconds();

// We print the moment all 3 of the statements in the previous group complete.
print("$greeting, $planet!");
```

The result is a terse yet readable way to sequence asynchronous logic.

### 2+ function parameters must be explicitly labeled

Denim requires that parameters are labeled when 2 or more parameters are
included in a function invocation.

```rust
fn add(a: int, b = 0) -> int {
  a + b
}

add(1) // compiles

add(1, 2)       // does not compile
add(a: 1, b: 2) // compiles
```

When all you need is positional arguments, consider a tuple or array.

```rust
fn add(nums: (int, int?)) -> int {
  nums.0 + (nums.1 ?? 0)
}

add((1, 2)) // compiles
```

### Fluency

In many lanuages, lots of the good stuff is only usable when invoking a function
as a member of a thing. Take for instance `?.` in JavaScript.

```ts
const foo = Math.random() > 0.5 ? { bar() {} } : null;

foo?.doSomethingMaybe(); // this is dope

function baz(foo) {
  // ...
}

// I need this grossness to **safely** call `foo`:
if (foo) {
  baz(foo);
}
```

This seems silly. `?.` is one example of the fact that we humans love to create
sequential, causal chains of things to do. Breaking these chains with control
flow like `if` does not feel good.

Denim is designed to make "chaining" operations as ergonomic. In a sense, Denim
takes Rust's `.await` syntactic concept to its logical conclusion: let's make
everything that can be used as a "prefix" usable as a "suffix".

```rs
fn baz(foo: Foo, scalar = 1.0) -> double {
  // ...
  scalar * 123
}

foo.baz(foo: it, scalar: 2.0).if it < 200 {
  print("it is $it!")
}
```

Denim accomplishes a language feature called "fluency" by:

- Allowing control flow keywords to be suffixed like `.try` and `.if`
- Supporting the `it` keyword which is the value of the preceding expression in
  the "chain"

## Medium Takes

### Imports at the bottom of the file

This is normal:

```
import stuff up here

yada yada yada

maybe some exports

mhm yeah

**finally** the stuff you came here to read
```

Denim puts imports and exports a the bottom of the file _after_ the logic and
stuff.

```rust
pub type ImportantStuff = struct {
  // ...
};

---
from "github.com/some/lib" use something;

from "~/internal/lib" show something_else;

```

### No `>` or `>=` comparison operators

Why use `>` or `>=` when `<` and `<=` do trick?

### No bitwise operators whatsoever

Denim does not have bitwise and, or, zor, and not. Why? Most logic doesn't use
these operators. Logic that needs to do bitwise math should use good ol'
fashioned functions. Good riddance.

### `and` and `or` instead of `&&` and `||`

Pretty much only Python does this, but I think it reads nicely and reduces
parsing ambiguity (`||` could be the beginning of a lambda).

### Everything is an expression

Like in Rust, most things in Denim are expressions. This means they yield a
value. `;` is used to turn an expression into statement. The value of a
statement is ignored.

```rust
print(if value > 8 { "pretty big" } else { "not that big" });
```

### Spaces instead of tabs

Yeah. 2 space indent. Deal with it 😎.

## Cold Takes

### Rust-style `enum`

Denim steals Rust's enums because they are super expressive while remaining
practical and readable.

```rust
enum Take {
  Hot { is_outta_pocket: bool },
  Medium,
  Cold(temp: f32),
}
```

### Rust-style pattern matching

Denim steals Rust's pattern matching because it gets a lot right.

```rust
match number {
  // Match a single value.
  1 => print("One!"),
  // Match several values.
  2 | 3 | 5 | 7 | 11 => print("This is a prime"),
  // Match an inclusive range.
  13..=19 => print("A teen"),
  // Handle the rest of cases.
  _ => print("Ain't special"),
}

// Match is an expression too
let binary = match boolean {
  // The arms of a match must cover all the possible values
  false => 0,
  true => 1,
};
```

The only thing it was missing is being able to eaily match a single arm in a if
statement:

```rust
if thing is Some::EnumVariant {
  print("Bingo!");
}
```

### Dart-style string syntax

Dart makes declaring, concatenating, and interpolating values within strings
super easy. Denim steals (most) of this syntax. A notable exception Denim string
literals use `"` instead of `'`.

```dart
let abc = "123""xyz" // concat just by putting literals next to each other.
let multiline = """
  take
    all
      the
        space
          you
            need
""";

let a = 1;
let b = 2;

let c = "$a + $b = ${a = b}" // Use `$` for string interpolation!
```
