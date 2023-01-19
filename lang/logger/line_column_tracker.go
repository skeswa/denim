package logger

import "unicode/utf8"

// A "bookmark" of sorts that helps us remember where we left off while scanning
// for warnings.
//
// It's not common for large files to have many warnings. But when it happens,
// we want to make sure that it's not too slow. Source code locations are
// represented as byte offsets for compactness but transforming these to
// line/column locations for warning messages requires scanning through the
// file. A naive approach for this would cause O(n^2) scanning time for n
// warnings distributed throughout the file.
//
// Warnings are typically generated sequentially as the file is scanned. So
// one way of optimizing this is to just start scanning from where we left
// off last time instead of always starting from the beginning of the file.
// That's what this object does.
//
// Another option could be to eagerly populate an array of line/column offsets
// and then use binary search for each query. This might slow down the common
// case of a file with only at most a few warnings though, so think before
// optimizing too much. Performance in the zero or one warning case is by far
// the most important.
type LineColumnTracker struct {
	// ???
	contents string
	// ???
	hasLineEnd bool
	// ???
	hasLineStart bool
	// ???
	hasSource bool
	// ???
	line int32
	// ???
	lineEnd int32
	// ???
	lineStart int32
	// ???
	offset int32
	// ???
	prettyPath string
}

// Returns a new instance of `LineColumnTracker` for the provided `source`.
func MakeLineColumnTracker(source *Source) LineColumnTracker {
	if source == nil {
		return LineColumnTracker{
			hasSource: false,
		}
	}

	return LineColumnTracker{
		contents:     source.Contents,
		prettyPath:   source.PrettyPath,
		hasLineStart: true,
		hasSource:    true,
	}
}

// Creates a new `MsgData` out of `text` referring to the specified range `r`.
func (tracker *LineColumnTracker) MsgData(r Range, text string) MsgData {
	return MsgData{
		Text:     text,
		Location: tracker.MsgLocationOrNil(r),
	}
}

// Tries to return a `MsgLocation` for the given range `r`, returning `nil` if
// that isn't possible for whatever reason.
func (tracker *LineColumnTracker) MsgLocationOrNil(r Range) *MsgLocation {
	if tracker == nil || !tracker.hasSource {
		return nil
	}

	// Convert the index into a line and column number
	lineCount, columnCount, lineStart, lineEnd := tracker.computeLineAndColumn(int(r.Loc.Start))

	return &MsgLocation{
		File:     tracker.prettyPath,
		Line:     lineCount + 1, // 0-based to 1-based
		Column:   columnCount,
		Length:   int(r.Len),
		LineText: tracker.contents[lineStart:lineEnd],
	}
}

// Represents the given `offset` as a line-column pair within the source of this
// tracker.
func (t *LineColumnTracker) computeLineAndColumn(offset int) (lineCount int, columnCount int, lineStart int, lineEnd int) {
	t.scanTo(int32(offset))

	// Scan for the start of the line
	if !t.hasLineStart {
		contents := t.contents
		i := t.offset
		for i > 0 {
			r, size := utf8.DecodeLastRuneInString(contents[:i])
			if r == '\n' || r == '\r' || r == '\u2028' || r == '\u2029' {
				break
			}
			i -= int32(size)
		}
		t.hasLineStart = true
		t.lineStart = i
	}

	// Scan for the end of the line
	if !t.hasLineEnd {
		contents := t.contents
		i := t.offset
		n := int32(len(contents))
		for i < n {
			r, size := utf8.DecodeRuneInString(contents[i:])
			if r == '\n' || r == '\r' || r == '\u2028' || r == '\u2029' {
				break
			}
			i += int32(size)
		}
		t.hasLineEnd = true
		t.lineEnd = i
	}

	return int(t.line), offset - int(t.lineStart), int(t.lineStart), int(t.lineEnd)
}

// Moves forward `offset` indices.
func (t *LineColumnTracker) scanTo(offset int32) {
	contents := t.contents
	i := t.offset

	// Scan forward
	if i < offset {
		for {
			r, size := utf8.DecodeRuneInString(contents[i:])
			i += int32(size)

			switch r {
			case '\n':
				t.hasLineStart = true
				t.hasLineEnd = false
				t.lineStart = i
				if i == int32(size) || contents[i-int32(size)-1] != '\r' {
					t.line++
				}

			case '\r', '\u2028', '\u2029':
				t.hasLineStart = true
				t.hasLineEnd = false
				t.lineStart = i
				t.line++
			}

			if i >= offset {
				t.offset = i
				return
			}
		}
	}

	// Scan backward
	if i > offset {
		for {
			r, size := utf8.DecodeLastRuneInString(contents[:i])
			i -= int32(size)

			switch r {
			case '\n':
				t.hasLineStart = false
				t.hasLineEnd = true
				t.lineEnd = i
				if i == 0 || contents[i-1] != '\r' {
					t.line--
				}

			case '\r', '\u2028', '\u2029':
				t.hasLineStart = false
				t.hasLineEnd = true
				t.lineEnd = i
				t.line--
			}

			if i <= offset {
				t.offset = i
				return
			}
		}
	}
}
