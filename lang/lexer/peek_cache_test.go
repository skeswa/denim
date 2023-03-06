package lexer

import "testing"

func TestATypicalPeekScenario(t *testing.T) {
	peekCache := newPeekCache(3)

	if _, _, ok := peekCache.dequeue(); ok {
		t.Error("ok should be false - there is nothing in the peek cache")
	}

	if ok := peekCache.enqueue('🤠', 4); !ok {
		t.Error("ok should be true - there is enough space left")
	}
	if ok := peekCache.enqueue('😍', 2); !ok {
		t.Error("ok should be true - there is enough space left")
	}
	if ok := peekCache.enqueue('😭', 1); !ok {
		t.Error("ok should be true - there is enough space left")
	}
	if ok := peekCache.enqueue('🤑', 1); ok {
		t.Error("ok should be false - there is no space left")
	}

	if runeOffset, byteOffset := peekCache.frontier(); runeOffset != 3 || byteOffset != 4+2+1 {
		t.Errorf(
			"frontier should not return (%d, %d)",
			runeOffset, byteOffset,
		)
	}

	if nextRune, ok := peekCache.at(1); nextRune != '😍' || !ok {
		t.Errorf(
			"at should not return (%c, %v)",
			nextRune, ok,
		)
	}
	if _, ok := peekCache.at(3); ok {
		t.Error("ok should be false - index is out of bounds")
	}
	if _, ok := peekCache.at(999); ok {
		t.Error("ok should be false - index is out of bounds")
	}
	if nextRune, ok := peekCache.at(2); nextRune != '😭' || !ok {
		t.Errorf(
			"at should not return (%c, %v)",
			nextRune, ok,
		)
	}

	if nextRune, nextRuneWidth, ok := peekCache.dequeue(); nextRune != '🤠' || nextRuneWidth != 4 || !ok {
		t.Errorf(
			"dequeue should not return (%c, %d, %v)",
			nextRune, nextRuneWidth, ok,
		)
	}
	if nextRune, nextRuneWidth, ok := peekCache.dequeue(); nextRune != '😍' || nextRuneWidth != 2 || !ok {
		t.Errorf(
			"dequeue should not return (%c, %d, %v)",
			nextRune, nextRuneWidth, ok,
		)
	}

	if runeOffset, byteOffset := peekCache.frontier(); runeOffset != 1 || byteOffset != 1 {
		t.Errorf(
			"frontier should not return (%d, %d)",
			runeOffset, byteOffset,
		)
	}

	if _, ok := peekCache.at(2); ok {
		t.Error("ok should be false - index is out of bounds")
	}
	if _, ok := peekCache.at(1); ok {
		t.Error("ok should be false - index is out of bounds")
	}
	if nextRune, ok := peekCache.at(0); nextRune != '😭' || !ok {
		t.Errorf(
			"at should not return (%c, %v)",
			nextRune, ok,
		)
	}

	if ok := peekCache.enqueue('🤯', 123); !ok {
		t.Error("ok should be true - there is enough space left")
	}

	if nextRune, ok := peekCache.at(1); nextRune != '🤯' || !ok {
		t.Errorf(
			"at should not return (%c, %v)",
			nextRune, ok,
		)
	}
	if nextRune, ok := peekCache.at(0); nextRune != '😭' || !ok {
		t.Errorf(
			"at should not return (%c, %v)",
			nextRune, ok,
		)
	}

	peekCache.dequeue()
	peekCache.dequeue()

	if _, ok := peekCache.at(0); ok {
		t.Error("ok should be false - index is out of bounds")
	}

	if runeOffset, byteOffset := peekCache.frontier(); runeOffset != 0 || byteOffset != 0 {
		t.Errorf(
			"frontier should not return (%d, %d)",
			runeOffset, byteOffset,
		)
	}
}
