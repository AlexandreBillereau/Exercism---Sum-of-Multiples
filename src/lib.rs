use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {

    let mut unique_multiple: HashSet<u32> = HashSet::new();
    unique_multiple.insert(0);

    factors.iter().for_each(|e| {
        let mut counter = 1;
        loop {
            if e * counter >= limit || *e == 0 { break; }
            unique_multiple.insert(e * counter);
            counter += 1;
        }
    });
    
    unique_multiple.iter().sum()
}
