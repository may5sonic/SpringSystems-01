fn main() {


    
    /*
    pub fn two_sum(nums: Vec<i32>, target: i32) ->
    Vec<i32> {
        for (i, &a) in nums.iter().enumerate() {
            for (j, &b) in nums.iter().enumerate().skip(i+1) { // ~N linear in essense
                if a + b == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
    */
}

// O(N^2)
// O(N * log(N)) sorting! + O(N) two pointers
// O(N) // mapping hashmap dictionary!
// O(log(N)) binary search ! bisect (python) lower_bound(c++)
// O(1)