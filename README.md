# esper
A programming language designed for cross-environment business logic.

## Pitch

So, you know how pretty much each modern, garbage-collected language feels eerily like the others lately? I think it is because they are converging on the same features and concepts; they all have first-class functions, co-routines, and data classes now. The only difference between one of these languages and another _really_ is the ecosystems and platforms that they can hook into. Go gets you into the cloud and terminal ecosystems, while JS/TS gets you into the browser and to the edge. Swift and Kotlin get you onto phones, and with C# you can ship on an Xbox.

And it got me thinking: **if the languages we use to write our apps are this similar, why on earth are we writing the same logic over and over again**. Why can't we write most of our logic, constants, and types once, and important them anywhere? What if there was a language purely design to be imported in other languages?

Esper is that language.

The intent behind Esper is to incorporate the smallest set of common features from these garbage-collected languages sufficient to:
- Create common types
- Implement business logic
- Declare common constants

Of course, it wouldn't hurt to end up with a language that is pleasant to use and maintain while we're at it.

## Design

Esper is not designed to be fast, or sexy, or interesting, or well-suited for any specific domain. It should fit right into the source coce powering any ol' user interface, backend API, and smart fridge. Esper's guiding design principles, are to be:

- Simple,
- Familar, and
- Practical

Esper should never feel as esoteric and ornate as Rust, but it should feel a smidge more expressive than Go. It should be easy to read, follow, and document like Java; getting out of your way and letting you solve the damn problem like Node.js.

Described above is language that will be difficult to design, and even harder to implement. My hope in all of this, at the very least, is to move the [Overton window](https://en.wikipedia.org/wiki/Overton_window) in a direction that we bet the programming world would enjoy.

Wish me luck.

### Lineage



```dart
// No more `library`.
// No more `'package:'` in front of imports.
// Explicit imports by default.
// Falls back to importing `index.dart` when no file is specified.
// No more importing specific dart files from dart packages.

from 'flutter/material' import StatefulWidget

// Absolute path support. Import * without an `as` puts all the exports of '/helpers' in this file's namespace.
from '/helpers' import *
// Relative path support.
from '../some/helpers' import * as someHelpers

// Duck type interfaces.
interface Thing {
  height?: int
  act(): Future<void>
}

interface SuperBigThing extends Super, Big, Thing {
  someOtherProperty(): void
}

// Value classes.
struct Person {
  id: num
  name?: String
}

struct State {
  height?: int
  owner?: Person
  width: int
}

let _x = State { width: 12 }
let _y = State { ..._x, height: 14 }
let mut _z = State { width: 16, ..._x }
_z.width = 17

let { height, width } = _y

// Rust enums
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Option<T> {
    Some(T),
    None,
}

// Rust pattern matching
match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
}

// Rust macros
// TODO!

// No more `get`. No more `set`. All classes are abstract by default.
class SomeClass<T implements Thing> extends SomeBaseClass<T> {
  // Class fields are immutable by default - you need `mut` to make them mutable.
  mut _state: State

  // Constructors look like this now.
  // No more `const` constructors - the compiler will decide if something can be `const`.
  new(t: T, { requiredOptions: Map<String, int>, optionalOptions?: Map<String, int> }) {
    // Initialize state here.
  }
  
  /// Dartdoc still looks like this, but [] will be smarter:
  /// - [a] refers to the a 
  static async method(a: bool, b: int, c: num => num): Future<num> {
    return a ? b : c(12)
  }
  
  abstract doAThing(): bool
}

fn main() {
}
```
