const MOD:u64 = 1_000_000_007;
impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let (zero, one, limit) = (zero as usize, one as usize, limit as usize);
        let mut dp = vec![vec![vec![vec![0;limit+1];2];one+1];zero+1];
        dp[0][0][0][0] = 1;dp[0][0][1][0] = 1;

        for z in 0..=zero {
            for o in 0..=one {
                let mut tmp = dp[z][o][1].iter().sum::<u64>() % MOD;                
                for i in 1..=limit.min(zero-z) {
                    dp[z+i][o][0][i] += tmp;
                }
                tmp = dp[z][o][0].iter().sum::<u64>() % MOD;
                for i in 1..=limit.min(one-o) {
                    dp[z][o+i][1][i] += tmp;
                }
            }
        }
        let res = dp[zero][one][1].iter().sum::<u64>()
            + dp[zero][one][0].iter().sum::<u64>();
        return (res % MOD) as i32;
    }
}