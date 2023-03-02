package denimtesting

// Function used by golden tests that accepts test [input] and returns golden
// test output, or an [error] if something goes wrong.
type GoldenTestFunc = func(input string) (string, error)
