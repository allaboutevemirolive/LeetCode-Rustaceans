// https://leetcode.com/problems/erect-the-fence/solutions/2828910/python-c-rust-upper-lower-convex-hulls-bonus-scipy-numpy-explained/
from scipy.spatial import Delaunay
from numpy import array, average, polyfit

class Solution:
    def outerTrees(self, trees, eps=10**-10):
        
        T = array(trees)                                    # [1] convert to numpy array for convenience
        
        if len(T) < 4: return trees                         # [2] consider trivial cases when either there
        if polyfit(T[:,0], T[:,1], 1, full=True)[1] < eps:  #     are less than 4 trees or all of them lie
            return trees                                    #     on the same line (sum of residuals < eps)
        
        cm = average(T, axis=0)                             # [3] compute center of mass of all trees
        F = Delaunay(T)                                     # [4] triangulate (it's a way to make a convex hull)
        
        def on_fence(t):                                    # [5] if a tree is ON fence, then by moving it away from
            return F.find_simplex(t+(t-cm)*eps) < 0         #     the center of mass, we'll expell it from all simplices

        return [t for t in T if on_fence(t)]                # [6] return trees that satisfy the 'on_fence' criterion

        