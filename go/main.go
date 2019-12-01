package main

import (
	"fmt"
	"time"
)

func main() {
	start := time.Now()
	fibonacci(50)
	end := fmt.Sprintf("%.2f", time.Since(start).Seconds())
	fmt.Println(end, "s")
}

func fibonacci(n int) int {
	if n <= 1 {
		return 1
	}
	return fibonacci(n-1) + fibonacci(n-2)
}
