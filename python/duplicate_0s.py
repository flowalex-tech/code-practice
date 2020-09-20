#!/usr/bin/env python3

"""
Given a fixed length array arr of integers, duplicate each occurrence of zero, shifting the remaining elements to the right.
Note that elements beyond the length of the original array are not written.
Do the above modifications to the input array in place, do not return anything from your function.

Example 1:
Input: [1,0,2,3,0,4,5,0]
Output: null
Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]

Example 2:
Input: [1,2,3]
Output: null
Explanation: After calling your function, the input array is modified to: [1,2,3]

Note:
    1 <= arr.length <= 10000
    0 <= arr[i] <= 9
    https://leetcode.com/problems/duplicate-zeros/
"""

import array as arr

arr = [1,0,2,3,0,4,5,0]
#arr = [1,2,3]
#arr = [0,8,2]
i = 0
while i < len(arr):
    if arr[i] == 0
        arr.insert(i,0)
        arr.pop()
        i += 1
        print(arr)
