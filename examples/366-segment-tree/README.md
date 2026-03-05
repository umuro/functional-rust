# 366: Segment Tree for Range Queries

**Difficulty:** 4  **Level:** Expert

O(log n) range queries and point updates on an array.

## The Problem This Solves

You have an array and need to answer many queries of the form "what's the sum of elements from index L to R?" while also supporting updates. A naive approach takes O(n) per query. A prefix sum array gives O(1) queries but O(n) updates.

A segment tree achieves O(log n) for both operations by storing precomputed sums of ranges in a tree structure.

## How It Works

The segment tree is a binary tree where:
- Each leaf represents a single array element
- Each internal node stores the sum of its children's ranges
- The root stores the total sum

To query a range, we recursively visit only nodes whose ranges overlap with our query - at most O(log n) nodes.

## Key Operations

| Operation | Complexity |
|-----------|------------|
| Build | O(n) |
| Range query | O(log n) |
| Point update | O(log n) |
| Range update (lazy) | O(log n) |
