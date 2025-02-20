pub fn dp_rec_mc(amount: u32) -> u32 {
    // Define the coin denominations
    let coins = [1, 2, 5, 10, 20, 30, 50, 100];

    // Create a dynamic programming array to store the minimum number of coins for each amount
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0; // Base case: 0 coins are needed to make amount 0

    // Iterate through each amount from 1 to the target amount
    for i in 1..=amount {
        for &coin in coins.iter() {
            if coin <= i {
                // Update the dp array if using this coin results in a better solution
                dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
            }
        }
    }

    // Return the result for the target amount
    dp[amount as usize]
}
