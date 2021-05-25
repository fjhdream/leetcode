use std::collections::HashMap;

pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
    // x 的范围为 [0, 2^10)
    let _MAX_X:usize = 1 << 10;
    // 极大值，为了防止整数溢出选择 INT_MAX / 2
    let _MAX: i32 = i32::MAX / 2;
    let n = nums.len();
    let k = k as usize;

    // 初始化第一次f(i,mask)的所有值
    let mut f = vec![_MAX; _MAX_X];

    // 边界条件 f(-1,0)=0, 当 i=0 时，f(−1,..) 都是需要特殊考虑的边界条件。
    // 由于 f(-1, ..)f(−1,..) 表示没有考虑任何组时的异或和，因此该异或和一定为 00，即 f(-1, 0) = 0f(−1,0)=0。其它的状态都是不合法的状态，我们可以将它们赋予一个极大值 \infty∞。
    f[0] = 0;

    /*
      f(i,mask)=  min {f(i−1,mask⊕x)+size(i)−count(i,x)}
      f(i,mask)=size(i) + min {f(i−1,mask⊕x)−count(i,x)}
      t2 : 对应 x 不在哈希表中出现的状态，以及 x 在哈希表中出现并且我们额外添加的状态 min {f(i−1,mask⊕x)}
      t1 : t1 x∈HashTable(i) min{f(i−1,mask⊕x)−count(i,x)}
    */
    for i in 0..k {
        let mut size = 0;
        // 第 i 个组的哈希映射
        let mut cnt: HashMap<i32, usize> = HashMap::new();
        for j in (i..n).step_by(k) {
            cnt.insert(nums[j],
                       if cnt.contains_key(&nums[j]) {
                           *cnt.get(&nums[j]).unwrap() + 1
                       } else {
                           1
                       });
            size += 1;
        }

        // 求出 t2
        let t2_min = *f.iter().min().unwrap();
        let mut g = vec![t2_min; _MAX_X];

        for mask in 0.._MAX_X {
            // t1 则需要枚举 x 才能求出
            for (key, value) in cnt.iter() {
                let x = *key as usize;
                let cnt_x = *value as i32;
                g[mask] = g[mask].min(f[mask^x] - cnt_x);
            }
        }

        // 别忘了加上 size
        for j in 0.._MAX_X {
            g[j] += size;
        }

        f = g;
    }
    f[0]
}

#[test]
pub fn test_example() {
    let numbs = vec![1, 2, 0, 3, 0];
    let k = 1;
    let ans = min_changes(numbs, k);
    println!("{:?}", ans)
}