# Overview

Denim is a comfortable, familiar programming language designed for interoperability.

## Pitch

So, you know how pretty much every modern, garbage-collected language feels
eerily like the others lately? I think we can blame this phenomenon on the fact
that many of these languages are converging on the same features and concepts;
how many languages have added first-class functions, co-routines, data classes,
and language-level immutablity recently?

The only _tangible_ differences between one language and another are the
ecosystems and platforms that they can unlock for you. Go gets you into the
cloud and terminal ecosystems, while JS/TS gets you into the browser and to the
edge. Swift and Java get you onto phones, and with C# you can ship on an Xbox.

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

Denim is not designed to be particularly fast, sexy, interesting, or well-suited
for any specific domain. It should fit right into the source code powering any
piece of software - from servers to smart fridges. Denim's guiding design
principles, are to be maximally:

- Easy to reason about and read
- Pleasing to look at and work with
- Quick to learn and manipulate

Denim should never feel as esoteric and ornate as Rust, but it should feel a
smidge more expressive than Go. It should be easy to read, follow, and document
like Java, while getting out of your way and letting you practically solve your
problem like Node.js.

### Inspiration

As Denim is designed to feel familiar, it borrows heavily from some popular
programming languages/runtimes:

- Dependency management from [Deno](https://deno.land/)
- Module system and batteries-included standard library championed by
  [Go](https://go.dev/)
- Syntax largely stolen from [Rust](https://www.rust-lang.org/) with a few
  tricks from [Dart](https://dart.dev/) and [Python](https://www.python.org/)
  included
- Extensibility workflow taken from both [Dart](https://dart.dev/) and
  [Rust](https://www.rust-lang.org/)

**tl;dr** "Dart with Rust syntax and Go's packaging model".

### Compatibility

For Denim to be useful, in needs to be able to interop with most of the major
languages with an established industrial presence. Initially, Denim is being
developed with the following transpilation targets in mind:

1. [TypeScript](https://www.typescriptlang.org/) for web (and also [everything else](https://www.laws-of-software.com/laws/atwood/))
2. [Python](https://www.python.org/) for data science
3. [Swift](https://www.swift.org/) for Apple's ecosystem
4. [Kotlin](https://kotlinlang.org/) for Google's ecosystem

Thereafter, compatibility could be coming to a friendly neighborhood language near you!
