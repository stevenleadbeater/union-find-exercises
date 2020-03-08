struct Percolation {
    size: i64,
    grid: Vec<Vec<(i64, i64)>>,
    sizes: Vec<Vec<i64>>,
}

impl Percolation {
    fn new(size: i64) -> Percolation {
        let mut result = Percolation {
            size,
            grid: Vec::new(),
            sizes: Vec::new(),
        };
        for x in 0..size {
            result.grid.push(Vec::new());
            result.sizes.push(Vec::new());
            for y in 0..size {
                result.grid[x as usize].push((x, y));
                result.sizes[x as usize].push(0);
            }
        }
        result.grid.push(Vec::new());
        result.sizes.push(Vec::new());
        // virtual top
        result.grid[size as usize].push((size, 0));
        result.sizes[size as usize].push(0);
        // virtual bottom
        result.grid[size as usize].push((size, 1));
        result.sizes[size as usize].push(0);
        for x in 0..size {
            result.union((size, 0), (x, 0));
            result.union((size, 1), (x, size - 1));
        }
        result
    }

    fn union(&mut self, start: (i64, i64), end: (i64, i64)) {
        let start_root = self.get_root(start);
        let end_root = self.get_root(end);

        if self.sizes[start_root.0 as usize][start_root.1 as usize] >= self.sizes[end_root.0 as usize][end_root.1 as usize] {
            self.grid[end_root.0 as usize][end_root.1 as usize] = start_root;
            self.sizes[start_root.0 as usize][start_root.1 as usize] += 1;
        } else {
            self.grid[start_root.0 as usize][start_root.1 as usize] = end_root;
            self.sizes[end_root.0 as usize][end_root.1 as usize] += 1
        }
    }

    fn connected(&mut self, start: (i64, i64), end: (i64, i64)) -> bool {
        let start_root = self.get_root(start);
        let end_root = self.get_root(end);
        start_root.0 == end_root.0 && start_root.1 == end_root.1
    }

    fn get_root(&mut self, item: (i64, i64)) -> (i64, i64) {
        let mut result = self.grid[item.0 as usize][item.1 as usize];
        while result.0 != self.grid[result.0 as usize][result.1 as usize].0
            && result.1 != self.grid[result.0 as usize][result.1 as usize].1 {
            self.grid[result.0 as usize][result.1 as usize] =
                self.grid
                    [self.grid[result.0 as usize][result.1 as usize].0 as usize]
                    [self.grid[result.0 as usize][result.1 as usize].1 as usize];
            result = self.grid[result.0 as usize][result.1 as usize];
        }
        result
    }

    fn percolates(&mut self) -> bool {
        self.connected((self.size, 0), (self.size, 1))
    }
}

#[cfg(test)]
mod test {
    use crate::percolation::Percolation;

    #[test]
    pub fn new() {
        let percolation = Percolation::new(5);
        assert_eq!(percolation.grid[0][0], (5, 0));
        assert_eq!(percolation.grid[0][1], (0, 1));
        assert_eq!(percolation.grid[0][2], (0, 2));
        assert_eq!(percolation.grid[0][3], (0, 3));
        assert_eq!(percolation.grid[0][4], (5, 1));
        assert_eq!(percolation.grid[1][0], (5, 0));
        assert_eq!(percolation.grid[1][1], (1, 1));
        assert_eq!(percolation.grid[1][2], (1, 2));
        assert_eq!(percolation.grid[1][3], (1, 3));
        assert_eq!(percolation.grid[1][4], (5, 1));
        assert_eq!(percolation.grid[2][0], (5, 0));
        assert_eq!(percolation.grid[2][1], (2, 1));
        assert_eq!(percolation.grid[2][2], (2, 2));
        assert_eq!(percolation.grid[2][3], (2, 3));
        assert_eq!(percolation.grid[2][4], (5, 1));
        assert_eq!(percolation.grid[3][0], (5, 0));
        assert_eq!(percolation.grid[3][1], (3, 1));
        assert_eq!(percolation.grid[3][2], (3, 2));
        assert_eq!(percolation.grid[3][3], (3, 3));
        assert_eq!(percolation.grid[3][4], (5, 1));
        assert_eq!(percolation.grid[4][0], (5, 0));
        assert_eq!(percolation.grid[4][1], (4, 1));
        assert_eq!(percolation.grid[4][2], (4, 2));
        assert_eq!(percolation.grid[4][3], (4, 3));
        assert_eq!(percolation.grid[4][4], (5, 1));
    }

