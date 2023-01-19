package logger

// Describes the verbosity of logging.
type LogLevel int8

const (
	// Turns all logging off.
	LevelNone LogLevel = iota
	// Most verbose level of logging.
	LevelVerbose
	// 2nd-most verbose level of logging.
	LevelDebug
	// 3rd-most verbose level of logging.
	LevelInfo
	// 3rd-least verbose level of logging.
	LevelWarning
	// 2nd-least verbose level of logging.
	LevelError
	// Least verbose level of logging.
	LevelSilent
)
