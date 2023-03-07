package source_test

import (
	"fmt"
	"testing"

	"github.com/skeswa/denim/lang/lexer/source"
)

func TestLeBronTweet(t *testing.T) {
	c := source.NewCursor(3, "🤔Something is REAL 🐠 🐟 🎣 🐟🐠 going on")

	expectPeekRune(
		c, t, 0,
		false, // Current rune is not yet a thing
	)
	expectPeekRune(
		c, t, 2,
		true, 'S',
	)
	expectPeekRune(
		c, t, 3,
		true, 'o',
	)
	expectPeekRune(
		c, t, 4,
		true, 'm',
	)

	expectNextRune(
		c, t,
		true, '🤔',
	)
	expectNextRune(
		c, t,
		true, 'S',
	)
	expectNextRune(
		c, t,
		true, 'o',
	)
	expectNextRune(
		c, t,
		true, 'm',
	)
	expectPeekRune(
		c, t /* runeOffset= */, 0,
		true, 'm',
	)
	expectPeekRune(
		c, t /* runeOffset= */, 3,
		true, 'h',
	)

	expectNextRune(
		c, t,
		true, 'e',
	)
	expectNextRune(
		c, t,
		true, 't',
	)
	expectNextRune(
		c, t,
		true, 'h',
	)
	expectNextRune(
		c, t,
		true, 'i',
	)
	expectPeekRune(
		c, t /* runeOffset= */, 28,
		true, 'n',
	)

	// The 29th rune ahead of the current one is the last rune in the string.
	expectPeekRune(
		c, t /* runeOffset= */, 29,
		false,
	)

	for i := 0; i < 27; i++ {
		expectNextRune(
			c, t,
			true,
		)
	}

	expectNextRune(
		c, t,
		true, 'n',
	)

	expectNextRune(
		c, t,
		false,
	)
}

func expectNextRune(
	c *source.Cursor,
	t *testing.T,
	expectedOk bool,
	optionalExpectedRune ...rune,
) {
	var expectedRune *rune
	if len(optionalExpectedRune) > 0 {
		expectedRune = &optionalExpectedRune[0]
	}

	actualRune, actualOk := c.NextRune()

	if (expectedRune == nil || (actualRune == *expectedRune)) &&
		actualOk == expectedOk {
		return
	}

	expectedRuneString := "?"
	if expectedRune != nil {
		expectedRuneString = fmt.Sprintf("'%c'", *expectedRune)
	}

	t.Errorf(
		"Expected NextRune() to return (%s, %v), but instead got (%c, %v)",
		expectedRuneString, expectedOk, actualRune, actualOk,
	)
}

func expectPeekRune(
	c *source.Cursor,
	t *testing.T,
	runeOffset int,
	expectedOk bool,
	optionalExpectedRune ...rune,
) {
	var expectedRune *rune = nil
	if len(optionalExpectedRune) > 0 {
		expectedRune = &optionalExpectedRune[0]
	}

	actualRune, actualOk := c.PeekRune(runeOffset)

	if (expectedRune == nil || (actualRune == *expectedRune)) &&
		actualOk == expectedOk {
		return
	}

	expectedRuneString := "?"
	if expectedRune != nil {
		expectedRuneString = fmt.Sprintf("'%c'", *expectedRune)
	}

	t.Errorf(
		"Expected PeekRune(%d) to return (%s, %v), but instead got (%c, %v)",
		runeOffset, expectedRuneString, expectedOk, actualRune, actualOk,
	)
}
