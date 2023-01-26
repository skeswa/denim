package logger

import (
	"strings"
	"unicode/utf8"

	"github.com/skeswa/denim/lang/text"
)

// Represents a single, relevant source file.
type Source struct {
	// Stringified contents of this source file.
	Contents string

	// Identifier that is mixed in to automatically-generated symbol names to
	// improve readability.
	//
	// For example, if the identifier is "util" then the symbol for an
	// "export default" statement will be called "util_default".
	IdentifierName string

	// Ordinal used to keep track of this source file's position in the build
	// relative to the other source files.
	Index uint32

	// Path used as a key to uniquely identify this source file.
	//
	// This path should never be shown to the user (e.g. never print this to the
	// terminal).
	//
	// If it's marked as an absolute path, it's a platform-dependent path that
	// includes environment-specific things such as Windows backslash path
	// separators and potentially the user's home directory. Only use this for
	// passing to syscalls for reading and writing to the file system. Do not
	// include this in any output data.
	//
	// If it's marked as not an absolute path, it's an opaque string that is used
	// to refer to an automatically-generated module.
	KeyPath Path

	// Prettified path used for error messages and the metadata JSON file.
	//
	// This is a mostly platform-independent path. It's relative to the current
	// working directory and always uses standard path separators. Use this for
	// referencing a file in all output data. These paths still use the original
	// case of the path so they may still work differently on file systems that
	// are case-insensitive vs. case-sensitive.
	PrettyPath string
}

// ???
func (s *Source) CommentTextWithoutIndent(r text.Range) string {
	text := s.Contents[r.Loc.Start:r.End()]
	if len(text) < 2 || !strings.HasPrefix(text, "/*") {
		return text
	}
	prefix := s.Contents[:r.Loc.Start]

	// Figure out the initial indent
	indent := 0
seekBackwardToNewline:
	for len(prefix) > 0 {
		c, size := utf8.DecodeLastRuneInString(prefix)
		switch c {
		case '\r', '\n', '\u2028', '\u2029':
			break seekBackwardToNewline
		}
		prefix = prefix[:len(prefix)-size]
		indent++
	}

	// Split the comment into lines
	var lines []string
	start := 0
	for i, c := range text {
		switch c {
		case '\r', '\n':
			// Don't double-append for Windows style "\r\n" newlines
			if start <= i {
				lines = append(lines, text[start:i])
			}

			start = i + 1

			// Ignore the second part of Windows style "\r\n" newlines
			if c == '\r' && start < len(text) && text[start] == '\n' {
				start++
			}

		case '\u2028', '\u2029':
			lines = append(lines, text[start:i])
			start = i + 3
		}
	}
	lines = append(lines, text[start:])

	// Find the minimum indent over all lines after the first line
	for _, line := range lines[1:] {
		lineIndent := 0
		for _, c := range line {
			if c != ' ' && c != '\t' {
				break
			}
			lineIndent++
		}
		if indent > lineIndent {
			indent = lineIndent
		}
	}

	// Trim the indent off of all lines after the first line
	for i, line := range lines {
		if i > 0 {
			lines[i] = line[indent:]
		}
	}
	return strings.Join(lines, "\n")
}

// ???
func (s *Source) LocBeforeWhitespace(loc text.Loc) text.Loc {
	for loc.Start > 0 {
		c, width := utf8.DecodeLastRuneInString(s.Contents[:loc.Start])
		if c != ' ' && c != '\t' && c != '\r' && c != '\n' {
			break
		}
		loc.Start -= int32(width)
	}
	return loc
}

// ???
func (s *Source) RangeOfLegacyOctalEscape(loc text.Loc) (r text.Range) {
	txt := s.Contents[loc.Start:]
	r = text.Range{Loc: loc, Len: 0}

	if len(txt) >= 2 && txt[0] == '\\' {
		r.Len = 2
		for r.Len < 4 && int(r.Len) < len(txt) {
			c := txt[r.Len]
			if c < '0' || c > '9' {
				break
			}
			r.Len++
		}
	}
	return
}

// ???
func (s *Source) RangeOfOperatorAfter(loc text.Loc, op string) text.Range {
	txt := s.Contents[loc.Start:]
	index := strings.Index(txt, op)
	if index >= 0 {
		return text.Range{Loc: text.Loc{Start: loc.Start + int32(index)}, Len: int32(len(op))}
	}
	return text.Range{Loc: loc}
}

// ???
func (s *Source) RangeOfOperatorBefore(loc text.Loc, op string) text.Range {
	txt := s.Contents[:loc.Start]
	index := strings.LastIndex(txt, op)
	if index >= 0 {
		return text.Range{Loc: text.Loc{Start: int32(index)}, Len: int32(len(op))}
	}
	return text.Range{Loc: loc}
}

// ???
func (s *Source) RangeOfNumber(loc text.Loc) (r text.Range) {
	txt := s.Contents[loc.Start:]
	r = text.Range{Loc: loc, Len: 0}

	if len(txt) > 0 {
		if c := txt[0]; c >= '0' && c <= '9' {
			r.Len = 1
			for int(r.Len) < len(txt) {
				c := txt[r.Len]
				if (c < '0' || c > '9') && (c < 'a' || c > 'z') && (c < 'A' || c > 'Z') && c != '.' && c != '_' {
					break
				}
				r.Len++
			}
		}
	}
	return
}

// ???
func (s *Source) RangeOfString(loc text.Loc) text.Range {
	txt := s.Contents[loc.Start:]
	if len(txt) == 0 {
		return text.Range{Loc: loc, Len: 0}
	}

	quote := txt[0]
	if quote == '"' || quote == '\'' {
		// Search for the matching quote character
		for i := 1; i < len(txt); i++ {
			c := txt[i]
			if c == quote {
				return text.Range{Loc: loc, Len: int32(i + 1)}
			} else if c == '\\' {
				i += 1
			}
		}
	}

	if quote == '`' {
		// Search for the matching quote character
		for i := 1; i < len(txt); i++ {
			c := txt[i]
			if c == quote {
				return text.Range{Loc: loc, Len: int32(i + 1)}
			} else if c == '\\' {
				i += 1
			} else if c == '$' && i+1 < len(txt) && txt[i+1] == '{' {
				break // Only return the range for no-substitution template literals
			}
		}
	}

	return text.Range{Loc: loc, Len: 0}
}

// ???
func (s *Source) TextForRange(r text.Range) string {
	return s.Contents[r.Loc.Start : r.Loc.Start+r.Len]
}
