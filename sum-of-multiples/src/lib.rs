use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();

    for item_value in factors.iter() {
        for index in 1..=limit {
            let result = index * item_value;

            if result < limit {
                set.insert(result);
            } else {
                break;
            }
        }
    }

    set.iter().sum()
}
