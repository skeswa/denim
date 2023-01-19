package logger

// Describes a stretch of a file or piece of text.
type Range struct {
	// Length of this `Range`.
	Len int32
	// Inclusive start position of this `Range`.
	Loc Loc
}

// Returns the first index _after_ this `Range`.`
func (r Range) End() int32 {
	return r.Loc.Start + r.Len
}
