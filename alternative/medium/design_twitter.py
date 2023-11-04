from typing import List
import heapq

class Twitter:

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.tweets = {}
        self.follows = {}
        self.time = 0
        
    def postTweet(self, userId: int, tweetId: int) -> None:
        if self.tweets.get(userId) is None:
            self.tweets[userId] = []

        self.tweets[userId].append([self.time, tweetId])
        self.time -= 1
        

    def getNewsFeed(self, userId: int) -> List[int]:
        minHeap = []
        res = []

        if self.follows.get(userId) is None:
            self.follows[userId] = set()

        self.follows[userId].add(userId)

        for followeeId in self.follows[userId]:
            if followeeId in self.tweets:
                index = len(self.tweets[followeeId]) - 1
                count, tweetId = self.tweets[followeeId][index]
                minHeap.append([count, tweetId, followeeId, index - 1])
        
        heapq.heapify(minHeap)

        while len(res) < 10 and minHeap:
            count, tweetId, followeeId, index = heapq.heappop(minHeap)
            res.append(tweetId)
            if index >= 0:
                count, tweetId, self.tweets[followeeId][index]
                heapq.heappush(minHeap, [count, tweetId, followeeId, index - 1])

        return res
        

    def follow(self, followerId: int, followeeId: int) -> None:
        if self.follows.get(followerId) is None:
            self.follows[followerId] = set()

        self.follows[followerId].add(followeeId)
        

    def unfollow(self, followerId: int, followeeId: int) -> None:
        if followeeId in self.follows[followerId]:
            self.follows[followerId].remove(followeeId)
        


# Your Twitter object will be instantiated and called as such:
# obj = Twitter()
# obj.postTweet(userId,tweetId)
# param_2 = obj.getNewsFeed(userId)
# obj.follow(followerId,followeeId)
# obj.unfollow(followerId,followeeId)

twitter = Twitter()
twitter.postTweet(1, 5)
assert twitter.getNewsFeed(1) == [5]
