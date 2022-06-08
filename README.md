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

"Esper" is a contraction of
["Esperanto"](https://en.wikipedia.org/wiki/Esperanto). Esperanto was originally
intended to be a universal second language for international communication. In a
sense, Esper aspires to a similar goal - just with programming languages
instead.

### Potential other names

- **Creo**\
  Contraction of ["Creole language"](https://en.wikipedia.org/wiki/Creole_language)
  which this lanuage would be.
- **Mutt**\
  This language is a mix of the other ones and will live amongst them.

## Design

Esper is not designed to be fast, or sexy, or interesting, or well-suited for
any specific domain. It should fit right into the source code powering any ol'
user interface, backend API, system admin script, and smart fridge. Esper's
guiding design principles, are to be maximally:

- Simple,
- Familiar,
- Practical, and
- Interoperable

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

**tl;dr** "Dart with Rust syntax and Go's packaging model".

### Compatibility

For Esper to be useful, in needs to be able to interop with most of the major
languages with an established industrial presence. Esper is being developed with
the following transpilation targets in mind:

- [Dart](https://dart.dev/) for Flutter
- [Go](https://go.dev/) for Cloud and CLI
- [Java](https://www.java.com/) for Android and Enterprise
- [Python](https://www.python.org/) for Data Science
- [Swift](https://www.swift.org/) for Apple's ecosystem
- [TypeScript](https://www.typescriptlang.org/) for Web

Prototyping will likely focus on `Go` and `TypeScript` because they are
sufficiently, popular, and generally useful.

### Tour

There is always where you want to start with a new language - what will Esper
look like? The answer is a lot like Rust. I just happen to think that Rust gets
a lot of stuff right. That said, expect some deviations made in the interest of
developer ergonomics and Esper's particular domain challenges.

#### Primitives

Esper's primitives are mostly stolen from Go. It has:

- `bool`\
  Boolean value that can be `true` or `false`.

  A `bool` literal is either `true` or `false`.

- `byte`\
  An 8-bit unsigned integer, often in strings. NOTE: in the context of a `string`,
  `byte` does not encapsulate the semantic concept of a "character" since in some
  encodings, like [UTF-8](https://developer.mozilla.org/en-US/docs/Glossary/UTF-8),
  a character may be expressed with more than one byte.

  A `byte` literal is just a non-negative number like `128`.

- `double`\
  A 64-bit signed floating point number.

  A `double` literal is either dot-delimited decimal numbers like `-12.80001`,
  or scientific notation like `1.2e-12`.

- `int`\
  A 64-bit signed integer number.

  An `int` literal is just a number like `11` or `-7`. Notably, Esper does not
  support binary, hex, or octal `int` literals.

- `rune`\
  A 32-bit unsigned integer number intended to represent the semantic concept of
  a "character" in strings.

  A `rune` literals is just a single-quoted character like `'k'` or `'💩'`.

- `string`\
  A sequence of bytes semantically associated with text.

  A `string` literal is usually a quoted span of text like `"hello world"`, but
  it comes in other flavors too.

  ```dart
  "\"With escaped characters\t";

  """A
  multiline
  string""";

  r#"
   a raw string where \n escaped characters are not a thing
   "#;
  ```

  Esper also supports Dart-style `string` concatentation. You can concatentate
  string literals by declaring them adjacent to one another. Though many other
  languages support it however, Esper will not support `string` concatentation
  via the `+` operator.

  ```dart
  "this a long string that I'm' worried will end up exceeding the line limit, "
  "but luckily I can just continue with another adjacent string on the "
  "next line";
  ```

  `string` interpolation will look very familar to fans of
  [ES2015+](https://babeljs.io/docs/en/learn/#template-strings) or
  [Dart](https://dart.dev/guides/language/language-tour#strings).

  ```dart
  "${1 + 2 + 3 + 4 + 5} is fifteen";

  "You can also reference $some_variable without the `{}`";
  ```

##### Special primitives

It is important to note that **Esper does not have a null-type like Go's
`nil`**. The closest idea that Esper has in this regard is the `()` type also
called "unit". Since this idea is wholly stolen from Rust, we can lean on the
[Rust docs](https://doc.rust-lang.org/std/primitive.unit.html) for a
description:

> The `()` type has exactly one value `()`, and is used when there is no other
> meaningful value that could be returned. `()` is most commonly seen
> implicitly: functions without a `-> ...` implicitly have return type `()`,
> that is, these are equivalent:

> ```rust
> fn long() -> () {}
>
> fn short() {}
> ```

Esper also steals the `unknown` type from TypeScript. `unknown` represents all
possible values. Every type is assignable to type `unknown`. Therefore the type
`unknown` is a universal supertype of the type system. However, the Esper
compiler won't allow any operation on values typed `unknown` - the values must
first be cast to a narrower type. For more on this concept, check out
[some information on TypeScript's `unknown`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-0.html#new-unknown-top-type).

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

#### Tuples

Tuples are a fixed-size collection of different types. They can be helpful in
situations where you want to group a few different pieces of data without
creating a dedicated, named data structure for them. Rust Tuples are great. Why
mess with a good thing? Esper Tuples are functionally identical.

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

#### Lists

Perhaps the most common collection in most languages is a `List`, an ordered
sequence of values that supports random access. In JavaScript, it is called
`Array` while in Rust it is called `Vec`. Esper lists should look feel and
behave like Dart's `List` or JavaScript's `Array`.

```rust
// The type of `list` is inferred to be `[int]` here.
let list = [1, 2, 3];
```

Like in other languages, Esper lists support random access by index with the
`[]` operator.

```rust
// The type of `list` is inferred to be `[int]` here.
let list = [1, 2, 3];

print(list[0]); // Prints "1"
print(list[2]); // Prints "3"

print(list[17]); // Compile-time error
```

Need your list to be mutable? Suffix the literal with a `!`.

```rust
let mutable_list = [1, 2, 3]!;
```

Sometimes when you have a mutable list, it starts empty. In this situation, the
inner type of the list is ambiguous, so it falls back to `unknown` by default.
You can help provide more type information on the variable or explicitly cast
the array literal to correct his.

```rust
// Esper is able to infer that the list should be created mutably from its type
// annotation, so the `!` prefix is not necessary on the list literal itself.
let another_list: [string]! = [];
// In Esper, like in Rust, you can cast a value with the `as` keyword.
let yet_another_list = [] as [bool]!;
```

Esper lists have lots of helpful methods focused on mutation.

```rust
let some_list: [int]! = [];
some_list.add(2);
some_list.add(1);

print(some_list); // Prints "[2, 1]"

some_list.remove(at_index: 0);

print(some_list); // Prints "[1]"

print(list[2]); // Prints "3"
print(mutable_list[0]); // Prints "2"

mutable_list.remove(index: 2); // Prints "[1]"
```

TODO(skeswa): spread notation

```rust
let x = [1, 2, 3];
let y = [...x, 4, 5, 6];
```

TODO(skeswa): destructuring

```rust
let [first, second, ...everything_else] = y;
```

#### Maps

TODO(skeswa): flesh this out (Dart Maps).

```rust
// Below, `x` is inferred to be of type `{string: int}`.
let x = {
  "one": 1,
  "two": 2,
  "three": 3,
};

// Below, `y` is inferred to be of type `{int: unknown}` since `unknown` is the
// only common ancestor of `string`, `[int]`, and `{string: string}`.
let y = {
  1: "one",
  2: [1, 2],
  3: {"hello": "world"},
};
```

TODO(skeswa): spread notation

```rust
let x = {
  "one": 1,
  "two": 2,
  "three": 3,
};
let y = {...x, "four": 4};
```

TODO(skeswa): destructuring

```rust
let {"one": one, "two": two, ...everything_else} = y;
```

#### Sets

TODO(skeswa): flesh this out (Dart Sets).

```rust
let x = {"one", "two", "three"};
let y: {double} = {1, 2.2, 3};
```

TODO(skeswa): spread notation

```rust
let x = {
  "one": 1,
  "two": 2,
  "three": 3,
};
let y = {...x, "four": 4};
```

TODO(skeswa): no destructuring

#### Type Aliases

Esper allows you to come up with another name for an existing type using
something called a type alias. Like Rust type aliases, Esper type aliases are
declared with the keyword `type`. For example, the following defines the type
`Point` as a synonym for the type `(int, int)`:

```rust
type Point = (int, int);

let p: Point = (41, 68);
```

Type aliases can be useful to abbreviate long, verbose types. Type aliases also
come in handy when attaching more semantic meaning or description to a common
type, like a tuple in the case above, would make your code easier to reason
about.

#### Operators

Esper features the "usual suspects" for a C-family language:

- `&&`, `||`, `==`, and `!=`\
  Logical comparison operators work the way that you think they do. One thing to
  note is that the equality comparison operators do not perform qualitative comparison.
  Instead they do a dumb equality check, like Java, for instance.
- `+`, `-`, `*`, `/`, and `%`\
  Arithmetic operators can only be applied to numbers of the same kind.
- `**`, the exponentiation operator, is stolen from
  [Python](https://docs.python.org/3/reference/expressions.html#the-power-operator)
- `~/`, the truncating division operator, is stolen from
  [Dart](https://api.flutter.dev/flutter/dart-core/num/operator_truncate_divide.html)

You might be wondering where the bitwise operators are - there are none! Looking
for operator overloads? You won't find them here.

Good riddance.

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
// Pretend that `some_random_int` is defined elsewhere and is a randomly
// generated `int`.
let x = some_random_int;

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
to this rule: **if a function has just a single argument, no label is
necessary**.

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
```

Esper functions can also have optional arguments. One way to accomplish this is
to specify a default value for a parameter.

```rust
// Below Esper infers that `action` is a `string` from its default value.
//
// We explicitly specify a type for `times` because `= 1` would it an `int` by
// default.
//
// NOTE: default values must be immutable.
fn i_cant_wait_to(action = "take a nap", times: double = 1) {
  print("Time to $action $times time(s)!");
}

print(
  i_cant_wait_to(
    action: "eat donuts",
  ), // prints "Time to eat donuts 1 time(s)!"
);
print(
  i_cant_wait_to(
    action: "eat donuts",
    times: 1.5,
  ), // prints "Time to eat donuts 1.5 times(s)!"
);
print(i_cant_wait_to()); // prints "Time to take a nap 1 times!"
```

Another way to declare an optional argument is to make its type `Option<T>`.
`Option<T>` represents an optional value of type `T`. We'll describe `Option<T>`
in greater depth later, but it works identically to how Rust's
[`Option<T>`](https://doc.rust-lang.org/std/option/) works.

```rust
fn measurement(scalar: double, unit: Option<string>) {
  "$scalar"
  // `Option::unwrap_or` is a method defined on any `Option<T>` that
  // evaluates to the wrapped value of type `T`, if such a value exists, falling
  // back to the specified value of type `T`. We'll cover `Option` and other
  // enums like it a little later.
  "${unit.unwrap_or("")}"
}

print(measurement(scalar: 12.5, unit: Some("px"))); // prints "12.5px"
print(measurement(scalar: 12.5, unit: None)); // prints "12.5"
```

Esper also includes some syntactic sugar to make using this a little less
verbose by allowing `Some` or `None` to be implied by the respective inclusion
or exclusion of an argument.

```rust
print(measurement(scalar: 12.5)); // prints "12.5"
print(measurement(scalar: 12.5, unit: "px")); // prints "12.5px"
```

Esper even borrows Rust's syntax for lambda functions.

```rust
let lambda_annotated = |i: int| -> int { i + 1 };
let lambda_inferred  = |i     |          i + 1  ;
```

#### Enums

TODO(skeswa): flesh this out.

##### Special enums

TODO(skeswa): flesh this out (`Option`, `Result`).

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

Like Deno and Go, remote modules are fetched from source control with an
accompanying version. Version is typically inferred from a tag or reference on
the imported module repository.

```rust
from "github.com/abc/xyz@v0.9.0" use { hello, world };
// `as` lets you alias things you import or export.
from "my.repo.com/foo/bar@v1.2.0-beta/nested/module" use * as module;
```

Esper allows imports from other modules in the same repository, too. These are
called respository-relative imports. `~` always refers to the root of the
repository containing the current module. Respository-relative imports do not
specify a version because the version will always be the same as the current
module.

```rust
from "~/bing/bang" use { boom as büm };
from "~/some/sub/module" use { something };
```

You can also re-export stuff using the `show` keyword.

```rust
from "github.com/abc/xyz@v0.9.0" show { hello };
from "my.repo.com/foo/bar@v1.2.0-beta/nested/module" show *;

from "~/bing/bang" show { boom as büm };
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
    print(
      "A ${self.model} ${self.model} owned "
      "by ${self.owner.name} is driving",
    );
  }

  // Functions defined within a `struct` don't have to be attached to a
  // particular instance of that struct. They can instead function like static
  // methods in other languages. For instance, you would invoke this function
  // with `Car::create_lemon_for(some_user)`.
  fn create_lemon_for(owner: User) {
    Car { make: "AMC", model: "Pacer", owner }
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

The only way to create a mutable `struct` instance is to create it with a `!`
suffixing the `struct` type. In Esper, structs and traits with a `!` are
internally mutable.

```rust
let my_mut_car = Car! {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

my_mut_car.make = "Toyota"; // 👍
```

All Esper structs can be shallow cloned with `fork`. This useful when a field
inside an immutable `struct` value should change.

```rust
let my_car = Car {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

let my_other_car = fork my_car {
  // `make` is changed, but all of the other fields stay the same.
  make: "Toyota",
};

print(my_car.make) // Prints "Mazda"
print(my_other_car.make) // Prints "Toyota"

my_other_car.make = "Toyota"; // Compile-time error (`my_other_car` is not a `Car!`)
```

But what if you need the forked `struct` instance to be internally mutable? This
is made possible by suffixing `fork` with a `!`.

```rust
let my_car = Car {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

let my_other_car = fork! my_car;

my_car.make = "Toyota"; // 👍
```

Sometimes you need to `fork` a `struct` instance nested within another `struct`
instance. Given how frequently this is necessary, it felt like a good idea for
Esper to ship with a dedicated syntax.

```rust
let my_car = Car {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

let my_other_car = fork my_other_car {
  make: "Rivian",
  model: "R1T",
  user: fork {
    // `self` refers to the original value of this `User` field.
    coolness_rating: self.coolness_rating + 1,
  },
};
```

In some situations, you may want nested internal mutation: you way want to be
able to directly mutate an inner `struct` instance nested within another
`struct` instance. Esper supports this by declaring the inner `struct` field as
mutable with `!`. Note that this nested internal mutablility is only accessible
in situations where the surrounding type is itself mutable.

```rust
struct House {
  address: Address;
  owner: User!;
}

struct Address {
  number: string;
  street: string;
}

let a_house = House {
  address: {
    number: "42",
    street: "Wallaby Way",
  },
  owner: {
    active: false,
    coolness_rating: -1,
    name: "P. Sherman",
  },
};

a_house.owner.active = true; // Compile-time error (`a_house` is not a `House!`)

let another_house = House! {
  address: {
    number: "221B",
    street: "Baker St",
  },
  owner: {
    active: false,
    coolness_rating: 99,
    name: "S. Holmes",
  },
};

a_house.owner.active = true; // 👍
a_house.address.street = "Baker Street"; // Compile-time error (`House::address` is not an `Address!`)
```

#### Traits

TODO(skeswa): flesh this out.

#### Impls

##### Importing impls

TODO(skeswa): flesh this out.

```rust
from "a/b/c" use { Trait + OtherTrait for Thing, Thing::custom_impl_method };
```

#### Generics

TODO(skeswa): flesh this out.

#### Type Unions and Intersections

TODO(skeswa): flesh this out (Rust Type Unions (trait + trait) and TS
Intersections (type | type).

#### Destructuring

TODO(skeswa): flesh this out.

#### Pattern matching

TODO(skeswa): flesh this out.

#### Concurrency

TODO(skeswa): flesh this out. This is gonna be a spicy meatball.

```rust
// This blocks.
let weather = fetch(url: "http://whatistheweather.com");

// `async` makes the expression following it not block. `async` yields an
// `Eventual<T>`.
let x = async fetch(url: "http://google.com/favicon.ico");
let y = async wait(Duration::new(milliseconds: 100));
let z = async readFile(path: "./a/b/c.txt");

// Promise.all(...)
let (x, y, z) = parallel (
  x,
  y,
  z
);

// Promise.allSettled(...)
let (x, y, z) = parallel::settle (
  x,
  y,
  z
);

// Promise.race(...)
let (x, y, z) = parallel::race (
  x,
  y,
  z
);
```

#### Interop

TODO(skeswa): flesh this out.

```rust
#[link(go_package = "github.com/my/go@1.18/thing")]
extern {
  fn fetch(url: string) -> Eventual<Result<Response, FetchError>>;
}
```

## Prototype

I think it might be a good idea to check out something like
[lalrpop](http://lalrpop.github.io/lalrpop/).
