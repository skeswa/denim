package logger

// Logs things.
//
// Logging is either done to stderr (via "NewStderrLog") or to an in-memory
// array (via "NewDeferLog"). In-memory arrays are used to capture messages
// from parsing individual files because during incremental builds, log
// messages for a given file can be replayed from memory if the file ends up
// not being reparsed.
//
// Errors are streamed asynchronously as they happen, each error contains the
// contents of the line with the error, and the error count is limited by
// default.
type Log struct {
	// ???
	AddMsg func(Msg)
	// ???
	HasErrors func() bool

	// This is called after the build has finished but before writing to stdout.
	// It exists to ensure that deferred warning messages end up in the terminal
	// before the data written to stdout.
	AlmostDone func()

	// ???
	Done func() []Msg

	// ???
	Level LogLevel
	// ???
	Overrides map[MsgID]LogLevel
}

// Adds an error to the log.
func (log Log) AddError(tracker *LineColumnTracker, r Range, text string) {
	log.AddMsg(Msg{
		Kind: Error,
		Data: tracker.MsgData(r, text),
	})
}

// Adds an error with additional commentary to the log.
func (log Log) AddErrorWithNotes(tracker *LineColumnTracker, r Range, text string, notes []MsgData) {
	log.AddMsg(Msg{
		Kind:  Error,
		Data:  tracker.MsgData(r, text),
		Notes: notes,
	})
}
