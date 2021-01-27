pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    if (connections.len() as i32) < n - 1 {
        return -1;
    }
    let mut union_find = UnionFind::new(n);
    for conn in connections {
        union_find.unite(conn[0], conn[1]);
    }
    union_find.set_count - 1
}

struct UnionFind {
    parent: Vec<i32>,
    size : Vec<i32>,
    pub set_count : i32,
}

impl UnionFind {
    fn new(n: i32) -> UnionFind {
        let parent = (0..n).collect();
        let size = vec![1; n as usize];
        let set_count = n;
        UnionFind {
            parent,
            size,
            set_count,
        }
    }

    fn find(&mut self, x : i32) -> i32{
        let x = x as usize;
        if x as i32 != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn unite(&mut self, x: i32, y :i32) {
        let x = self.find(x) as usize;
        let y = self.find(y) as usize;
        if x == y {
            return;
        }

        if self.size[x] < self.size[y] {
            let _tmp = x;
            let x = y;
            let y = x;
        }
        self.parent[y] = x as i32;
        self.set_count -= 1;
        self.size[x] += self.size[y];
    }
}
