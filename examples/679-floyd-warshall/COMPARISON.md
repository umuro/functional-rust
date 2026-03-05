# Floyd-Warshall Comparison

Simple triple-nested loop. Both implementations are nearly identical.

Key recurrence: `dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])`
