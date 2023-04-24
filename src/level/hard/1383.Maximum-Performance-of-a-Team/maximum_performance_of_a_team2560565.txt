// https://leetcode.com/problems/maximum-performance-of-a-team/solutions/2560565/python-rust-priority-queue/
class Solution:
    def maxPerformance(self, n: int, speed: List[int], efficiency: List[int], k: int) -> int:
        pairs = sorted([(e,s) for e,s in zip(efficiency,speed)], reverse = True)
        best = 0
        heap = []
        cur = 0
        for e,s in pairs:
            heappush(heap, s)
            cur += s
            if len(heap) > k:
                cur -= heappop(heap)
            if e * cur > best:
                best = e * cur
        return best % (10**9 + 7)