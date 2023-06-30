//!
//!
//!
//!
//!
//!
//!
fn _00_basic_concept() {}

fn sum_of_val(nums: &[i32], num: i32) -> i32 {
    let mut sum: i32 = 0;
    for n in nums {
        sum += n;
    }
    sum + num // 无分号表示返回(return)
}

#[test]
fn _01_basic_retro() {
    let num = 10;
    let nums = [1, 2, 3, 4, 5, 6, 7, 8];
    let sum = sum_of_val(&nums, num);
    assert_eq!(46, sum)
}
