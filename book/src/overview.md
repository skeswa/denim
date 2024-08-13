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
- [Java](https://www.java.com/) for Android and legacy codebases
- [Python](https://www.python.org/) for Data Science
- [Swift](https://www.swift.org/) for Apple's ecosystem
- [TypeScript](https://www.typescriptlang.org/) for Web

Prototyping will likely focus on **Go** and **TypeScript** because they are
sufficiently, popular, and generally useful.

Thereafter, since I suspect that Denim's "killer app" will be code sharing
between client-side and server-side codebases, it would make sense to target
**Swift** and **Java**.
