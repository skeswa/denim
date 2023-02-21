//go:build tools
// +build tools

package tools

// For more on what the heck is going on here, please consult:
// https://stackoverflow.com/questions/52428230/how-do-go-modules-work-with-installable-commands/54028731#54028731

import (
	_ "github.com/abice/go-enum"
)
