pub fn new_birthday_probability(n: u32) -> f64 {
    if n == 0 {
        return 0.0;
    }
    if n > 365 {
        return 1.0;
    }

    let mut probability = 1.0;
    for k in 1..n {
        probability *= 1.0 - (k as f64) / 365.0;
    }

    1.0 - probability
}
