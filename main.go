package main

import (
	"fmt"
	"time"
)

func fib32(n int32) int32 {
	if n == 0 || n == 1 {
		return n
	}
	return fib32(n-1) + fib32(n-2)
}

func fib64(n float64) float64 {
	if n == 0 || n == 1 {
		return n
	}
	return fib64(n-1) + fib64(n-2)
}

func main() {
	t1 := time.Now()
	fib32(42)
	fmt.Println(time.Now().Sub(t1))

	t2 := time.Now()
	fib64(42)
	fmt.Println(time.Now().Sub(t2))

	// t3 := time.Now()
	// fib64(42.1) // 42.1 会溢出
	// fmt.Println(time.Now().Sub(t3))
}
