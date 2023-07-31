struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut l: Vec<i32> = Vec::new();
        let mut r: Vec<i32> = Vec::new();
        l.push(nums[0]);
        for i in 1..nums.len() {
            l.push(nums[i] * l[l.len() - 1])
        }
        r.push(nums[nums.len() - 1]);
        for i in 1..nums.len() {
            r.push(nums[nums.len() - (i + 1)] * r[r.len() - 1])
        }
        let mut res: Vec<i32> = Vec::new();
        res.push(r[r.len() - 2]);
        for i in 1..nums.len()-1 {
            res.push(l[i-1] * r[r.len() - 2 - i])
        }
        res.push(l[l.len() - 2]);
        return res;
    }

}
fn main() {
    let nums = vec![-1,1,0,-3,3];
    let result = Solution::product_except_self(nums);
    println!("{:?}", result);
}

