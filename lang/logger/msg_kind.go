package logger

// Enumerates every kind of `Msg`.
type MsgKind uint8

const (
	// `MsgKind` used for severe error messages.
	Error MsgKind = iota
	// `MsgKind` used for less-than-severe error messages.
	Warning
	// `MsgKind` used for "FYI"-style messages.
	Info
	// `MsgKind` used for auxilliary details.
	Note
	// `MsgKind` used for messages that are helpful for debugging.
	Debug
	// `MsgKind` used for super noisy, details and metadata.
	Verbose
)

// Represents this `MsgKind` as a string.
func (kind MsgKind) String() string {
	switch kind {
	case Error:
		return "ERROR"
	case Warning:
		return "WARNING"
	case Info:
		return "INFO"
	case Note:
		return "NOTE"
	case Debug:
		return "DEBUG"
	case Verbose:
		return "VERBOSE"
	default:
		panic("Internal error")
	}
}

// Represents this `MsgKind` as a UTF-8 icon.
func (kind MsgKind) Icon() string {
	switch kind {
	case Error:
		return "✘"
	case Warning:
		return "▲"
	case Info:
		return "▶"
	case Note:
		return "→"
	case Debug:
		return "●"
	case Verbose:
		return "⬥"
	default:
		panic("Internal error")
	}
}
