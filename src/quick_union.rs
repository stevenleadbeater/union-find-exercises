struct QuickUnion {
    internal: Vec<i64>
}

impl QuickUnion {
    fn new(size: i64) -> QuickUnion {
        QuickUnion {
            internal: (0..size).collect()
        }
    }
    
    fn union(&mut self, start: i64, end: i64) {
        let start_root: i64 = self.get_root(start);
        self.internal[start_root as usize] = self.get_root(end);
    }
    
    fn connected(&self, start: i64, end: i64) -> bool {
        let start_root: i64 = self.get_root(start);
        let end_root: i64 = self.get_root(end);
        end_root == start_root
    }

    fn get_root(&self, item: i64) -> i64 {
        let mut root: i64 = self.internal[item as usize];
        while root != self.internal[root as usize] {
            root = self.internal[root as usize];
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use crate::quick_union::QuickUnion;

    #[test]
    fn new() {
        let answer = QuickUnion::new(10);
        assert_eq!(answer.internal.len(), 10)
    }

    #[test]
    fn new_contents() {
        let answer = QuickUnion::new(10);
        let expected: Vec<i64> = (0..10).collect();
        assert_eq!(answer.internal, expected)
    }

    #[test]
    fn union_1_2() {
        let mut answer = QuickUnion::new(10);
        answer.union(1 as i64, 2 as i64);
        assert_eq!(answer.internal, vec![0, 2, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn union_1_2_3_4() {
        let mut answer = QuickUnion::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        assert_eq!(answer.internal, vec![0, 2, 2, 4, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn union_1_2_3_4_8_3() {
        let mut answer = QuickUnion::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        assert_eq!(answer.internal, vec![0, 2, 2, 4, 4, 5, 6, 7, 4, 9])
    }

    #[test]
    fn union_1_2_3_4_8_3_4_9() {
        let mut answer = QuickUnion::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        answer.union(4 as i64, 9 as i64);
        assert_eq!(answer.internal, vec![0, 2, 2, 4, 9, 5, 6, 7, 4, 9])
    }

    #[test]
    fn union_1_2_3_4_8_3_4_9_5_6() {
        let mut answer = QuickUnion::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        answer.union(4 as i64, 9 as i64);
        answer.union(5 as i64, 6 as i64);
        assert_eq!(answer.internal, vec![0, 2, 2, 4, 9, 6, 6, 7, 4, 9])
    }

    #[test]
    fn connected_8_9() {
        let mut answer = QuickUnion::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        answer.union(4 as i64, 9 as i64);
        answer.union(5 as i64, 6 as i64);
        assert!(answer.connected(8, 9))
    }

    #[test]
    fn connected_5_0() {
        let mut answer = QuickUnion::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        answer.union(4 as i64, 9 as i64);
        answer.union(5 as i64, 6 as i64);
        assert!(!answer.connected(5, 0))
    }
}