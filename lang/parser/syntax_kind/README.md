# `syntax_kind`

This package generates the `SyntaxKind` enum using
[`go-enum`](https://github.com/abice/go-enum). The codegen itself is enabled by
`go generate`; to regenerate generated source in this package:

```sh
$ cd lang/parser/syntax_kind
$ go generate .
```

## Pre-requisites

You may need to run `go mod tidy` first before `go generate` works correctly.
