package logger

// Every kind of non-error log message that can be logged.
//
// Most non-error log messages are given a message ID that can be used to set
// the log level for that message. Errors do not get a message ID because you
// cannot turn errors into non-errors (otherwise the build would incorrectly
// succeed). Some internal log messages do not get a message ID because they
// are part of verbose and/or internal debugging output. These messages use
// "MsgID_None" instead.
type MsgID = uint8

const (
	MsgID_None MsgID = iota

	// TODO(skeswa): steal from esbuild's msg_ids.go where appropriate.

	MsgID_END // Keep this at the end (used only for tests)
)
