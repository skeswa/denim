# Esper

A programming language narrowly designed for code sharing across environments.

## Pitch

So, you know how pretty much every modern, garbage-collected language feels
eerily like the others lately? I think we can blame this phenomenon on the fact
that many of these languages are converging on the same features and concepts;
how many languages have added first-class functions, co-routines, data classes,
and language-level immutablity recently? The only _tangible_ differences between
one language and another are the ecosystems and platforms that they can unlock
for you. Go gets you into the cloud and terminal ecosystems, while JS/TS gets
you into the browser and to the edge. Swift and Kotlin get you onto phones, and
with C# you can ship on an Xbox.

And it got me thinking: **if the languages we use to write our apps are this
similar, why on earth are we writing the same logic over and over again?** Why
can't we write most of our logic, constants, and types once, and use them
anywhere? What if there was a language purely designed to be interoperable with
other languages?

Esper is that language.

The intent behind Esper is to incorporate the smallest set of common features
from these garbage-collected languages sufficient to:

- Create common types
- Implement business logic
- Declare common constants

Of course, it wouldn't hurt to end up with a language that is pleasant to use
and maintain while we're at it.

## Design

Esper is not designed to be fast, or sexy, or interesting, or well-suited for
any specific domain. It should fit right into the source coce powering any ol'
user interface, backend API, and smart fridge. Esper's guiding design
principles, are to be:

- Simple,
- Familar, and
- Practical

Esper should never feel as esoteric and ornate as Rust, but it should feel a
smidge more expressive than Go. It should be easy to read, follow, and document
like Java; getting out of your way and letting you solve the damn problem like
Node.js.