    #[test]
    fn percolates_column_1() {
        let mut percolation = Percolation::new(5);
        percolation.union((0, 0), (0, 1));
        percolation.union((0, 1), (0, 2));
        percolation.union((0, 2), (0, 3));
        percolation.union((0, 3), (0, 4));
        assert_eq!(percolation.percolates(), true);
    }

    #[test]
    fn percolates_column_2() {
        let mut percolation = Percolation::new(5);
        percolation.union((1, 0), (1, 1));
        percolation.union((1, 1), (1, 2));
        percolation.union((1, 2), (1, 3));
        percolation.union((1, 3), (1, 4));
        assert_eq!(percolation.percolates(), true);
    }

    #[test]
    fn percolates_column_3() {
        let mut percolation = Percolation::new(5);
        percolation.union((2, 0), (2, 1));
        percolation.union((2, 1), (2, 2));
        percolation.union((2, 2), (2, 3));
        percolation.union((2, 3), (2, 4));
        assert_eq!(percolation.percolates(), true);
    }

    #[test]
    fn percolates_column_4() {
        let mut percolation = Percolation::new(5);
        percolation.union((3, 0), (3, 1));
        percolation.union((3, 1), (3, 2));
        percolation.union((3, 2), (3, 3));
        percolation.union((3, 3), (3, 4));
        assert_eq!(percolation.percolates(), true);
    }

    #[test]
    fn percolates_column_5() {
        let mut percolation = Percolation::new(5);
        percolation.union((4, 0), (4, 1));
        percolation.union((4, 1), (4, 2));
        percolation.union((4, 2), (4, 3));
        percolation.union((4, 3), (4, 4));
        assert_eq!(percolation.percolates(), true);
    }

    #[test]
    fn percolates_2() {
        let mut percolation = Percolation::new(5);
        percolation.union((0, 0), (0, 1));
        percolation.union((0, 1), (1, 1));
        percolation.union((1, 1), (1, 2));
        percolation.union((1, 2), (1, 3));
        percolation.union((1, 3), (1, 4));
        assert_eq!(percolation.percolates(), true);
    }

    #[test]
    fn percolates_3() {
        let mut percolation = Percolation::new(5);
        percolation.union((0, 0), (0, 1));
        percolation.union((0, 1), (1, 1));
        percolation.union((1, 1), (2, 1));
        percolation.union((2, 1), (2, 2));
        percolation.union((2, 2), (2, 3));
        percolation.union((2, 3), (2, 4));
        assert_eq!(percolation.percolates(), true);
    }

    #[test]
    fn percolates_4() {
        let mut percolation = Percolation::new(5);
        percolation.union((0, 0), (0, 1));
        percolation.union((0, 1), (1, 1));
        percolation.union((1, 1), (2, 1));
        percolation.union((2, 1), (3, 1));
        percolation.union((3, 1), (3, 2));
        percolation.union((3, 2), (3, 3));
        percolation.union((3, 3), (3, 4));
        assert_eq!(percolation.percolates(), true);
    }

    #[test]
    fn percolates_5() {
        let mut percolation = Percolation::new(5);
        percolation.union((0, 0), (0, 1));
        percolation.union((0, 1), (1, 1));
        percolation.union((1, 1), (2, 1));
        percolation.union((2, 1), (3, 1));
        percolation.union((3, 1), (4, 1));
        percolation.union((4, 1), (4, 2));
        percolation.union((4, 2), (4, 3));
        percolation.union((4, 3), (4, 4));
        assert_eq!(percolation.percolates(), true);
    }

    #[test]
    fn does_not_percolate() {
        let mut percolation = Percolation::new(5);
        percolation.union((0, 0), (0, 1));
        percolation.union((0, 3), (0, 4));

        percolation.union((1, 0), (1, 1));
        percolation.union((1, 3), (1, 4));

        percolation.union((2, 0), (2, 1));
        percolation.union((2, 3), (2, 4));

        percolation.union((3, 0), (3, 1));
        percolation.union((3, 3), (3, 4));

        percolation.union((4, 0), (4, 1));
        percolation.union((4, 3), (4, 4));

        assert_eq!(percolation.percolates(), false);
    }
}