package source

import (
	"unicode/utf8"
)

// Iterates through the runes of a [source] string.
type Cursor struct {
	// Rune at this [Cursor]'s current byte-aligned position within the
	// [source] string.
	//
	// Begins at a negative value.
	currentRune rune
	// How many runes have come before [currentRune].
	//
	// Begins at zero.
	currentRuneIndex int
	// Length of [peekCacheNextRuneByteOffsets] and [peekCacheRunes].
	//
	// We don't use `len(peekCacheRunes)` because it is [very slightly less
	// optimal](https://stackoverflow.com/questions/26634554/go-multiple-len-calls-vs-performance).
	furthestPeek int
	// Byte-aligned position of the rune following [currentRune].
	//
	// Begins at zero.
	nextRuneByteOffset int
	// Corresponding byte offsets within the [source] string for the rune
	// **after** each rune in [peekCacheRunes].
	//
	// We use a slice instead of a map for a reduced memory footprint.
	peekCacheNextRuneByteOffsets []int
	// Corresponding rune-aligned positions within the [source] string for each
	// rune in [peekCacheRunes].
	//
	// We use a slice instead of a map for a reduced memory footprint.
	peekCacheRuneIndices []int
	// Circular buffer used to cache the runes ahead of [currentRune].
	//
	// We use a slice instead of a map for a reduced memory footprint.
	peekCacheRunes []rune
	// String through which this [Cursor] iterates.
	sourceString string
}

// Creates a brand new [Cursor] that will iterate through the runes of the
// specified [sourceString].
//
// [furthestPeek] is the maximum number of runes ahead of the current one that
// can be queried.
func NewCursor(furthestPeek int, sourceString string) *Cursor {
	var (
		peekCacheNextRuneByteOffsets = make([]int, furthestPeek)
		peekCacheRuneIndices         = make([]int, furthestPeek)
		peekCacheRunes               = make([]rune, furthestPeek)
	)

	// Make sure that the peek cache starts with the correct "empty" default
	// state. `make` would have the default values be zeroes.
	for i := 0; i < furthestPeek; i++ {
		peekCacheRuneIndices[i] = -1
		peekCacheRunes[i] = cacheMissRune
	}

	return &Cursor{
		currentRune:                  preSourceRune,
		currentRuneIndex:             -1,
		furthestPeek:                 furthestPeek,
		peekCacheNextRuneByteOffsets: peekCacheNextRuneByteOffsets,
		peekCacheRuneIndices:         peekCacheRuneIndices,
		peekCacheRunes:               peekCacheRunes,
		sourceString:                 sourceString,
	}
}

// Advances sequentially to the next rune of the source string, returning it and
// a bool that is false if such a rune does not exist.
func (c *Cursor) NextRune() (rune, bool) {
	// Return early if this [Cursor] cannot possibly have a next rune.
	if c.currentRune < preSourceRune {
		return c.currentRune, false
	}

	var (
		hasNewCurrentRune     = true
		newCurrentRune        = cacheMissRune
		newNextRuneByteOffset = c.nextRuneByteOffset
	)

	// The "new" current rune will immediately follow the "old" current rune, as
	// this method advances the rune cursor by one.
	newCurrentRuneIndex := c.currentRuneIndex + 1

	// Steal the new current rune from the peek cache if possible; Otherwise, we
	// need to do some utf-8 decoding that has already been done.
	if cachedRune, cachedRuneByteOffset := c.cachedRuneAt(newCurrentRuneIndex); newCurrentRune != cacheMissRune {
		newCurrentRune = cachedRune
		newNextRuneByteOffset = cachedRuneByteOffset
	} else {
		// Looks like the peek cache doesn't have the rune we're looking for, so we
		// need to decode it from the source string.
		decodedRune, decodedRuneWidth := utf8.DecodeRuneInString(c.sourceString[c.nextRuneByteOffset:])

		if decodedRuneWidth > 0 {
			newCurrentRune = decodedRune
			newNextRuneByteOffset += decodedRuneWidth
		} else {
			// A width of `0` indicates the end of the source string. This also means
			// that there is no new current rune.
			hasNewCurrentRune = false
			newCurrentRune = postSourceRune
		}
	}

	c.currentRune = newCurrentRune
	c.currentRuneIndex = newCurrentRuneIndex
	c.nextRuneByteOffset = newNextRuneByteOffset

	return newCurrentRune, hasNewCurrentRune
}

