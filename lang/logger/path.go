package logger

// Represents both file system paths (Namespace == "file") and abstract module
// paths (Namespace != "file").
//
// Abstract module paths represent "virtual modules" when used for an input file
// and "package paths" when used to represent an external module.
type Path struct {
	// Additional configuration for this `Path`.
	Flags PathFlags

	// This feature was added to support ancient CSS libraries that append things
	// like "?#iefix" and "#icons" to some of their import paths as a hack for IE6.
	// The intent is for these suffix parts to be ignored but passed through to
	// the output. This is supported by other bundlers, so we also support this.
	IgnoredSuffix string

	// Indicates what kind of file this `Path` points to.
	Namespace string

	// String form of this `Path`.
	Text string
}

// Configures a `Path`.
type PathFlags uint8

const (
	// This corresponds to a value of "false' in the "browser" package.json field
	PathDisabled PathFlags = 1 << iota
)

// Returns `true` if this `Path` comes before `b`
func (a Path) ComesBeforeInSortedOrder(b Path) bool {
	return a.Namespace > b.Namespace ||
		(a.Namespace == b.Namespace && (a.Text < b.Text ||
			(a.Text == b.Text && (a.Flags < b.Flags ||
				(a.Flags == b.Flags && a.IgnoredSuffix < b.IgnoredSuffix)))))
}

// Returns `true` if this `Path` is flagged as disabled
func (p Path) IsDisabled() bool {
	return (p.Flags & PathDisabled) != 0
}
