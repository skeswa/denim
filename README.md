# indigo
A compile-to-go language concept.

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
