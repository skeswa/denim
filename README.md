# indigo
A compile-to-go language concept.

```go
import { Println } from "fmt"
import { Now, Time, Sleep, Second } from "time"
import * as server from "github.com/go/server"

func main() {
  q! := Queue<Time>.create()
  q.Enqueue(Now())
  Println(q.Dequeue())
  
  q = Queue<Time>.create()
  q.Enqueue(Now())
  Sleep(3 * Second)
  q.Enqueue(Now())
  Println(q.Dequeue(), q.Dequeue())
  
  // Lambda!
  x -> Println(x)
  (x, y) => x + y
  
  // interface{} becomes dynamic
}

enum Woop {
  Woooop
  Wooooooop
  Wooooooooooooop
}

enum Transportation<string> {
  Trains = "trains"
  Cars = "cars"
  Planes = "planes"
}

interface Enqueueable<T> {
  Len() int
  Enqueue(t T)
}

struct Color {
  Hex string
}

enum Colors<Color> {
  Red = Color{Hex: "#ff0000"}
  Blue = Color{Hex: "#0000ff"}
  Green = Color{Hex: "#00ff00"}
}

struct Queue<T> {
  lock *sync.RWMutex
  head!, tail! *node<T>
  length! int
  
  static Create() *Queue {
    return &Queue{lock: &sync.RWMutex{}}
  }
  
  *Len() int {
    this.lock.RLock()
    defer this.lock.RUnlock()

    return this.length
  }
  
  *Dequeue() *T {
    this.lock.Lock()
    defer this.lock.Unlock()

    if this.head != nil {
      data := this.head.data
      this.head = this.head.next
      if this.head == nil && this.tail != nil {
        this.tail = nil
      }
      this.length--
      return &data
    }

    return nil
  }
  
  *Enqueue(t *T) {
    this.lock.Lock()
    defer this.lock.Unlock()

    if this.tail == nil {
      this.head = node<T>.create(t)
      this.tail = this.head
    } else {
      this.tail.next = node<T>.create(t)
    }

    this.length++
    this.lock.Unlock()
  }
}

struct node<T> {
  data T
  next *node<T>!

  static create(t T) *node<T> {
    return &node<T>{data: t}
  }
}


```
