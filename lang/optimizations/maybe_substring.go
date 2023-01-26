package optimizations

// Represents a string that is maybe a substring of the current file's
// "source.Contents" string; the point of doing this is that if it is a
// substring (the common case), then we can represent it more efficiently.
//
// For compactness and performance, the AST represents identifiers as a symbol
// reference instead of as a string. However, we need to track the string
// between the first pass and the second pass because the string is only
// resolved to a symbol in the second pass. To avoid allocating extra memory
// to store the string, we instead use an index+length slice of the original JS
// source code. That index is what "Start" represents here. The length is just
// "len(String)".
//
// Set "Start" to invalid (the zero value) if "String" is not a substring of
// "source.Contents". This is the case for escaped identifiers. For example,
// the identifier "fo\u006f" would be "MaybeSubstring{String: "foo"}". It's
// critical that any code changing the "String" also set "Start" to the zero
// value, which is best done by just overwriting the whole "MaybeSubstring".
//
// The substring range used to be recovered automatically from the string but
// that relied on the Go "unsafe" package which can hypothetically break under
// certain Go compiler optimization passes, so it has been removed and replaced
// with this more error-prone approach that doesn't use "unsafe".
type MaybeSubstring struct {
	// Contents of the substring.
	String string
	// First index of the substring.
	//
	// Invalid value (the zero value) if `String` is not a substring of
	// `source.Contents`.
	Start Index32
}
