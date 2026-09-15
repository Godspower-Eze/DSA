func productExceptSelf(nums []int) []int {
	n := len(nums)
	output := make([]int, n)

	// prefix pass: output[i] = product of everything before i
	output[0] = 1
	for i := 1; i < n; i++ {
		output[i] = output[i-1] * nums[i-1]
	}

	// suffix pass: multiply in product of everything after i
	suffix := 1
	for i := n - 1; i >= 0; i-- {
		output[i] *= suffix
		suffix *= nums[i]
	}

	return output
}