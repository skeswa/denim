# indigo
A compile-to-go language concept.

```go
import (
  "fmt" as fmt
  "sync" as sync
  "time" as time

  "github.com/go/server" as server
)

func main() {
  q! := Queue<*time.Time>.create()
  q.Enqueue(time.Now())
  fmt.Println(q.Dequeue())
  
  q = Queue<*time.Time>.create()
  q.Enqueue(time.Now())
  time.Sleep(3 * time.Second)
  q.Enqueue(time.Now())
  fmt.Println(q.Dequeue(), q.Dequeue())
}

type Queue<*T> struct {
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
      this.head = node<T>.create(*t)
      this.tail = this.head
    } else {
      this.tail.next = node<T>.create(*t)
    }

    this.length++
    this.lock.Unlock()
  }
}

type node<T> struct {
  data T
  next *node<T>!
  
  static create(t T) *node<T> {
    return &node<T>{data: t}
  }
}


```
