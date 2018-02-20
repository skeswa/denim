# indigo
A compile-to-go language concept.

```go
from "fmt" import Println
from "time" import Now, Time, Sleep, Second
from "github.com/go/server" import * as server

// Relative imports work as expected.
from "../../hello" as Something

func main() {
  q := Queue<Time>.create()
  q.Enqueue(Now())
  Println(q.Dequeue())
  
  q = Queue<Time>.create()
  q.Enqueue(Now())
  Sleep(3 * Second)
  q.Enqueue(Now())
  Println(q.Dequeue(), q.Dequeue())

  // Function types look like:
  var fn (int, int) => int
  
  // Lambda decls. have type inference!
  fn = (x, y) => x + y
  
  // interface{} becomes dynamic
}

// Rust-style enums! Yay!
enum Transportation<string> {
  Trains
  Cars(year int)
  Planes {
    Airline string
    FlightNumber int
  }
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
  
  // Methods that start with a `*` are only accessible for pointer values.
  *Len() int {
    ~lock.RLock()
    defer ~lock.RUnlock()

    return ~length
  }
  
  *Dequeue() *T {
    ~lock.Lock()
    defer ~lock.Unlock()

    if ~head != nil {
      data := ~head.data
      this.head = ~head.next
      if this.head == nil && ~tail != nil {
        ~tail = nil
      }
      ~length--
      return &data
    }

    return nil
  }
  
  *Enqueue(t *T) {
    ~lock.Lock()
    defer ~lock.Unlock()

    if ~tail == nil {
      ~head = node<T>.create(t)
      ~tail = this.head
    } else {
      ~tail.next = node<T>.create(t)
    }

    ~length++
    ~lock.Unlock()
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
func makeRequest() *int, error {
    // 
    req, fatal!(nil, cause) := http.NewRequest("GET", url, nil)
    // Set cookies if they were passed as argument
    if cookies != "" {
        req.Header.Set("Cookie", cookies)
    }
    
    resp, fatal(nil, (cause) => errors.New("uh oh")) := client.Do(req)
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
