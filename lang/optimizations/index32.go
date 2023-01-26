package optimizations

// Stores a 32-bit index where the zero value is an invalid index.
//
// This is a better alternative to storing the index as a pointer since that has
// the same properties but takes up more space and costs an extra pointer
// traversal.
type Index32 struct {
	flippedBits uint32
}

// Returns a new `Index32` wrapping the given `index`.
func MakeIndex32(index uint32) Index32 {
	return Index32{flippedBits: ^index}
}

// Returns the `uint32` wrapped by this `Index32`.
func (i Index32) GetIndex() uint32 {
	return ^i.flippedBits
}

// Returns `true` if this is a correctly constructed `Index32`.
func (i Index32) IsValid() bool {
	return i.flippedBits != 0
}
