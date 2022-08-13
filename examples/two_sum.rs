struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mem = std::collections::HashMap::<i32, usize>::with_capacity(nums.len());
        let mut ret: Vec<i32> = vec![-1, -1];

        for (i, &v) in nums.iter().enumerate() {
            let diff = target - v;
            
            if mem.contains_key(&diff) {
                let v0 = i as i32;
                let v1 = *(mem.get(&diff).unwrap()) as i32;

                if v0 < v1 {
                    ret = [v0, v1].to_vec();
                    break;
                }

                ret = [v1, v0].to_vec();
                break;
            }
            
            if !mem.contains_key(&v) {
                mem.insert(v, i);
            }
        }
        
        return ret
    }
}

fn main() {
    let ret = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(ret, vec![0, 1]);
}