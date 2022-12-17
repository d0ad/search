#[derive(Debug)]
pub struct SearchResult {
    pub position: Option<usize>,
    pub iterations: u32,
    pub comparisons: u32,
}

pub fn binary_search(arr: &[u16], target_value: &u16) -> SearchResult {
    let mut iterations = 0;
    let mut comparisons = 0;

    let mut low: i32 = 0;
    let mut high: i32 = (arr.len() - 1) as i32;

    while low <= high {
        iterations += 1;
        let mid = (high - low) / 2 + low;
        let val = &arr[mid as usize];

        comparisons += 1;
        if val == target_value {
            return SearchResult {
                position: Some(mid as usize),
                iterations,
                comparisons,
            };
        }

        comparisons += 1;
        if val < target_value {
            low = mid + 1;
        }

        comparisons += 1;
        if val > target_value {
            high = mid - 1;
        }
    }

    SearchResult {
        position: None,
        iterations,
        comparisons,
    }
}

pub fn interpolation_search(arr: &[u16], target: &u16) -> SearchResult {
    let mut iterations = 0;
    let mut comparisons = 0;

    if arr.is_empty() {
        return SearchResult {
            position: None,
            iterations,
            comparisons,
        };
    }

    let mut hi = arr.len() - 1;
    let mut lo = 0_usize;

    comparisons += 2;
    if *target < arr[lo] && *target > arr[hi] {
        return SearchResult {
            position: None,
            iterations,
            comparisons,
        };
    }

    let mut interpolant: usize;

    loop {
        iterations += 1;
        let lo_val = arr[lo];
        let hi_val = arr[hi];

        if hi <= lo {
            break;
        }

        comparisons += 1;
        if *target < lo_val {
            break;
        }

        comparisons += 1;
        if *target > hi_val {
            break;
        }

        let offset: u64 = (*target - lo_val) as u64 * (hi - lo) as u64 / (hi_val - lo_val) as u64;
        interpolant = lo + offset as usize;

        let mid_val = arr[interpolant];
        if mid_val == *target {
            return SearchResult {
                position: Some(interpolant),
                iterations,
                comparisons,
            };
        }

        comparisons += 1;
        if mid_val > *target {
            hi = interpolant - 1;
            continue;
        }

        comparisons += 1;
        if mid_val < *target {
            lo = interpolant + 1;
            continue;
        }

        break;
    }

    SearchResult {
        position: None,
        iterations,
        comparisons,
    }
}
