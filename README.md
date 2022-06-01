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

## Name

"Esper" is a contraction of ["Esperanto"](https://en.wikipedia.org/wiki/Esperanto).
Esperanto was originally intended to be a universal second language for
international communication. In a sense, Esper aspires to a similar goal -
just with programming languages instead.

## Design

Esper is not designed to be fast, or sexy, or interesting, or well-suited for
any specific domain. It should fit right into the source code powering any ol'
user interface, backend API, system admin script, and smart fridge. Esper's
guiding design principles, are to be maximally:

- Simple,
- Familar,
- Practical, and
- Compatible (with other languages)

Esper should never feel as esoteric and ornate as Rust, but it should feel a
smidge more expressive than Go. It should be easy to read, follow, and document
like Java, getting out of your way and letting you solve the damn problem like
Node.js.

Described above is a language that will be difficult to design, and even harder
to implement. My hope in all of this, at the very least, is to move the
[Overton window](https://en.wikipedia.org/wiki/Overton_window) in a direction
that we bet the programming world would enjoy.

Wish me luck.

### Inspiration

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

### Compatibility

For Esper to be useful, in needs to be able to interop with most of the major
languages with an established industrial presence. Esper is being developed with
the following transpilation targets in mind:

- [Dart](https://dart.dev/) for UIs
- [Go](https://go.dev/) for Cloud and CLI
- [Java](https://www.java.com/) for Android and Enterprise
- [Python](https://www.python.org/) for Data Science
- [Swift](https://www.python.org/) for Apple's ecosystem
- [TypeScript](https://www.typescriptlang.org/) for Web

### Tour

There is always where you want to start with a new language - what will Esper
look like? The answer is a lot like Rust. I just happen to think that Rust gets
a lot of stuff right. That said, expect some deviations made in the interest of
developer ergonomics and Esper's particular domain challenges.

#### Simple types

##### Primitives

Esper's primitives are mostly stolen from Go. It has:

- `bool`\
  Boolean value that can be `true` or `false`.
  
  `bool` literals look like `true` or `false`.
- `byte`\
  An 8-bit unsigned integer, often in strings. NOTE: in the context of a `string`,
  `byte` does not encapsulate the semantic concept of a "character" since in some
  encodings, like [UTF-8](https://developer.mozilla.org/en-US/docs/Glossary/UTF-8),
  a character may be expressed with more than one byte.
  
  `byte` literals are just non-negative numbers like `128`.
- `double`\
  A 64-bit signed floating point number.
  
  `double` literals are either dot-delimited decimal numbers like `-12.80001`, or
  scientific notation like `1.2e-12`.
- `int`\
  A 64-bit signed integer number.
  
  `int` literals are just numbers like `11` or `-7`. Esper does not support binary,
  hex, or octal `int` literals.
- `rune`\
  A 32-bit unsigned integer number intended to represent the semantic concept of
  a "character" in strings.
  
  `rune` literals are just characters like `'k'` or `'💩'`.
- `string`\
  A sequence of bytes semantically associated with text.
  
  `string` literals are usually quoted spans of text like `"hello world"`, but they
  come in other flavors too
  
  ```
  "\"With escaped characters\t";

  """A
  multiline
  string""";

  r#"
   a raw string where \n escaped characters are not a thing
   ";
  ```

##### Tuples

Tuples are a fixed-size collection of different types. They can be helpful in situations where you want to group a few different pieces of data without creating a dedicated, named data structure for them. Rust Tuples are great. Why mess with a good thing? Esper Tuples are functionally identical.

```rust
// Tuple of an `int`, a `string`, and a `bool`. By the way, `let` is how we
// create variables in Esper. We'll elaborate in depth a little later.
let tuple = (123, "456", true);

// You can read different parts of a Tuple with dot notation.
print(tuple.0); // Prints "123"
print(tuple.1); // Prints "456"
print(tuple.2); // Prints "true"

print(tuple.7); // Compile-time error
```

##### Lists

TODO(skeswa): flesh this out.

##### Maps

TODO(skeswa): flesh this out.

##### Sets

TODO(skeswa): flesh this out.

##### Unions

TODO(skeswa): flesh this out.

##### Aliases

TODO(skeswa): flesh this out.

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

#### Expressions and statements

Following Rust's lead, Esper is (mostly) an expression language.
[Rust's documentation](https://doc.rust-lang.org/reference/statements-and-expressions.html)
does a good job describing what this means, and some of its implications:

> Most forms of value-producing or effect-causing evaluation are directed by the
> uniform syntax category of _expressions_. Each kind of expression can
> typically nest within each other kind of expression, and rules for evaluation
> of expressions involve specifying both the value produced by the expression
> and the order in which its sub-expressions are themselves evaluated.
>
> In contrast, statements in Rust serve mostly to contain and explicitly
> sequence expression evaluation.

The quoted description can be a bit difficult to fully understand, but it
basically boils down to a simple mantra: in Esper, almost everything, including
control flow like `if...else`, is an "expression" can be used like a value.
Expressions can be terminated, and their values contained, by capping them with
a `;` character. Loosely, a terminated expression _is_ a "statement".

Perhaps the best way to visualize this is to demonstrate an example involving
`if...else`, Esper's simplest branching control flow expression.

```rust
// Pretend that `some_random_number` is defined elsewhere and is a randomly
// generated `int`.
let x = some_random_number;

// Below, `message`'s value results from an `if...else if...else` expression
// on `x`. When `x` is `4`, `message` is `"x is four"`. Also, if `x` is not `3`
// or `4`, `message` is the empty string.
let message = if x == 4 {
  "x is four"
} else if x == 3 {
  "x is three"
} else {
  ""
};

// As you can see, expressions can also be used in a sort of stand-alone
// fashion.
if !message.is_empty() {
  print(message);
} else {
  print("Looks like there is nothing to say!");
}
```

#### Functions

Syntactically, Esper functions are very similar Rust functions.

```rust
// Functions can specify a return type using the `->` symbol.
pub fn multiply_by_two(num: double) -> double {
  // Functions implicitly return the last value in their body. Since, the next
  // line is not terminated by a `;`, it evaluates to `num * 2`.
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

#### Complex Types

##### Structs

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
    // literals adjacent to each other. Esper borrows this syntax from Dart and
    // JavaScript.
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

my_car.drive(); // Prints "A Mazda Miata owned by Some User is driving"
```

Esper has some syntactic sugar to make this a little smoother.

```rust
let my_other_car = Car {
  make: "Nissan",
  model: "Altima",
  owner: {
    active: false,
    coolness_rating: -1,
    name: "Another User",
  },
};

my_car.drive(); // Prints "A Nissan Altima owned by Another User is driving"
```

One important thing to note here is that Esper structs, like most Esper data
structures, are immutable by default. So, direct imperative mutation of Esper
structs won't work in all the cases that you may be used to.

```rust
let my_car = Car {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

my_car.make = "Toyota"; // Compile-time error.
```

To "change" a value within an immutable `struct` instance, we have to first
clone it first as a mutable value. Esper has a special syntax for doing just
this.

```rust
let mut my_mut_car = my_car.mut;

my_mut_car.make = "Toyota"; // This is a-ok.
```

`x.mut` produces a mutable clone of `x`. Usually when you use `.mut`, you want
to change one or more fields of a `struct`. To make this a little more
ergonomic, Esper ships with some syntax sugar.

```rust
let my_first_car = my_car.mut {
  make: "Toyota",
  model: "Camry",
  year: 2008,
};
```

Often times, mutation is needs to happen deeper in the the struct. Luckily,
Esper allows for `mut { ... }` to be used on sub-structs too.

```rust
let my_next_car = my_immutable_first_car.mut {
  make: "Rivian",
  model: "R1T",
  user: mut {
    coolness_rating: self.coolness_rating + 1,
  },
  year: 2023,
};
```

Sometimes, you just gotta mutate structs directly. This is fairly simple to do
in Esper. All you have to do in order to create a mutable `struct` is use the
`mut` at creation time:

```rust
struct Donut {
  is_tasty: bool;
}

// Esper infers that since the variable is declared as `mut`, the instantiated
// `struct` is mutable.
let mut disappointing = Donut { is_tasty: false };

disappointing.is_tasty = true; // This is a-ok.
```

###### Generics

TODO(skeswa): flesh this out.

##### Traits

TODO(skeswa): flesh this out.

##### Enums

TODO(skeswa): flesh this out.

#### Pattern matching

TODO(skeswa): flesh this out.

## Prototype

I think it might be a good idea to check out something like
[lalrpop](http://lalrpop.github.io/lalrpop/).
