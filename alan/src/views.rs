pub struct Matrix<T> {
    pub inner: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(inner: Vec<Vec<T>>) -> Self {
        Self { inner }
    }
}

impl<T: Eq> Matrix<T> {
    pub fn height(&self) -> usize {
        self.inner.len()
    }

    pub fn width(&self) -> usize {
        self.inner.first().unwrap().len()
    }

    pub fn candidate(&self, row: usize, column: usize, start: T) -> bool {
        self.inner.get(row).unwrap().get(column).unwrap() == &start
    }
}

impl<T: Copy> Matrix<T> {
    pub fn north(&self, row: usize, column: usize, want: usize) -> North<'_, T> {
        North::new(self, row, column, want)
    }

    pub fn south(&self, row: usize, column: usize, want: usize) -> South<'_, T> {
        South::new(self, row, column, want)
    }

    pub fn east(&self, row: usize, column: usize, want: usize) -> East<'_, T> {
        East::new(self, row, column, want)
    }

    pub fn west(&self, row: usize, column: usize, want: usize) -> West<'_, T> {
        West::new(self, row, column, want)
    }
}

pub struct North<'a, T> {
    matrix: &'a Matrix<T>,
    row: usize,
    column: usize,
    taken: usize,
    want: usize,
}

impl<'a, T> North<'a, T> {
    pub fn new(matrix: &'a Matrix<T>, row: usize, column: usize, want: usize) -> Self {
        Self {
            matrix,
            row,
            column,
            want,
            taken: 0,
        }
    }
}

impl<'a, T: Copy> Iterator for North<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.taken == self.want {
            return None;
        }
        let got = self
            .matrix
            .inner
            .get(self.row - self.taken)
            .unwrap()
            .get(self.column)
            .copied();
        self.taken += 1;
        got
    }
}

pub struct South<'a, T> {
    matrix: &'a Matrix<T>,
    row: usize,
    column: usize,
    taken: usize,
    want: usize,
}

impl<'a, T> South<'a, T> {
    pub fn new(matrix: &'a Matrix<T>, row: usize, column: usize, want: usize) -> Self {
        Self {
            matrix,
            row,
            column,
            want,
            taken: 0,
        }
    }
}

impl<'a, T: Copy> Iterator for South<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.taken == self.want {
            return None;
        }
        let got = self
            .matrix
            .inner
            .get(self.row + self.taken)
            .unwrap()
            .get(self.column)
            .copied();
        self.taken += 1;
        got
    }
}

pub struct East<'a, T> {
    matrix: &'a Matrix<T>,
    row: usize,
    column: usize,
    taken: usize,
    want: usize,
}

impl<'a, T> East<'a, T> {
    pub fn new(matrix: &'a Matrix<T>, row: usize, column: usize, want: usize) -> Self {
        Self {
            matrix,
            row,
            column,
            want,
            taken: 0,
        }
    }
}

impl<'a, T: Copy> Iterator for East<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.taken == self.want {
            return None;
        }
        let got = self
            .matrix
            .inner
            .get(self.row)
            .unwrap()
            .get(self.column + self.taken)
            .copied();
        self.taken += 1;
        got
    }
}

pub struct West<'a, T> {
    matrix: &'a Matrix<T>,
    row: usize,
    column: usize,
    taken: usize,
    want: usize,
}

impl<'a, T> West<'a, T> {
    pub fn new(matrix: &'a Matrix<T>, row: usize, column: usize, want: usize) -> Self {
        Self {
            matrix,
            row,
            column,
            want,
            taken: 0,
        }
    }
}

impl<'a, T: Copy> Iterator for West<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.taken == self.want {
            return None;
        }
        let got = self
            .matrix
            .inner
            .get(self.row)
            .unwrap()
            .get(self.column - self.taken)
            .copied();
        self.taken += 1;
        got
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn north() {
        let matrix: Matrix<char> = vec![
            // n == 4
            // height = 5
            // width = 5
            //    0    1    2    3    4
            vec!['a', 'l', 'a', 'n', 'a'], // 0
            vec!['a', 'l', 'l', 'n', 'l'], // 1
            vec!['a', 'l', 'a', 'n', 'a'], // 2
            vec!['a', 'l', 'n', 'n', 'n'], // 3
            vec!['a', 'l', 'a', 'n', 'p'], // 4
        ]
        .into();
        let north = matrix.north(4, 4, 5);
        if !north.eq("pnala".chars()) {
            panic!("balls")
        }
    }

    #[test]
    fn south() {
        let matrix: Matrix<char> = vec![
            // n == 4
            // height = 5
            // width = 5
            //    0    1    2    3    4
            vec!['a', 'l', 'a', 'n', 'a'], // 0
            vec!['a', 'l', 'l', 'n', 'l'], // 1
            vec!['a', 'l', 'a', 'n', 'a'], // 2
            vec!['a', 'l', 'n', 'n', 'n'], // 3
            vec!['a', 'l', 'a', 'n', 'p'], // 4
        ]
        .into();
        let south = matrix.south(0, 4, 4);
        if !south.eq("alan".chars()) {
            panic!("balls")
        }
    }

    #[test]
    fn east() {
        let matrix: Matrix<char> = vec![
            // n == 4
            // height = 5
            // width = 5
            //    0    1    2    3    4
            vec!['a', 'l', 'a', 'n', 'a'], // 0
            vec!['a', 'l', 'l', 'n', 'l'], // 1
            vec!['a', 'l', 'a', 'n', 'a'], // 2
            vec!['a', 'l', 'n', 'n', 'n'], // 3
            vec!['a', 'l', 'a', 'n', 'p'], // 4
        ]
        .into();
        let east = matrix.east(0, 0, 4);
        if !east.eq("alan".chars()) {
            panic!("balls")
        }
    }

    #[test]
    fn west() {
        let matrix: Matrix<char> = vec![
            // n == 4
            // height = 5
            // width = 5
            //    0    1    2    3    4
            vec!['a', 'l', 'a', 'n', 'a'], // 0
            vec!['a', 'l', 'l', 'n', 'l'], // 1
            vec!['a', 'l', 'a', 'n', 'a'], // 2
            vec!['a', 'l', 'n', 'n', 'n'], // 3
            vec!['a', 'l', 'a', 'n', 'p'], // 4
        ]
        .into();
        let east = matrix.west(2, 3, 4);
        if !east.eq("nala".chars()) {
            panic!("balls")
        }
    }
}
