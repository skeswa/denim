# 👖 Denim

A programming language for cross-platform code sharing.

## Pitch

So, you know how pretty much every modern, garbage-collected language feels
eerily like the others lately? I think we can blame this phenomenon on the fact
that many of these languages are converging on the same features and concepts;
how many languages have added first-class functions, co-routines, data classes,
and language-level immutablity recently?

The only _tangible_ differences between one language and another are the
ecosystems and platforms that they can unlock for you. Go gets you into the
cloud and terminal ecosystems, while JS/TS gets you into the browser and to the
edge. Swift and Kotlin get you onto phones, and with C# you can ship on an Xbox.

And it got me thinking: **if the languages we use to write our apps are this
similar, why on earth are we writing the same logic over and over again?** Why
can't we write most of our logic, constants, and types once, and use them
anywhere? What if there was a language purely designed to be interoperable with
other languages?

Denim is that language.

The intent behind Denim is to incorporate the smallest set of common features
from these garbage-collected languages sufficient to:

- Create common types
- Implement business logic
- Declare common constants

Of course, it wouldn't hurt to end up with a language that is pleasant to use
and maintain while we're at it.

## Why "Denim"?

Well, because **"denim goes with everything"** _rimshot_.

While our slogan is a little playful and intended to earn a few chuckles, I
think it accurately embodies the vision of the language. Denim aims to be as
comfortable and complementary as any well-worn pair of jeans. This is a language
at its best as a part of your outfit (or codebase for that matter).

## Design

Denim is not designed to be fast, or sexy, or interesting, or well-suited for
any specific domain. It should fit right into the source code powering any ol'
user interface, backend API, system admin script, and smart fridge. Denim's
guiding design principles, are to be maximally:

- Comfortable,
- Familiar,
- Practical, and
- Interoperable

Denim should never feel as esoteric and ornate as Rust, but it should feel a
smidge more expressive than Go. It should be easy to read, follow, and document
like Java, getting out of your way and letting you solve the damn problem like
Node.js.