Described above is language that will be difficult to design, and even harder to
implement. My hope in all of this, at the very least, is to move the
[Overton window](https://en.wikipedia.org/wiki/Overton_window) in a direction
that we bet the programming world would enjoy.

Wish me luck.

### Lineage

As Esper is designed to feel familiar, it borrows heavily from some popular
programming languages/runtimes:

- Dependency management from [Deno](https://deno.land/)
- Module system and batteries-included standard library championed by
  [Go](https://go.dev/)
- Syntax largely stolen from [Rust](https://www.rust-lang.org/) with a few
  tricks from [Dart](https://dart.dev/) and [Python](https://www.python.org/)
  included
- Concurrency model inspired by [Dart](https://dart.dev/)
- Testing is an hommage to [Dart](https://dart.dev/),
  [JavaScript](https://www.javascript.com/), and [Go](https://go.dev/)

### Tour

There is always where you want to start with a new language - what will Esper
look like? The answer is a lot like Rust. I just happen to think that Rust gets
a lot of stuff right. That said, expect some deviations made in the interest of
developer ergonomics and Esper's particular domain challenges.

#### Primitive types

Esper's primitives are mostly stolen from Go. It has:

- `bool`\
  Boolean value that can be `true` or `false`.
- `byte`\
  An 8-bit unsigned integer, often in strings. NOTE: in the context of a `string`,
  `byte` does not encapsulate the semantic concept of a "character" since in some
  encodings, like [UTF-8](https://developer.mozilla.org/en-US/docs/Glossary/UTF-8),
  a character may be expressed with more than one byte.
- `double`\
  A 64-bit signed floating point number.
- `int`\
  A 64-bit signed integer number.
- `rune`\
  A 32-bit unsigned integer number intended to represent the semantic concept of
  a "character" in strings.
- `string`\
  A sequence of bytes semantically associated with text.

#### Operators

Esper features the "usual suspects" for a C-family language:

- `&&`, `||`, `==`, and `!=`\
  Logical comparison operators work the way that you think they do. One thing to
  note is that the equality comparison operators do not perform qualitative comparison.
  Instead they do a dumb equality check, like Java, for instance.
- `+`, `-`, `*`, `/`, and `%`\
  Arithmetic operators can only be applied to numbers of the same kind.
- `**`, the exponentiation operator, is stolen from
  [ES2017](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Exponentiation)
- `~/`, the truncating division operator, is stolen from
  [Dart](https://api.flutter.dev/flutter/dart-core/num/operator_truncate_divide.html)

You might be wondering where the bitwise operators are - there are none! Looking
for operator overloads? You won't find them here.

Good riddance.

#### Variables

Esper steals variable declaration from Rust.

```rust
let abc = 123;
```

Like in Rust, Esper's `let` creates immutable variables by default. This means
that `abc` cannot by be assigned a new value.

```rust
let abc = 123;
abc = 321; // Compile-time error
```

To create a mutable variable, you need to use the `mut` keyword too.

```rust
let mut xyz = 123;
xyz = 456; // 👍
```

Importantly, Esper does not have a notion of `const`. Instead `let` is also used
to declare constants at the top-level lexical scope,

#### Comments

Comments are almost purely stolen from Rust.

```rust
// This is a simple line comment.

let abc /* this is an inline comment */ = /* stick these anywhere */ 123;

/// This is a doc comment.
///
/// These comments are intended to document your code in greater detail. To
/// facilitate this greater detail, these kinds of comments have:
/// 1. **full** _Markdown_ `support`
/// 2. Dart-style `[]` code links
///    For example, [abc] references the variable created above explicitly.
let forty_two = 42;
```

#### Expressions vs. statements

TODO(skeswa): flesh this out.

#### Functions

Syntactically, Esper functions are very similar Rust functions.

```rust
// Functions can specify a return type using the `->` symbol.
pub fn multiply_by_two(num: double) -> double {
  num * 2
}

// No need to specify if a function is `void`, just say nothing at all:
fn print_hello_world() {
  // By the way, printing works the same way it does in Dart. `print` is a
  // globally visible function that takes a `string` and outputs it to
  // console/stdout.
  print("hello world");
}
```

There is just one wrinkle with Esper functions - there are no positional
arguments, only named arguments that can appear in any order. When a function is
invoked, its arguments must be explicitly labeled at the call-site unless a
variable is passed along sharing the name of an argument. There is one exception
to this rule: if a function has just a single argument, no label is necessary.

```rust
fn multiply_by_two(num: double) -> double {
  num * 2
}

print(multiply_by_two(3)); // prints "6"

fn multiply(a: double, b: double) -> double {
  a * b
}

print(multiply(a: 2, b: 5)); // prints "10"

let b = 5;

print(multiply(a: 2, b)); // prints "10"

// Oh! And one more thing, all you have to make an argument optional is give
// it a default value:

fn i_cant_wait_to(action /* `: string` is inferred here */ = "take a nap") {
  // Now seems like a good time to mention we stole string interpolation from
  // Dart.
  print("Time to $action!");
}

print(i_cant_wait_to("eat donuts")); // prints "Time to eat donuts!"
print(i_cant_wait_to()); // prints "Time to take a nap!"

// Esper even borrows Rust's syntax for lambda expressions (Rust calls them closures):
let lambda_annotated = |i: i32| -> i32 { i + 1 };
let lambda_inferred  = |i     |          i + 1  ;
```

#### Modules

Esper's core packaging primitive is called a "module". Modules designed to
define behavior, state, and types for a particular context or domain.
Furthermore, Esper modules are intended to depend on each other through the
import and export of behavior, state, and types.

Esper modules are expressed a file system as a directory with Esper source
files. Each source file in an Esper module has full visibility of everything
declared in the other source files of the module. Additionally, each source file
can do its own importing and exporting. Esper source files can only import stuff
exported by another Esper module. It might help to think of this style of
packaging is a blend of
[Go packages](https://www.golang-book.com/books/intro/11) and
[ES modules](https://hacks.mozilla.org/2018/03/es-modules-a-cartoon-deep-dive/).

```rust
// Like Deno and Go, remote modules are fetched from source control with an
// accompanying version. Version is typically inferred from a tag or reference
// on the imported module repository.
from "github.com/abc/xyz@v0.9.0" use { hello, world };
// `as` lets you alias things you import or export.
from "my.repo.com/foo/bar@v1.2.0-beta/nested/module" use * as module;

// Relative imports are a thing too. Notice how the version isn't specified -
// this is because relatively imported modules always share the version of the
// importing module.

from "some/sub/module" use { something };
from "../bing/bang" use { boom as büm };

// You can also re-export stuff using the `show` keyword.

from "github.com/abc/xyz@v0.9.0" show { hello };
from "my.repo.com/foo/bar@v1.2.0-beta/nested/module" show *;

from "../bing/bang" show { boom as büm };
```

Like in Rust, you can export stuff from Esper modules with the `pub` keyword.
Anything not declared with a `pub` will only be visible to things in its own
module.

```rust
pub let stuff_outside_of_this_module_can_see_me = true;
```

#### Structs

You may now be wondering how more complex data structures are created and
managed in Esper. I'm sure you are _so_ shocked to find out that we (mostly)
stole Rust syntax here too.

```rust
// We can make some or even all of the `struct` or its fields visible to
// external modules using the `pub` keyword.
pub struct User {
  /// You can put doc comments on fields like this.
  active: bool;
  coolness_rating: int;
  pub name: string;
}

// We can make some or even all of the `struct` or its fields public using the
// `pub` keyword.

pub struct Company {
  pub name: string;
  pub phone_number: string;
  cash_in_the_bank: double;
}
```

You may notice that we opted to go with `;` to terminate field declarations
instead of `,`. This is mostly to make adding methods to structs feel more
natural. That's right! Esper sugarifies Rust's default `impl` by simply allowing
you to declare methods within the `struct` itself. This should feel familiar to
anyone coming from a language that makes heavy use of classes.

```rust
struct Car {
  make: string;
  model: string;
  owner: User;

  // `self` is a special argument. Like in Rust, it means that this method is
  // attached to any instance of `Car`.
  fn drive(self: Car) {
    // In Esper, strings can be concatenated together just by placing string
    // literals adjacent to each other. Esper borrows this syntax from Dart.
    print(
      "A ${self.model} ${self.model} owned "
      "by ${self.owner.name} is driving",
    );
  }
}
```

We can instantiate and use `struct` like this:

```rust
let some_user = User {
  active: true,
  coolness_rating: 42,
  name: "Some User",
};

let my_car = Car {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

my_car.drive(); // Prints "A Mazda Miata owned by Some User is driving".
```

Esper has some syntactic sugar to make this a little smoother:

```rust
let my_other_car = Car {
  make: "Nissan",
  model: "Altima",
  owner: {
    active: true,
    coolness_rating: 42,
    name: "Some User",
  },
};

my_car.drive(); // Prints "A Nissan Altima owned by Some User is driving".
```

One important thing to note here is that Esper structs, like most Esper data
structures are immutable by default. So, direct imperative mutation of Esper
structs won't work in most situations.

```rust
let my_car = Car {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

my_car.make = "Toyota"; // Compile time error.
```

To "change" an immutable value, we have to first clone it first as a mutable
value.

```rust
let mut my_mut_car = my_car.mut;

my_mut_car.make = "Toyota"; // This is a-ok.

// Luckily, Esper has some syntactic sugar to make this look a little cleaner:

let my_first_car = my_car.mut {
  make: "Toyota",
  model: "Camry",
  year: 2008,
};

// Notice that `my_first_car` is mutable, but you may not always want this
// side-effect.

my_first_car.make = "Ferrari"; // This isn't quite right, but is not an error.

// Often times, mutation is needs to happen deeper in the the struct. Esper
// allows for this use case with some more syntax sugar.

let my_next_car = my_immutable_first_car.mut {
  make: "Rivian",
  model: "R1T",
  user: mut {
    coolness_rating: self.coolness_rating + 1,
  },
  year: 2023,
};

// Sometimes, you just gotta mutate structs directly. This is fairly simple to
// do in Esper. All you have to do in order to create a mutable `struct` is use
// the `mut` at creation time:

struct Donut {
  is_tasty: bool;
}

let disappointing = mut Donut { is_tasty: false };

disappointing.is_tasty = true; // This is a-ok.

```

#### Module system

As mentioned above, Esper modules work a lot like Go modules. Each directory,
and all of the source files within it, act as a single module. This means that
all source files in the same directory act sort of like one big source file.
Additionally, from outside of an Esper module, there is not visibility into
anything lacking a `pub` keyword.

TODO(skeswa): continue noodling

## Prototype

I think it might be a good idea to check out something like
[lalrpop](http://lalrpop.github.io/lalrpop/).
