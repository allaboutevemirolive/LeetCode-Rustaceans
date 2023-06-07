// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/solutions/1823343/more-intuitive-permutation-derivation/
class Solution:
    def countOrders(self, n: int) -> int:
		MOD = 1_000_000_007
		answer = 1
		for i in range(2, n+1):
			answer = i*(2*i - 1)*answer
			answer %= MOD
		return answer