Described above is a language that will be difficult to design, and even harder
to implement. Our hope in all of this, at the very least, is to move the
[Overton window](https://en.wikipedia.org/wiki/Overton_window) in a direction
that we bet the programming world would enjoy.

Wish me luck.

### Inspiration

As Denim is designed to feel familiar, it borrows heavily from some popular
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

For Denim to be useful, in needs to be able to interop with most of the major
languages with an established industrial presence. Denim is being developed with
the following transpilation targets in mind:

- [Dart](https://dart.dev/) for Flutter
- [Go](https://go.dev/) for Cloud and CLI
- [Kotlin](https://kotlinlang.org/) for Android and Enterprise Java
  compatibility
- [Python](https://www.python.org/) for Data Science
- [Swift](https://www.swift.org/) for Apple's ecosystem
- [TypeScript](https://www.typescriptlang.org/) for Web

Prototyping will likely focus on `Go` and `TypeScript` because they are
sufficiently, popular, and generally useful.

### Tour

There is always where you want to start with a new language - what will Denim
look like? The answer is a lot like Rust. I just happen to think that Rust gets
a lot of stuff right. That said, expect some deviations made in the interest of
developer ergonomics and Denim's particular domain challenges.

#### Primitives

Denim's primitives are mostly stolen from Go. It has:

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

  An `int` literal is just a number like `11` or `-7`. Notably, Denim does not
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

  Denim also supports Dart-style `string` concatentation. You can concatentate
  string literals by declaring them adjacent to one another. Though many other
  languages support it however, Denim will not support `string` concatentation
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

It is important to note that **Denim does not have a null-type like Go's
`nil`**. The closest idea that Denim has in this regard is the `void`. The
`void` type has exactly one value, `void`, and is used when there is no other
meaningful value that could be returned. `void`, much like Rust's `()`, is most
commonly seen implicitly: functions without a `->` implicitly have a `void`
return type.

```rust
// The functions below are equivalent:

fn long() -> void {}
fn short() {}
```

Denim also steals the `unknown` type from TypeScript. `unknown` represents all
possible values. Every type is assignable to type `unknown`. Therefore the type
`unknown` is a universal supertype of the type system. However, the Denim
compiler won't allow any operation on values typed `unknown` - the values must
first be cast to a narrower type. For more on this concept, check out
[some information on TypeScript's `unknown`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-0.html#new-unknown-top-type).

#### Variables

Denim steals variable declaration from Rust.

```rust
let abc = 123;
```

Like in Rust, Denim's `let` creates immutable variables by default. This means
that `abc` cannot by be assigned a new value.

```rust
let abc = 123;
abc = 321; // Compile-time error
```

To create a mutable variable, you need to add a `!` suffix to the `let` keyword.

```rust
let xyz! = 123;

xyz = 456; // 👍
```

Importantly, Denim does not have a notion of `const`. Instead `let` is also used
to declare constants at the top-level lexical scope,

#### Printing

Denim ships with two main ways to print out to the console, `print` and
`eprint`.

`print` is a function (explained in greater depth later) that takes a `string`
which it prints to its own line in the console. In the browser, this means it
calls `console.log(...)` under the hood. Outside of the browser, it appends a
line to stdout.

```rust
print("hello world"); // Prints "hello world" on its own line.
```

`eprint` is another version of `print` intended for warnings, log messages, and
errors. In the browser, this means it calls `console.warn(...)` under the hood.
Outside of the browser, it appends a line to `stderr`.

```rust
eprint("uh oh"); // Prints "uh oh" on its own line.
```

#### Tuples

Tuples are a fixed-size collection of different types. They can be helpful in
situations where you want to group a few different pieces of data without
creating a dedicated, named data structure for them. Rust Tuples are great. Why
mess with a good thing? Denim Tuples are functionally identical.

```rust
// Tuple of an `int`, a `string`, and a `bool`. By the way, `let` is how we
// create variables in Denim. We'll elaborate in depth a little later.
let tuple = (123, "456", true);

// You can read different parts of a Tuple with dot notation.
print(tuple.0); // Prints "123"
print(tuple.1); // Prints "456"
print(tuple.2); // Prints "true"

print(tuple.7); // Compile-time error
```

While Denim Tuples are always immutable, they can be quite ergonomic to use and
manipulate. Tuples can be composed together via the `...` operator, and split
apart in a similar way through de-structuring.

```rust
let x = (1.2e-3, 'e', false);
let y = ("yo", ...x, x.1);

print(y); // Prints "(yo, 0.0012, e, false, e)"

let (first, ...middle_stuff, last) = y;

print(first); // Prints "yo"
print(third); // Prints "e"

// NOTE: `middle_stuff` is itself a Tuple.
print(middle_stuff); // Prints "(0.0012, e, false)"
```

#### Array

Perhaps the most common collection in most languages is an array - an ordered
sequence of values that supports random access. In JavaScript, it is called
`Array` while in Rust it is called `Vec`. Denim Arrays should look feel and
behave like Dart's `List` or JavaScript's `Array`.

```rust
let array = [1, 2, 3];
```

Like in other languages, Denim Arrays support random access by index with the
`[]` operator.

```rust
// The type of `array` is inferred to be `[int]` here.
let array = [1, 2, 3];

print(array[0]); // Prints "1"
print(array[2]); // Prints "3"

print(array[17]); // Compile-time error
```

Need your Array to be mutable? Suffix the literal with a `!`.

```rust
let mutable_array = [1, 2, 3]!;
```

Sometimes when you have a mutable Array, it starts empty. In this situation, the
inner type of the Array is ambiguous, so it falls back to `unknown` by default.
You can help provide more type information on the variable or explicitly cast
the Array literal to correct his.

```rust
// Denim is able to infer that the Array should be created mutably from its type
// annotation, so the `!` prefix is not necessary on the Array literal itself.
let another_array: [string]! = [];
// In Denim, like in Rust, you can cast a value with the `as` keyword.
let yet_another_array = [] as [bool]!;
```

Denim Arrays have lots of helpful methods focused on mutation.

```rust
let some_array: [int]! = [];
some_array.add(2);
some_array.add(1);

print(some_array); // Prints "[2, 1]"

some_array.remove_at(0);

print(some_array); // Prints "[1]"

print(array[2]); // Prints "3"
print(mutable_array[0]); // Prints "2"
```

Denim Arrays ship with special syntax to instantiate arrays with a fixed number
of identical values.

```rust
let eight_zeroes = [0; 8]; // `eight_zeroes` is an `[int]`
let four_strings = [""; 4]; // `eight_zeroes` is an `[string]`
```

Denim Arrays are spreadable with `...` just like JavaScript arrays.

```rust
let x = [1, 2, 3];
let y = [...x, 4, 5, ...x, 6];

print(y); // Prints "[1, 2, 3, 4, 5, 1, 2, 3, 6]"
```

Denim allows for ergonomic slicing and dicing of arrays via de-structuring.
Denim Array de-structuring is very similar to JavaScript `Array` de-structuring.

```rust
let y = [1, 2, 3, 4, 5, 6];

let [first, second, ...middle_stuff, last] = y;

print(first); // Prints "1"
print(second); // Prints "2"
print(last); // Prints "6"

// NOTE: `middle_stuff` is itself an Array.
print(middle_stuff); // Prints "[3, 4, 5]"
```

#### Enums

Enums are a great way to model data that is best described in categories. For
example, the concept of "days of the week" _could_ be accurately described a
`string`, but since there are only seven kinds of them, `enum` is a better fit.
`enum` allows you to explicitly enumerate each variant.

```rust
enum DayOfTheWeek {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}
```

Broadly speaking, explicit enumeration via `enum` makes validation and pattern
matching over your data more ergnomic and less error prone. Sometimes, it makes
sense to also attach data to each variant, allowing `enum` to function more like
structs or classes in other languages.

```rust
enum IpAddress {
  // V4 IP addresses look like "192.168.1.1".
  //
  // Each `IpAddress::V4` has an inner field of type `[int]`.
  V4([int]),
  // V6 IP addresses look like "2001:db8::2:1".
  //
  // Each `IpAddress::V6` has a `segments` field of type `string` and a
  // `segment_count` of type `int`.
  V6 { segments: string, segment_count: int },
}

let some_v4_address = IpAddress::V4([192, 168, 1, 1]);

print(some_v4_address.0); // Prints "[192, 168, 1, 1]"

// `segments` does not need to be explicitly specified since it is the only
// field of `IpAddress::V4`.
let some_v4_address = IpAddress::V6 {
  segments: "2001:db8::2:1",
  segment_count: 5,
};

print(some_v6_address.segments); // Prints "2001:db8::2:1"
print(some_v6_address.segment_count); // Prints "5"
```

##### Special enums

TODO(skeswa): flesh this out by copying (`Option`, `Result`) from rust.

```rust
pub enum Option<T> {
  None,
  Some(value: T),
}

pub enum Result<T, E> where E = Error {
  Err(error: E),
  Ok(value: T),
}
```

We have special syntactic sugar for `Option`:

```rust
// This:
let x: Option<string>;
// Is the same as this:
let x: string?;
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

let y: {string: unknown} = {
  1: "one",
  2: [1, 2],
  3: {"hello": "world"},
};

print(x["one"]); // Prints "Some(1)"
print(y[2]); // Prints "Some([1, 2])"

print(x["four"]); // Prints "None"
print(y[7]); // Prints "None"
```

Denim Maps, like other Denim data structures, are made mutable with a `!`
suffix.

```rust
let mutable_map = {"one": 1, "two": 2}!;
```

Mutable Denim Maps feature useful methods and operators stolen from the Maps of
other languages.

```rust
let mutable_map: {string: string}! = {};
mutable_map["hello"] = "world";
mutable_map["foo"] = "bar";

print(mutable_map); // Prints "{hello: world, foo: bar}"

mutable_map.remove_key("hello");

print(mutable_map); // Prints "{foo: bar}"
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
// The type of `x` is inferred to be `[string]` here.
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

Denim allows you to come up with another name for an existing type using
something called a type alias. Like Rust type aliases, Denim type aliases are
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

Denim features the "usual suspects" for a C-family language:

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
- `+=`, `-=`, `*=`, `/=`\
  Arithmetic assignment operators, like in other languages, apply an arithmetic operator
  to a variable and reassign the resulting value to the variable

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

Following Rust's lead, Denim is (mostly) an expression language.
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
basically boils down to a simple mantra: in Denim, almost everything, including
control flow like `if...else`, is an "expression" can be used like a value.
Expressions can be terminated, and their values contained, by capping them with
a `;` character. Loosely, a terminated expression _is_ a "statement".

Perhaps the best way to visualize this is to demonstrate an example involving
`if...else`, Denim's simplest branching control flow expression.

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

#### Keywords

TODO(skeswa): list all of them here

##### Suffixing

In Denim,some keywords can be applied as suffixes with `.` notation. Namely,
`async`, `await`, `fork`, `if`, `match`, `try`, and `while`.

```rust
let some_eventual_function_value = async some_function("abc");

let some_other_eventual_function_value = some_other_function("abc").async;

print(await some_eventual_function_value);

print(some_other_eventual_function_value.await);

fork some_struct {
  some_field: "123",
};

some_struct.fork {
  some_field: "123",
};

if some_bool_expression {
  print("its bigger")
} else {
  print("its _not_ bigger")
}

some_bool_expression.if {
  print("its bigger")
} else {
  print("its _not_ bigger")
}

some_number.match {
  1..=122 => print("nope"),
  123 => print("yep"),
  _ => print("nope"),
}

match some_number {
  1..=122 => print("nope"),
  123 => print("yep"),
  _ => print("nope"),
}

fn x() -> Result {
  something_that_can_fail(123).try;
  try {
    let a! = 123;
    a = a * 2;

    something_that_can_fail(123)
  }
}

let i! = 0;
while i < 3 {
  print("i is $i");
}

let is_done! = false;
is_done.while {
  is_done = true;
}
```

#### Functions

Syntactically, Denim functions are very similar Rust functions.

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

There is just one wrinkle with Denim functions - there are no positional
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

Denim functions can also have optional arguments. One way to accomplish this is
to specify a default value for a parameter.

```rust
// Below Denim infers that `action` is a `string` from its default value.
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

Another way to declare an optional argument is to make its type `T?`. `T?`
represents an optional value of type `T`. We'll describe `T?` in greater depth
later, but it works identically to how Rust's
[`Option<T>`](https://doc.rust-lang.org/std/option/) works.

```rust
fn measurement(scalar: double, unit: string?) {
  let lowercase_unit = unit?.to_lower() ?? "";

  "$scalar $lowercase_unit"
}

print(measurement(scalar: 12.5, unit: Some("px"))); // prints "12.5px"
print(measurement(scalar: 12.5, unit: None)); // prints "12.5"
```

Denim also includes some syntactic sugar to make using this a little less
verbose by allowing `Some` or `None` to be implied by the respective inclusion
or exclusion of an argument.

```rust
print(measurement(scalar: 12.5)); // prints "12.5"
print(measurement(scalar: 12.5, unit: "px")); // prints "12.5px"
```

Denim also has a convenient alternate syntax for anonymous functions, called
lamba functions, that it borrows from Rust closures. Unlike Rust closures
however, Denim lambda functions are just plain ol' functions with no extra or
special behaviors.

```rust
fn sum_three(a: int, b: int, c: int) -> int {
  a + b + c;
}

let sum_three_as_a_lambda = |a: int, b: int, c: int| a + b + c;
```

The argument types of a lambda can be inferred if enough information is provided
at the call site.

```rust
let inferred: fn(a: int) -> int = |a| a + 1;
```

By the way, function types look like this:

```rust
fn(a: A, b: B, c: C) -> D;
```

Denim has a special syntax for functions that receive an inlined anonymous
function as an argument. This common when passing callbacks and in embedded
DSLs. Any function argument named `block` can have an inline block after its
invocation that works like a lambda function.

```rust
fn element(name: string, block: Option<fn() -> Element>) -> Element {
  Element { name, child: block() }
}

// This:
element(name: "div", block: || {
  element(name: "button", block: || {
    span("click me")
  })
})

// Is the same as this:
element("div") {
  element("button") {
    span("click me")
  }
}
```

Thanks to "getterification", we can make this syntax even sweeter:

```rust
fn button(block: Option<fn() -> Element>) -> Element {
  element("button", block)
}

fn div(block: Option<fn() -> Element>) -> Element {
  element("div", block)
}

div {
  button {
    span("click me")
  }
}
```

#### Modules

Denim's core packaging primitive is called a "module". Modules designed to
define behavior, state, and types for a particular context or domain.
Furthermore, Denim modules are intended to depend on each other through the
import and export of behavior, state, and types.

Denim modules are expressed a file system as a directory with Denim source
files. Each source file in an Denim module has full visibility of everything
declared in the other source files of the module. Additionally, each source file
can do its own importing and exporting. Denim source files can only import stuff
exported by another Denim module. It might help to think of this style of
packaging is a blend of
[Go packages](https://www.golang-book.com/books/intro/11) and
[ES modules](https://hacks.mozilla.org/2018/03/es-modules-a-cartoon-deep-dive/).

Like Deno and Go, remote modules are fetched from source control with an
accompanying version. Version is typically inferred from a tag or reference on
the imported module repository.

```rust
from "github.com/abc/xyz@v0.9.0" use { hello, world };
```

Denim allows imports from other modules in the same repository, too. These are
called respository-relative imports. `~` always refers to the root of the
repository containing the current module. Respository-relative imports do not
specify a version because the version will always be the same as the current
module.

```rust
from "~/bing/bang" use { boom: büm }; // Aliases `boom` to `büm`.
from "~/some/sub/module" use { something };

// You can use ... syntax to import "everything else".
from "github.com/foo/bar@v4.2" use { Foo, bar, ...foo_bar };
from "github.com/bing/bong@latest" use { ...bing_bong };
```

You can also re-export stuff using the `show` keyword.

```rust
from "github.com/abc/xyz@v0.9.0" show { hello };

from "~/bing/bang" show { boom as büm };
```

Like in Rust, you can export stuff from Denim modules with the `pub` keyword.
Anything not declared with a `pub` will only be visible to things in its own
module.

```rust
pub let stuff_outside_of_this_module_can_see_me = true;
```

#### Structs

You may now be wondering how more complex data structures are created and
managed in Denim. I'm sure you are _so_ shocked to find out that we (mostly)
stole Rust syntax here too.

```rust
// We can make some or even all of the `struct` or its fields visible to
// external modules using the `pub` keyword.
pub struct User {
  active = false,
  /// You can put doc comments directly on `struct` fields.
  coolness_rating: int,
  pub name: string,
  pub nickname: string?,
}
```

We can instantiate and use `struct` like this:

```rust
let some_user = User {
  active: true,
  coolness_rating: 42,
  name: "Some User",
  nickname: None,
};
```

In the `User` `struct` above, two fields are optional - `active` and `nickname`.
`active` is made optional by the specification of a default value for it,
`false`. `nickname` is optional because it is of type `Option<string>`. Optional
fields may be excluded when a struct is instantiated.

```rust
let sam = User {
  coolness_rating: 11,
  name: "Sam",
  // Since `nickname` is an `Option<string>` we can simply specify a `string`.
  nickname: "S-money",
};

let tria = User {
  active: true,
  coolness_rating: 12,
  name: "Tria",
};

print(tria.nickname); // Prints "None".

let jen = User {
  active: true,
  coolness_rating: 13,
  name: "Jen",
  // Below, `"Jenners"` is automatically wrapped in a `Some(..)`.
  nickname: "Jenners",
};
```

Denim sugarifies Rust's default `impl` by simply allowing you to declare methods
within the `struct` itself. This should feel familiar to anyone coming from a
language that makes heavy use of classes.

```rust
struct Car {
  make: string,
  model: string,
  owner: User,

  // `self` is a special argument. Like in Rust, it means that this method is
  // attached to any instance of `Car`.
  fn drive(self) {
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

As structs a big part of the language, Denim has some syntactic sugar to make
instantiating nested structs ergonomic.

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

One important thing to note here is that Denim structs, like most Denim data
structures, are immutable by default. So, direct imperative mutation of Denim
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
suffixing the `struct` type. In Denim, structs and traits with a `!` are
internally mutable.

```rust
let my_mut_car = Car! {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

my_mut_car.make = "Toyota"; // 👍
```

All Denim structs can be shallow cloned with `fork`. This useful when a field
inside an immutable `struct` value should change.

```rust
let my_car = Car {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

let my_other_car = my_car.fork {
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

let my_other_car = my_car.fork!;

my_car.make = "Toyota"; // 👍
```

Sometimes you need to `fork` a `struct` instance nested within another `struct`
instance. Given how frequently this is necessary, it felt like a good idea for
Denim to ship with a dedicated syntax.

```rust
let my_car = Car {
  make: "Mazda",
  model: "Miata",
  owner: some_user,
};

let my_other_car = my_other_car.fork {
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
`struct` instance. Denim supports this by declaring the inner `struct` field as
mutable with `!`. Note that this nested internal mutability is only accessible
in situations where the surrounding type is itself mutable.

```rust
struct House {
  address: Address,
  owner: User!,
}

struct Address {
  number: string,
  street: string,
}

let a_house = House {
  address: {
    number: "42",
    street: "Wallaby Way",
  },
  owner: {
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
    coolness_rating: 99,
    name: "S. Holmes",
  },
};

a_house.owner.active = true; // 👍
a_house.address.street = "Baker Street"; // Compile-time error (`House::address` is not an `Address!`)
```

### Control Flow

TODO(skeswa): document branching
TODO(skeswa): document loops
TODO(skeswa): document loop labels (https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html)

#### "Selfification" (a.k.a Functions as Methods)

Here, we take:

- "function" to mean a subroutine **without** a `self` reference to some piece
  of state
- "method" to mean a subroutine **with** a `self` reference to some piece of
  state

```rust
fn authenticate(environment: Environment, user: User) -> bool {
  // ...
}

// `@` is the secret sauce here
fn do_stuff(environment: Environment, user: User?) {
  user?.authenticate(environment, user: @).if {
    print("yes!");
  } else {
    print("no!");
  };
}
```

#### "Getterification" (a.k.a Functions as Fields)

In Denim, functions can be treated as fields if they are not passed any
arguments.

```rust
struct Foo {
  bar: int,

  pub fn baz(self, extra = 0) -> int {
    self.bar + extra + 1
  }
}

let foo = Foo { bar: 27 };

print(foo.baz(2)); // prints "30"
print(foo.baz()); // prints "28"
print(foo.baz); // prints "28"

```

#### Autoboxing

Given the prevalance and importance of special enums `Option<T>` and `Result<T>`
in Denim, you might find yourself wrapping things in `Some(..)` and `Ok(..)`
alot. To sweeten this syntax a little bit, Denim will automatically infer a `T`
as `Some(T)` or a `Ok(T)` depending on context; this is called "autoboxing".

```rust
// Why do this:
fn foo() -> string? {
  Some("bar")
}

// when you could simply do this:
fn autoboxed_foo() -> string? {
  "bar"
}

// `Result` is also autoboxable:
let x!: Result<int> = Ok(1);
x = 2;

// Autoboxing works in structs too!
struct X(Option<int>);
let x = X(1234);
```

#### Traits

TODO(skeswa): flesh this out.

```rust
pub trait Summary {
  fn summarize(self) -> string;
}

pub trait Tweak<T> {
  tweakable: T?;

  fn tweak(self!, some_other_arg: string) -> Self;
}

pub trait DefaultImpl {
  fn foo(self!) -> string {
    "bar"
  }
}

pub trait MethodGeneric {
  fn some_method<T>(self) -> T;
}
```

##### Impls

Declaration:

```rust
// non-public
impl B for A {
  fn b(self) {
    print("b");
  }
}

// non-public
impl A for X {
  fn a(self) {
    print("a");
  }
}
impl Y for X {
  fn y(self) {
    print("y");
  }
}
impl Z for X {
  fn z(self) {
    print("z");
  }
}

// non-public
impl X {
  fn new_functionality_for_x(self) {
    print("that newnew");
  }
}

// non-public
pub impl X {
  fn eat(self) {
    print("omnomnom");
  }
}
```

Importing:

```rust
from "~/foo/bar" use {
  B for A,
  Y + Z for X,
  X,
  A,
  X::new_functionality_for_x,
};
```

#### Generics

TODO(skeswa): flesh this out.

TODO(skeswa): `where`.

#### Type Unions and Intersections

TODO(skeswa): flesh this out (Rust Type Unions (trait + trait) and TS
Intersections (type | type).

```rust
type IntOrString = int | string;

fn do_a_thing(data: IntOrString) {
  if data is int {
    print("it is an int")
    return;
  }

  print("it is a string")
}

fn do_another_thing(data: int | string | byte) {
  match data.type {
    int => print("it is an int"),
    string => print("it is an int"),
    _ => print("it is a byte"),
  }
}

trait Foo {
  fn bar(self) -> int;
  fn baz(self) -> bool;
}

trait Ping {
  fn bar(self) -> int;
  fn pong(self);
}

fn do_one_more_thing(data: Foo | Ping) {
  print(data.bar())
}

type FooAndPing = Foo & Ping;

fn do_one_last_thing(data: FooAndPing) {
  data.pong();

  print(data.bar())
}
```

#### Error handling

You can hoist the error out of returned `Result` within a function that returns
`Result` with the `try` keyword.

TODO(skeswa): flesh this out.

```rust
fn do_a_thing() -> Result<void> {
  let csv_file_path = "a/b/c.csv";

  let csv_data = read_file(csv_file_path).try.to_utf8();

  basic_dance_move().context("tried to bust a move").try;
}
```

TODO(skeswa): flesh this out.

```rust
pub struct Error<ErrorKind> {
  pub cause: Error?;
  pub kind: ErrorKind;
}
```

TODO(skeswa): flesh this out.

```rust
panic("Oh no! We cannot recover from this!");
```

#### Pattern matching

TODO(skeswa): flesh this out.

#### Concurrency

In Denim, function invocations work differently than in other languages - they
do not "block" by default. Instead, Denim functions work like
[co-routines](https://en.wikipedia.org/wiki/Coroutine), suspending themselves
while invoking other functions, and resuming when they return.

```rust
// This call to `fetch` blocks, meaning that "done" will not be printed until
// the fetch is finished.
let weather = fetch(url: "https://get.theweather.com");
print("done");

// The code below:
// 1. Prints "a"
// 2. Waits for the first thing to be fetched
// 3. Prints "b"
// 4. Waits for the second thing to be fetched
// 5. Prints "c"

print("a");
let first_thing = fetch(url: "https://first.thing.com");
print("b");
let second_thing = fetch(url: "https://second.thing.com");
print("c");
```

To call a function concurrently, use the `async` keyword. Using `async` on a
function invocation wraps its return value in a special type called `Eventual`,
like `Future` or `Promise` in other languages, is a concurrency monad that
allows you to subscribe to the completion of an asynchronous operation.

```rust
// This logic prints "a" then "b" then "c", printing the time in joburg and in
// nyc (in no particular order) when the fetches complete later on.

print("a");
let time_in_joburg = fetch(url: "https://worldtime.com/in/joburg").async;
print("b");
let time_in_nyc = fetch(url: "https://worldtime.com/in/nyc").async;
print("c");

time_in_joburg.then(|time| print("time in joburg: $time"));
time_in_nyc.then(|time| print("time in nyc: $time_in_nyc"));
```

Denim provides a way to "wait" for the `Eventual` values returned by `async`
functions - the `await` keyword. With `await`, working with asynchronous
operations can be quite ergonomic.

```rust
// This logic:
// 1. Starts fetching the time in atlanta
// 2. Prints "a"
// 3. Waits for the time in atlanta to be available before printing it
// 4. Prints "b"

let time_in_atlanta = fetch(url: "https://worldtime.com/in/atlanta").async;

print("a");
print(time_in_atlanta.await);
print("b");
```

There are a garden variety of helpful ways to wait on multiple asynchronous
operations at once in Denim.

```rust
// Tuples are the best way to wait on `Eventual` values with different types.
//
// Any tuple of `Eventual` values has a method called `.after_all()` that waits
// for every `Eventual` to complete.
let (google_icon, _, c_text) = (
  fetch(url: "https://google.com/favicon.ico").async,
  pause(Duration::new(milliseconds: 100)).async,
  readFile(path: "./a/b/c.txt").async,
).after_all().await;

// Arrays of `Eventual` also support `.after_all()`.
let icons = [
  fetch(url: "https://nba.com/favicon.ico").async,
  fetch(url: "https://nhl.com/favicon.ico").async,
  fetch(url: "https://mlb.com/favicon.ico").async,
].after_all().await;

// Both tuples and arrays of `Eventual` support `.race()` to wait for the first
// `Eventual` to complete, wrapping values in `Option::None` to mark `Eventual`
// values that lost the race.
let results = [
  fetch(url: "https://later.com").async,
  fetch(url: "https://now.com").async,
  fetch(url: "https://way.later.com").async,
].race().await;

print("$results"); // Prints "[None, Some(data), None]"
```

#### Interop

TODO(skeswa): flesh this out.

```rust
trait Pluralizer {
  fn pluralize(self, text: string) -> string;
}

extern(go) {
  // Automatically compiles a Denim-friendly interface.
  from(go) "github.com/gertd/go-pluralize@0.2" use {...pluralize};

  struct GoPluralizer {
    client: pluralize.Client,
  }

  impl Pluralizer for GoPluralizer {
    fn pluralize(self, text: string) -> string {
      self.client.plural(text)
    }
  }

  pub fn new_pluralizer() -> impl Pluralizer {
    GoPluralizer { client: pluralize.NewClient() }
  }
}

extern(ts) {
  // @deno-types="npm:@types/pluralize@0.0.29"
  from(ts) "npm:pluralize@8.0.0" use {pluralize};

  struct TsPluralizer {}

  impl Pluralizer for TsPluralizer {
    fn pluralize(self, text: string) -> string {
      pluralize(text)
    }
  }

  pub fn new_pluralizer() -> impl Pluralizer {
    TsPluralizer {}
  }
}
```

#### Testing

TODO(skeswa): flesh this out.

In any "\*.(spec|test).denim" file (spec = unit test, test = integeration test):

`describe`, `before`, `test` are all functions that only apply to tests.

```rust
describe("Something") {
  before {

  }

  test("Something") {

  }

  #[timeout(seconds: 500)]
  test("Something") {

  }

  if !is_dev {
    test("Something") {

    }
  }
}
```

#### Keywords

- Stolen from Rust
  - `as`
  - `async`
    - Suffixable
  - `await`
    - Suffixable
  - `break`
  - `continue`
  - `else`
  - `enum`
  - `extern`
  - `false`
  - `fn`
  - `for`
  - `if`
    - Suffixable
  - `impl`
  - `in`
  - `let`
  - `loop`
  - `match`
    - Suffixable
  - `mod`
  - `pub`
    - `pub(pkg)` instead of `pub(crate)`
  - `return`
  - `self`
  - `Self`
    - Special type
  - `struct`
  - `trait`
  - `true`
  - `type`
  - `use`
  - `where`
  - `while`
    - Suffixable
- Stolen from Dart
  - `show`
  - `try`
    - Suffixable
  - `void`
- Stolen from Python
  - `from`
- Stolen from TypeScript
  - `unknown`
- Originals
  - `fork`
    - Suffixable
