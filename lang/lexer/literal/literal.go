package literal

// Wraps `LiteralKind`, adding relevant metadata in some cases.
type Literal struct {
	// Categorizes this [Literal].
	Kind LiteralKind
	// Base of this numeric [Literal].
	//
	// Is only set to a non-zero value if this literal is an [Int] or [Float].
	NumericBase int
	// True if this [Literal] of kind [Float] has an exponent
	// (e.g. the `-23` in `6.02e-23`).
	//
	// Is only set to a non-zero value if this literal is a [Float].
	NumericHasExponent bool
	// True if this [Literal] of kind [Int] does not have any digits
	// (e.g. `0x` or `0b`).
	//
	// Is only set to a non-zero value if this literal is an [Int].
	NumericHasNoDigits bool
}
