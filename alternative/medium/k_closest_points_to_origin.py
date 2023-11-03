from typing import List
import heapq

def kClosest(points: List[List[int]], k: int) -> List[List[int]]:
    res = []
    priority_queue = []

    for point in points:
        eucadient_distance = point[0] * point[0] + point[1] * point[1]
        heapq.heappush(priority_queue, (eucadient_distance, point))

    while k > 0:
        priority, data = heapq.heappop(priority_queue)
        k -=1
        res.append(data)
    
    return res

assert kClosest([[1,3],[-2,2]], 1) == [[-2,2]]