package main

import (
	"fmt"
	"strings"
	"testing"
)

func TestMain(m *testing.M) {

	path := "65200d89859fdecaceb7ca8b5c9671f3/image2/0b/16/th_0b1647df161ebb60794327e6cb43b933hd"

	fmt.Println(strings.Join(strings.SplitAfter(path, "/")[1:], ""))
}
