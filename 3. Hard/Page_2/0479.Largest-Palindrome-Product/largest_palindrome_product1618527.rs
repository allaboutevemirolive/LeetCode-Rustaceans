// https://leetcode.com/problems/largest-palindrome-product/solutions/1618527/python-fast-swift-100-faster-100-less-memory-no-cheating/
product = multiplicand * multiplier
Now for an n-digit number, the maximum a multiplicand or a multiplier can be is (10ⁿ - 1). So,
product = 
           = (10ⁿ - x) * (10ⁿ - y)           (where x, y > 0); n = no. of digits
           = 10²ⁿ - 10ⁿx - 10ⁿy + xy
           = 10²ⁿ - 10ⁿ(x + y) + xy
           = 10ⁿ * (10ⁿ - (x + y)) + xy
           = 10ⁿ * left + right               (let left = 10ⁿ - (x + y) and right = xy)