package logger

// A loggable message.
type Msg struct {
	// Primary contents of this message.
	Data MsgData
	// Categorizes this message.
	ID MsgID
	// Describes what kind of message this is.
	Kind MsgKind
	// Metadata and auxialliary details associated with this message.
	Notes []MsgData
	// Plugin that emitted this message.
	PluginName string
}

// Meat-and-potatoes of a `Msg`.
type MsgData struct {
	// ???
	DisableMaximumWidth bool
	// Where the surrounding `Msg` "happened".
	Location *MsgLocation
	// Optional user-specified data that is passed through unmodified
	UserDetail interface{}
	// Textual contents of this `MsgData`.
	Text string
}

// Describes a part of a file referred to by a `Msg`.
type MsgLocation struct {
	// Name of the file to which a `Msg` refers.
	File string
	// ???
	Namespace string
	// Snippet of representative text from the location.
	LineText string
	// A recommendation on what to do about the surrounding `Msg`.
	Suggestion string
	// Line within the file to which the surrounding `Msg` refers.
	Line int // 1-based
	// Column within the file to which the surrounding `Msg` refers.
	Column int // 0-based, in bytes
	// How many characters of the file are referred to by the surrounding `Msg`.
	Length int // in bytes
}
