fn main() {
    println!(
        "{}",
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
    );
    println!(
        "{}",
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3])
    );
    println!(
        "{}",
        Solution::can_complete_circuit(vec![3, 3, 4], vec![3, 4, 4])
    );
}

struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        /*
         * //shift cost right by one and
         * pair everything
         * we now have a list of (gas, how much needed to go to next), aka (n, m)
         * if every pair satisfies n >= m, return 0
         * establish excess = 0
         * for each pair
         * n += excess, excess = 0
         *  if n == m, ignore it
         *  if n > m, ignore it, excess = n-m
         *  if n < m, remember it
         * for the last pair,
         */
        let (original_indexes_of_nonzero_tank_deltas, nonzero_tank_deltas): (Vec<usize>, Vec<i32>) =
            gas.into_iter()
                .zip(cost)
                .map(|(g, c)| g - c)
                .enumerate()
                .filter(|(_, tank_delta)| *tank_delta != 0)
                .unzip();
        // println!("{:?}", nonzero_tank_deltas);

        if original_indexes_of_nonzero_tank_deltas.is_empty() {
            return 0;
        }

        let tank_delta_sum: i32 = nonzero_tank_deltas.iter().sum();
        // println!("tank_delta_sum: {}", tank_delta_sum);
        if tank_delta_sum < 0 {
            return -1;
        }

        let indexes_of_positive_tank_deltas: Vec<usize> = nonzero_tank_deltas
            .iter()
            .enumerate()
            .filter(|(_, tank_delta)| **tank_delta > 0)
            .map(|(i, _)| i)
            .collect();

        // let (indexes_of_positive_tank_deltas, indexes_of_negative_tank_deltas): (
        //     Vec<usize>,
        //     Vec<usize>,
        // ) = (0..nonzero_tank_deltas.len()).partition(|i| nonzero_tank_deltas[*i] > 0);
        // if indexes_of_negative_tank_deltas.len() < indexes_of_positive_tank_deltas.len() {
        //     // um
        // } else {
        // }

        let mut previous_index: Option<usize> = None;
        for i in indexes_of_positive_tank_deltas {
            // println!("starting at {}", i);
            // I don't actually need to test all of these indexes. I only need to test the first index of every contiguous stretch of positive tank_deltas
            if let Some(true) = previous_index.and_then(|pi| Some(pi == i || pi + 1 == i)) {
                previous_index = Some(i);
                continue;
            }
            previous_index = Some(i);
            if can_get_from_x_to_y(i, i, 0, &nonzero_tank_deltas) {
                return original_indexes_of_nonzero_tank_deltas[i] as i32;
            }
        }

        -1
    }
}

fn can_get_from_x_to_y(x: usize, y: usize, mut tank: i32, tank_deltas: &[i32]) -> bool {
    let next_index = (x + 1) % tank_deltas.len();
    tank += tank_deltas[x];
    if tank >= 0 {
        if next_index == y {
            return true;
        }
        return can_get_from_x_to_y(next_index, y, tank, tank_deltas);
    }
    // println!(
    //     "can't get from {} to {} with a tank of {}",
    //     x, next_index, tank
    // );
    false
}

//// the following is a naive solution that times out given nasty input
// impl Solution {
//     pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//         gas.iter().zip().filter(|pair| pair !=).unzip
//         (0..gas.len() as i32)
//             .find(|i| can_get_from_x_to_y(*i as usize, *i as usize, 0, &gas, &cost))
//             .or(Some(-1))
//             .unwrap()
//     }
// }

// fn can_get_from_x_to_y(x: usize, y: usize, mut tank: i32, gas: &[i32], cost: &[i32]) -> bool {
//     let next_index = (x + 1) % gas.len();
//     tank += gas[x];
//     if cost[x] <= tank {
//         if next_index == y {
//             return true;
//         }
//         tank -= cost[x];
//         return can_get_from_x_to_y(next_index, y, tank, gas, cost);
//     }
//     false
// }
