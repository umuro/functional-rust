📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1073-burst-balloons)**

---

# 1073: Burst Balloons

**Difficulty:** Advanced
**Category:** Dynamic Programming (Interval DP)
**Concept:** Maximize coins collected by bursting balloons, where bursting balloon i earns `nums[left] * nums[i] * nums[right]`
**Key Insight:** Think in reverse — instead of which balloon to burst *first*, think which to burst *last* in each interval. This turns the problem into interval DP where `dp[i][j]` = max coins from balloons between boundaries i and j.
