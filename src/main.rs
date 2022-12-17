use std::time::Instant;
mod search;

const VARIANT: u16 = 11;
const SEARCH_KEY: u16 = 10 * VARIANT;

fn main() {
    let sizes = vec![100, 1_000, 10_000, 100_000];

    for size in sizes.iter() {
        let mut vec: Vec<u16> = Vec::with_capacity(*size);
        for _ in 0..vec.capacity() {
            let num: u16 = rand::random();
            vec.push(num);
        }
        vec.push(SEARCH_KEY);
        vec.sort();

        println!("====================");
        println!("array size: {}", size);

        // Binary search
        let mut total_elapsed: std::time::Duration = std::time::Duration::ZERO;
        let mut total_iterations = 0;
        let mut total_comparisons = 0;

        for el in &vec {
            let now = Instant::now();
            let result = search::binary_search(&vec[..], el);
            let elapsed = now.elapsed();

            total_elapsed += elapsed;
            total_iterations += result.iterations;
            total_comparisons += result.comparisons;
        }

        println!(
            "binary_search average time elapsed: {:?}",
            total_elapsed / *size as u32
        );
        println!("iterations: {}", total_iterations / *size as u32);
        println!("comparisons: {}", total_comparisons / *size as u32);
        println!("\n");
        // Variant
        let now = Instant::now();
        let result = search::binary_search(&vec[..], &SEARCH_KEY);
        let elapsed = now.elapsed();
        println!(
            "binary_search time elapsed, variant {}: {:?}",
            VARIANT, elapsed
        );
        println!("search key: {}, result:  {:?}", SEARCH_KEY, result.position);
        println!("iterations: {}", result.iterations);
        println!("comparisons: {}", result.comparisons);
        println!("\n");

        // Interpolation search
        let mut total_elapsed: std::time::Duration = std::time::Duration::ZERO;
        let mut total_iterations = 0;
        let mut total_comparisons = 0;

        for el in &vec {
            let now = Instant::now();
            let result = search::interpolation_search(&vec[..], el);
            let elapsed = now.elapsed();

            total_elapsed += elapsed;
            total_iterations += result.iterations;
            total_comparisons += result.comparisons;
        }

        println!(
            "interpolation_search average time elapsed: {:.2?}",
            total_elapsed / *size as u32
        );
        println!("iterations: {}", total_iterations / *size as u32);
        println!("comparisons: {}", total_comparisons / *size as u32);
        println!("\n");
        // Variant
        let now = Instant::now();
        let result = search::interpolation_search(&vec[..], &SEARCH_KEY);
        let elapsed = now.elapsed();
        println!(
            "interpolation_search time elapsed, variant {}: {:?}",
            VARIANT, elapsed
        );
        println!("search key: {}, result:  {:?}", SEARCH_KEY, result.position);
        println!("iterations: {}", result.iterations);
        println!("comparisons: {}", result.comparisons);
        println!("\n");
    }
}

/*
 ====================
array size: 100
binary_search average time elapsed: 143ns
iterations: 5
comparisons: 15


binary_search time elapsed, variant 11: 281ns
search key: 110, result:  Some(0)
iterations: 6
comparisons: 16


interpolation_search average time elapsed: 128.00ns
iterations: 2
comparisons: 9


interpolation_search time elapsed, variant 11: 220ns
search key: 110, result:  Some(0)
iterations: 1
comparisons: 4


====================
array size: 1000
binary_search average time elapsed: 185ns
iterations: 8
comparisons: 24


binary_search time elapsed, variant 11: 226ns
search key: 110, result:  Some(0)
iterations: 9
comparisons: 25


interpolation_search average time elapsed: 139.00ns
iterations: 3
comparisons: 11


interpolation_search time elapsed, variant 11: 74ns
search key: 110, result:  Some(0)
iterations: 1
comparisons: 4


====================
array size: 10000
binary_search average time elapsed: 140ns
iterations: 12
comparisons: 34


binary_search time elapsed, variant 11: 70ns
search key: 110, result:  Some(18)
iterations: 9
comparisons: 25


interpolation_search average time elapsed: 41.00ns
iterations: 3
comparisons: 12


interpolation_search time elapsed, variant 11: 43ns
search key: 110, result:  Some(18)
iterations: 2
comparisons: 8


====================
array size: 100000
binary_search average time elapsed: 89ns
iterations: 14
comparisons: 41


binary_search time elapsed, variant 11: 126ns
search key: 110, result:  Some(169)
iterations: 12
comparisons: 34


interpolation_search average time elapsed: 36.00ns
iterations: 3
comparisons: 12


interpolation_search time elapsed, variant 11: 25ns
search key: 110, result:  Some(167)
iterations: 1
comparisons: 4
 */
