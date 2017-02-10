# indigo
A compile-to-go language concept.

```go
import { Println } from "fmt"
import { Now, Time, Sleep, Second } from "time"
import * as server from "github.com/go/server"

func main() {
  q := Queue<Time>.create()
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
  head, tail *node<T>
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
  next *node<T>

  static create(t T) *node<T> {
    return &node<T>{data: t}
  }
}


```

better error handling:
```go
func makeRequest() error {  
    req, err := http.NewRequest("GET", url, nil)
    if err != nil {
        return fmt.Errorf("failed to make request: %v", err)
    }

    // Set cookies if they were passed as argument
    if cookies != "" {
        req.Header.Set("Cookie", cookies)
    }

    // Send request
    resp, err := client.Do(req)
    if err != nil {
        return fmt.Errorf("failed to make request: %v", err)
    }
    // Save response body into data variable
    data, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        return fmt.Errorf("failed to make request: %v", err)
    }

    // If lastData is equal to "", it means that it is 
    // the first request and we set lastData to current 
    // response body
    // Otherwise, we compare previous and current HTML
    if lastData == "" {
        lastData = string(data)
    } else {
        checkChanges(string(data))
    }
    
    return nil
}
```
becomes:
```go
func makeRequest() error {  
    req, #err := http.NewRequest("GET", url, nil)
    // Set cookies if they were passed as argument
    if cookies != "" {
        req.Header.Set("Cookie", cookies)
    }
    // Send request
    resp, #err := client.Do(req)
    // Save response body into data variable
    data, #otherErr := ioutil.ReadAll(resp.Body)
    
    // Handle the errors
    #err(err error) {
      return fmt.Errorf("look how much better this is: %v", err)
    }
    #otherErr(err error) {
      return fmt.Errorf("look how much better this is: %v", err)
    }

    // If lastData is equal to "", it means that it is 
    // the first request and we set lastData to current 
    // response body
    // Otherwise, we compare previous and current HTML
    if lastData == "" {
        lastData = string(data)
    } else {
        checkChanges(string(data))
    }
    
    return nil
}
```
