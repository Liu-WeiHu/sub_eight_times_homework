//第五题
fn main() {
    //创建一个slice
    let nums = &[1, 1 << 31, 1 << 31, 2][..];
    //如果有值
    if let Some(sum) = sum(nums) {
        println!(" slice sum = {}", sum);
        //如果没有值
    } else {
        println!("slice sum overflow");
    }
}

// slice 求和方法
fn sum(nums: &[u32]) -> Option<u32> {
    // 利用 iter trait 的fold 方法，将所有的元素求和
    nums.into_iter().fold(
        // 初始值
        Some(0),
        // 将前一个值和当前值求和
        |acc, x| acc.
            // 如果 acc 为 None，则直接返回 None
            and_then(|acc| x.
                // 检查有没有溢出，如果溢出，则返回 None
                checked_add(acc)))
}