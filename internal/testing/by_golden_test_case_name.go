package denimtesting

// Extension to `[]GoldenTestCase` that adds sorting by name.
type byGoldenTestCaseName []GoldenTestCase

func (b byGoldenTestCaseName) Len() int {
	return len(b)
}
func (b byGoldenTestCaseName) Swap(i, j int) {
	b[i], b[j] = b[j], b[i]
}
func (b byGoldenTestCaseName) Less(i, j int) bool {
	return len(b[i].name) < len(b[j].name)
}
