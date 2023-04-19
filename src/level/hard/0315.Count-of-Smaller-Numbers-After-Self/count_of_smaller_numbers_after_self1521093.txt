// https://leetcode.com/problems/count-of-smaller-numbers-after-self/solutions/1521093/python-go-java-c-rust-c-c-dp-w-iterative-merge-sort-o-n-logn/
def countSmaller(self, nums):
    size = len(nums)
    dp, A = [0] * size, list(range(size))                              # A: indices of nums to be mergesorted
    for n in range(math.ceil(math.log(size, 2))):
        for i in range(0, size, 2 ** (n + 1)):
            L, R = A[i : i + 2 ** n], A[i + 2 ** n : i + 2 ** (n + 1)] # L / R: left / right half copied for mergesort
            for j in range(i, i + len(L) + len(R))[::-1]:              # iterate over A backwards to mergesort
                if not R or L and nums[L[-1]] > nums[R[-1]]:
                    dp[L[-1]] += len(R)                                # add up count of smaller right elements per legit left element
                    A[j] = L.pop()
                else:
                    A[j] = R.pop()
    return dp