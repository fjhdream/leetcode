pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let mut uf = UnionFind::new(n);

    for i in 0..n {
        for j in i + 1..n {
            let mut diff = 0;
            for (a, b) in strs[i].chars().zip(strs[j].chars()) {
                if a != b { diff += 1; }
                if diff > 2 { break; }
            }
            if diff <= 2 { uf.union(i, j); }
        }
    }
    
    uf.set_count as i32
}


#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    pub set_count: usize,
}

#[allow(unused)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind { parent: (0..n).collect(), set_count: n }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y { return false; }
        self.parent[root_y] = root_x;
        self.set_count -= 1;
        true
    }
}

#[test]
fn test_example() {
    let strs = vec![String::from("tars"),String::from("rats"),String::from("arts"),String::from("star")];
    let ans = num_similar_groups(strs);
    println!("{}", ans);
}