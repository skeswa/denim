package text

// Reference to some stretch of text.
type Span struct {
	// String that contains this `Span`.
	Text string
	// Information about where this `Span` lives within `Text`.
	Range Range
}
