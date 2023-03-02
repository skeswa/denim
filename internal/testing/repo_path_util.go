package denimtesting

import (
	"errors"
	"path/filepath"
	"runtime"
)

// Returns the absolute path to the root of the Denim repository, or an [error]
// if something goes wrong.
//
// This helper function is intended to be transitively called from within a Go
// test.
func PathToRepoRoot() (string, error) {
	_, pathToThisFile, _, _ := runtime.Caller(0)

	return filepath.Join(pathToThisFile, "../../.."), nil
}

// Turns the given repo-relative path into an absulte path and returns it, or an
// [error] if something goes wrong.
//
// This helper function is intended to be transitively called from within a Go
// test.
func RepoRelativePathToAbsolute(repoRelativePath string) (string, error) {
	repoRootPath, err := PathToRepoRoot()
	if err != nil {
		return "", errors.Join(errors.New("failed to find repo root"), err)
	}

	return filepath.Join(repoRootPath, repoRelativePath), nil
}
