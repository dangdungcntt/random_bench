package main

import (
	"fmt"
	"math/rand"
	"time"

	"golang.org/x/text/language"
	"golang.org/x/text/message"
)

func main() {
	rewards := []int{1, 2, 3, 4}

	length := len(rewards)

	totalPercent := float64(sum(rewards))

	countById := make([]int64, len(rewards))

	runCount := 100000000

	start := time.Now()

	for j := 0; j < runCount; j++ {
		random := rand.Float64() * totalPercent
		for i := 0; i < length; i++ {
			random -= float64(rewards[i])
			if random <= 0 {
				countById[i]++
				break
			}
		}
	}

	elapsed := time.Since(start)

	printer := message.NewPrinter(language.English)

	fmt.Printf("Total run: %s, elapsed: %.2fs. %d.00 ns/run, %s runs/s\n", printer.Sprintf("%d", runCount), elapsed.Seconds(), (elapsed / time.Duration(runCount)).Nanoseconds(), printer.Sprintf("%d", int64(runCount)/elapsed.Milliseconds()*1000))

	for i, count := range countById {
		fmt.Printf("id: %d count: %d (%.6f%%)\n", i, count, float64(count)/float64(runCount)*100)
	}
}

func sum(slice []int) int {
	s := 0
	for _, v := range slice {
		s += v
	}
	return s
}
