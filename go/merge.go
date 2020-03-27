package main
import (
    "fmt"
    "time"
    "math/rand"
    "os"
    "strconv"
    "sort"
)

func MergeSort(slice []int) []int {
	if len(slice) < 2 {
		return slice
	}
	mid := len(slice) / 2
	return Merge(
        MergeSort(slice[:mid]),
        MergeSort(slice[mid:]),
    )
}

func Merge(left, right []int) []int {
	size, i, j := len(left) + len(right), 0, 0
	slice := make([]int, size, size)
    for i < len(left) && j < len(right) {
        if left[i] < right[j] {
            slice[i+j] = left[i]
            i++
        } else {
            slice[i+j] = right[j]
            j++
        }
    }
    if i < len(left) {
        copy(slice[i+j:], left[i:])
    } else if j < len(right) {
        copy(slice[i+j:], right[j:])
    }
	return slice
}

func ParallelMergeSort(data []int, out chan []int) {
    if len(data) <= (1 << 13) {
        out <- MergeSort(data)
    } else {
        leftChan := make(chan []int)
        rightChan := make(chan []int)
        middle := len(data)/2
        go ParallelMergeSort(data[:middle], leftChan)
        go ParallelMergeSort(data[middle:], rightChan)
        ldata := <-leftChan
        rdata := <-rightChan
        close(leftChan)
        close(rightChan)
        out <- Merge(ldata, rdata)
    }
}

func Bench(N int, sorter func([]int)) time.Duration {
    r := rand.New(rand.NewSource(18))
    unsorted := make([]int, N)
    for i := 0; i < N; i++ {
        unsorted[i] = r.Intn(1 << 30)
    }
    now := time.Now()
    sorter(unsorted)
    elapsed := time.Now().Sub(now)
    return elapsed
}

func main() {
    N := 100000000
    if len(os.Args) > 1 {
        N, _ = strconv.Atoi(os.Args[1])
    }
    single := Bench(N, func (s []int) {
        MergeSort(s)
    })
    multi := Bench(N, func (s []int) {
        out := make(chan []int)
        go ParallelMergeSort(s, out)
        <-out
        close(out)
    })
    builtin := Bench(N, func (s []int) {
        sort.Ints(s)
    })
    fmt.Printf("Single thread: %v\n", single)
    fmt.Printf("Multi thread:  %v\n", multi)
    fmt.Printf("Builtin sort:  %v\n", builtin)
}
