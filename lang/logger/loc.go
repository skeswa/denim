package logger

// Points to a spot in a file or piece of text.
type Loc struct {
	// This is the 0-based index of this location from the start of the file, in bytes
	Start int32
}
