# Denim's language design takes

Sorted from hottest to coldest (more or less).

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
quick_async_operation_1();

// We wait for `quick_async_operation_1` to complete before starting
// `quick_async_operation_2` without having to use a keyword - just works.
quick_async_operation_2();

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

### Imports at the bottom of the file

This is normal:

```
import stuff up here

yada yada yada

maybe some epxorts

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


## Medium Takes

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
