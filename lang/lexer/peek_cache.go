package lexer

// Circular buffer that keeps track of the next few runes in the lexer's source.
//
// Importantly, [peekCache] is a queue - runes are removed from the front of it,
// and added to the back of it. Runes are removed when the lexer visits them,
// and are added when the lexer looks ahead.
type peekCache struct {
	// Buffer of upcoming runes.
	runes []rune
	// Buffer of the byte widths of upcoming runes.
	runeWidths []int
	// Length of [runes] and [runeWidths].
	size int
	// Position of the starting point of the circular buffer window.
	windowIndex int
	// How many upcoming runes are currently cached by this circular buffer.
	windowLength int
}

// Creates a new [peekCache] with the specified [size].
func newPeekCache(size int) peekCache {
	return peekCache{
		runes:      make([]rune, size),
		runeWidths: make([]int, size),
		size:       size,
	}
}

// Reads the rune at offset [i] in the cache, returning the rune, and true if it
// exists.
//
// Returns false in the last return value slot if the rune does not exist.
func (p *peekCache) at(i int) (rune, bool) {
	if i >= p.windowLength {
		return 0, false
	}

	index := (p.windowIndex + i) % p.size

	return p.runes[index], true
}

// Removes the first cached rune, returning it, its width, and true if the
// operation was successful.
//
// Returns false in the last return value slot if this operation is
// unsuccessful.
func (p *peekCache) dequeue() (rune, int, bool) {
	if p.windowLength <= 0 {
		return 0, 0, false
	}

	index := p.windowIndex % p.size
	r := p.runes[index]
	rWidth := p.runeWidths[index]

	p.windowIndex += 1
	p.windowLength -= 1

	return r, rWidth, true
}

// Adds a rune to the end of the cache, returning true if the operation was
// successful.
//
// Returns false if this operation is unsuccessful.
func (p *peekCache) enqueue(r rune, rWidth int) bool {
	if p.windowLength >= p.size {
		return false
	}

	index := (p.windowIndex + p.windowLength) % p.size
	p.runes[index] = r
	p.runeWidths[index] = rWidth
	p.windowLength += 1

	return true
}

// Returns the first uncached, in-source rune offset and byte offset
// (in that order).
func (p *peekCache) frontier() (int, int) {
	if p.windowLength <= 0 {
		return 0, 0
	}

	var (
		index      int
		byteOffset int
	)

	for i := 0; i < p.windowLength; i++ {
		index = (p.windowIndex + i) % p.size

		byteOffset += p.runeWidths[index]
	}

	return p.windowLength, byteOffset
}
