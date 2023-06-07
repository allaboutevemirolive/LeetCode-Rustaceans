// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/2757694/daily-leetcoding-challenge-october-day-30/
        // Opt 3
        if (m <= 2 && n == 1) {
            return 0;
        }

        int cnt = 0;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                cnt += grid[i][j];
            }
        }

        // Opt 2
        if (k >= m + n - 3 || k >= cnt) {
            return m + n - 2;
        }