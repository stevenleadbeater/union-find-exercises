struct QuickFind {
    internal: Vec<i64>
}

impl QuickFind {
    fn new(size: i64) -> QuickFind {
        QuickFind { internal: (0..size).collect() }
    }

    fn union(&mut self, start: i64, end: i64) {
        self.internal = self.internal.iter().map(|item| {
            if item.eq(&end) {
                let target: i64 = self.internal.get(start as usize).unwrap().clone();
                return target;
            }
            item.clone()
        }).collect();
    }

    fn connected(&self, start: i64, end: i64) -> bool {
        self
            .internal
            .get(start as usize)
            .unwrap()
            .eq(self.internal.get(end as usize).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::quick_find::QuickFind;

    #[test]
    fn new() {
        let answer = QuickFind::new(10);
        assert_eq!(answer.internal.len(), 10)
    }

    #[test]
    fn new_contents() {
        let answer = QuickFind::new(10);
        let expected: Vec<i64> = (0..10).collect();
        assert_eq!(answer.internal, expected)
    }

    #[test]
    fn union_1_2() {
        let mut answer = QuickFind::new(10);
        answer.union(1 as i64, 2 as i64);
        assert_eq!(answer.internal, vec![0, 1, 1, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn union_1_2_3_4() {
        let mut answer = QuickFind::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        assert_eq!(answer.internal, vec![0, 1, 1, 3, 3, 5, 6, 7, 8, 9])
    }

    #[test]
    fn union_1_2_3_4_8_3() {
        let mut answer = QuickFind::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        assert_eq!(answer.internal, vec![0, 1, 1, 8, 8, 5, 6, 7, 8, 9])
    }

    #[test]
    fn union_1_2_3_4_8_3_4_9() {
        let mut answer = QuickFind::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        answer.union(4 as i64, 9 as i64);
        assert_eq!(answer.internal, vec![0, 1, 1, 8, 8, 5, 6, 7, 8, 8])
    }

    #[test]
    fn union_1_2_3_4_8_3_4_9_5_6() {
        let mut answer = QuickFind::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        answer.union(4 as i64, 9 as i64);
        answer.union(5 as i64, 6 as i64);
        assert_eq!(answer.internal, vec![0, 1, 1, 8, 8, 5, 5, 7, 8, 8])
    }

    #[test]
    fn connected_8_9() {
        let mut answer = QuickFind::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        answer.union(4 as i64, 9 as i64);
        answer.union(5 as i64, 6 as i64);
        assert!(answer.connected(8, 9))
    }

    #[test]
    fn connected_5_0() {
        let mut answer = QuickFind::new(10);
        answer.union(1 as i64, 2 as i64);
        answer.union(3 as i64, 4 as i64);
        answer.union(8 as i64, 3 as i64);
        answer.union(4 as i64, 9 as i64);
        answer.union(5 as i64, 6 as i64);
        assert!(!answer.connected(5, 0))
    }
}