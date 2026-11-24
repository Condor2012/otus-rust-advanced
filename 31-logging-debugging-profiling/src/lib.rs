pub fn sum_to(limit: u64) -> u64 {
    (1..=limit).sum()
}

pub fn checked_average(values: &[i32]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let sum: i64 = values.iter().map(|value| i64::from(*value)).sum();
    Some(sum as f64 / values.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workload_returns_known_result() {
        assert_eq!(sum_to(1_000), 500_500);
    }

    #[test]
    fn average_handles_normal_and_empty_inputs() {
        assert_eq!(checked_average(&[1, 2, 3]), Some(2.0));
        assert_eq!(checked_average(&[]), None);
    }
}
