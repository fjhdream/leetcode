pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let n = n as usize;
    let mut alice = UnionFind::new(n);
    let mut bob = UnionFind::new(n);
    //先遍历类型3的边
    for edge in edges.iter() {
        if edge[0] == 3 {
            let p1 = (edge[1] - 1) as usize;
            let p2 = (edge[2] - 1) as usize;
            if !alice.union(p1, p2) {
                res += 1;
            }
            bob.union(p1, p2);
        }
    }
    //分别遍历类型1,2的边
    for edge in edges.iter() {
        let p1 = (edge[1] - 1) as usize;
        let p2 = (edge[2] - 1) as usize;
        if edge[0] == 1 {
            if !alice.union(p1, p2) {
                res += 1;
            }
        } else if edge[0] == 2 {
            if !bob.union(p1, p2) {
                res += 1;
            }
        }
    }
    return if alice.count == 1 && bob.count == 1 { res } else { -1 };
}

/*
并查集实现
 */
struct UnionFind {
    parent: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        let mut union_find = UnionFind { parent: vec![0; size], count: size };
        for i in 0..size {
            union_find.parent[i] = i;
        }
        return union_find;
    }

    fn find(&mut self, x: usize) -> usize {
        //声明一个新的x变量
        let mut x = x;
        while self.parent[x] != x {
            //路径压缩
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        return self.parent[x];
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rootx = self.find(x);
        let rooty = self.find(y);
        return if rootx != rooty {
            self.parent[rootx] = rooty;
            self.count -= 1;
            true
        } else {
            false
        };
    }
}

#[test]
fn test() {
    let edges = vec![
        vec![3, 1, 2],
        vec![3, 2, 3],
        vec![1, 1, 3],
        vec![1, 2, 4],
        vec![1, 1, 2],
        vec![2, 3, 4]
    ];
    let res = max_num_edges_to_remove(4, edges);
    println!("{}", res);
}

