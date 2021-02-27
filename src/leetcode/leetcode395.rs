pub fn longest_substring(s: String, k: i32) -> i32 {
   fn part(s: &Vec<u8>, lo: usize, hi: usize, k: usize) -> usize {
       if hi - lo + 1 < k {
           return 0;
       }
       let mut freq = vec![0; 26];
       for i in lo..=hi {
           freq[s[i] as usize - 97] += 1;
       }
       let mut i = lo;
       let mut max = 0;
       for j in lo..=hi {
           if freq[s[j] as usize - 97] < k {
               max = max.max(part(s, i, j-1, k));
               i = j + 1;
           }
       }
       if i == lo {
           hi + 1 - lo
       } else {
           max.max(part(s, i, hi, k))
       }
   }
   let n = s.len();
   part(&s.into_bytes(), 0, n-1, k as usize) as i32
}