// Looks ahead of the current rune by the specified [runeOffset], returning the
// rune at that position.
//
// NOTE: [runeOffset] must not be greater than the "furthest peek" passed to
// this [Cursor] when it was created.
func (c *Cursor) PeekRune(runeOffset int) (rune, bool) {
	// Return early if this [Cursor] cannot possibly have a next rune.
	if c.currentRune < preSourceRune {
		return c.currentRune, false
	}

	// Peeking zero always refers to the current rune.
	if runeOffset <= 0 {
		currentRune := c.currentRune

		return currentRune, currentRune >= 0
	}

	runeOffsetIndex := c.currentRuneIndex + runeOffset

	// Check to see if the rune being peeked at is in the cache; if so, return
	// quickly without doing any utf-8 decoding.
	if cachedRune, _ := c.cachedRuneAt(runeOffsetIndex); cachedRune != cacheMissRune {
		return cachedRune, cachedRune > 0
	}

	// Looks like we need to do some good ol' fashioned unicoding to look ahead of
	// the current rune. Let's cache our findings along the way to avoid duplicate
	// work.

	var (
		currentRune rune

		currentRuneIndex       = c.currentRuneIndex + 1
		lastCacheableRuneIndex = c.currentRuneIndex + c.furthestPeek
		nextRuneByteOffset     = c.nextRuneByteOffset
	)

	for ; currentRuneIndex <= runeOffsetIndex; currentRuneIndex++ {
		// Check to see if the peeked rune is already in the cache. If so, we can
		// just skip ahead.
		if cachedRune, cachedNextRuneByteOffset := c.cachedRuneAt(currentRuneIndex); cachedRune != cacheMissRune {
			currentRune = cachedRune
			nextRuneByteOffset = cachedNextRuneByteOffset

			continue
		}

		if decodedRune, decodedRuneWidth := utf8.DecodeRuneInString(c.sourceString[nextRuneByteOffset:]); decodedRuneWidth > 0 {
			currentRune = decodedRune
			nextRuneByteOffset += decodedRuneWidth

			if currentRuneIndex < lastCacheableRuneIndex {
				c.cacheRune(currentRuneIndex, currentRune, nextRuneByteOffset)
			}
		} else {
			// A width of `0` indicates the end of the source string. There is no
			// sense in continuing, let's exit early.
			return postSourceRune, false
		}
	}

	return currentRune, true
}

const (
	// Rune used to signal that the peek cache has an empty slot.
	cacheMissRune rune = -3
	// Rune used to signal that the [Cursor] has reached the end of its
	// input.
	postSourceRune rune = -2
	// Rune used to signal that the [Cursor] has not yet started iterating.
	preSourceRune rune = -1
)

// Returns in-peek-cache rune and its byte offset at the specified [runeIndex].
func (c *Cursor) cachedRuneAt(runeIndex int) (rune, int) {
	if runeIndexWithinCache := c.withinCache(runeIndex); runeIndexWithinCache >= 0 && runeIndex == c.peekCacheRuneIndices[runeIndexWithinCache] {
		return c.peekCacheRunes[runeIndexWithinCache], c.peekCacheNextRuneByteOffsets[runeIndexWithinCache]
	}

	return cacheMissRune, 0
}

// Caches a rune at the specified [runeIndex], returning true if caching was
// successful.
func (c *Cursor) cacheRune(runeIndex int, runeToCache rune, nextRuneByteOffset int) bool {
	if runeIndexWithinCache := c.withinCache(runeIndex); runeIndexWithinCache >= 0 {
		c.peekCacheNextRuneByteOffsets[runeIndexWithinCache] = nextRuneByteOffset
		c.peekCacheRuneIndices[runeIndexWithinCache] = runeIndex
		c.peekCacheRunes[runeIndexWithinCache] = runeToCache

		return true
	}

	return false
}

// Returns the in-cache version of [runeIndex], or a negative value if no such
// index exists.
func (c *Cursor) withinCache(runeIndex int) int {
	if runeIndex < 0 {
		return -1
	}

	return runeIndex % c.furthestPeek
}
