pub struct Direction {
    height: isize,
    width: isize,
    target_length: isize,
}

impl Direction {
    pub fn new(height: usize, width: usize, target_length: usize) -> Self {
        // Height, width, and the length of the target string are all one-indexed.
        // That is, the string "ALAN" has a length of 4.
        //
        // However, when we work with rows/columns those are zero-indexed.
        // That is, the "N" in "ALAN" is at index 3.
        //
        // This is all fine, but it can get very confusing very fast, so it might
        // be conducive to pick one (zero-versus-one indexing) and to stick to it.
        //
        // We COULD add 1 to the rows/columns, however doing so would require
        // that we add 1 to those values potentially millions of times (depending
        // on the dimensions of the provided puzzle).
        //
        // It would be far more efficient to subtract 1 from the dimensions once
        // at the beginning of the procedure.
        //
        // Thus, we choose to coerce all values to zero-indexing.
        Self {
            height: height as isize - 1,
            width: width as isize - 1,
            target_length: target_length as isize - 1,
        }
    }

    pub fn north(&self, row: usize) -> bool {
        // target_length == 3
        // height = 3
        // width = 3
        //       0    1    2    3
        // vec!['a', 'l', 'a', 'n'] // 0
        // vec!['a', 'l', 'l', 'a'] // 1
        // vec!['a', 'l', 'a', 'l'] // 2
        // vec!['a', 'l', 'n', 'a'] // 3
        (row as isize) - self.target_length >= 0
    }

    pub fn south(&self, row: usize) -> bool {
        // target_length == 3
        // height = 3
        // width = 3
        //       0    1    2    3
        // vec!['a', 'l', 'a', 'n'] // 0
        // vec!['a', 'l', 'l', 'a'] // 1
        // vec!['a', 'l', 'a', 'l'] // 2
        // vec!['a', 'l', 'n', 'a'] // 3
        (row as isize) + self.target_length <= self.height
    }

    pub fn west(&self, column: usize) -> bool {
        // target_length == 3
        // height = 3
        // width = 3
        //       0    1    2    3
        // vec!['a', 'l', 'a', 'n'] // 0
        // vec!['a', 'l', 'l', 'a'] // 1
        // vec!['a', 'l', 'a', 'l'] // 2
        // vec!['a', 'l', 'n', 'a'] // 3
        (column as isize) - self.target_length >= 0
    }

    pub fn east(&self, column: usize) -> bool {
        // target_length == 3
        // height = 3
        // width = 3
        //       0    1    2    3
        // vec!['a', 'l', 'a', 'n'] // 0
        // vec!['a', 'l', 'l', 'a'] // 1
        // vec!['a', 'l', 'a', 'l'] // 2
        // vec!['a', 'l', 'n', 'a'] // 3
        (column as isize) + self.target_length <= self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_by_even_by_even() {
        let height = 4;
        let width = 4;
        let length = 4;
        let direction = Direction::new(height, width, length);
        // North
        assert!(!direction.north(0));
        assert!(!direction.north(1));
        assert!(!direction.north(2));
        assert!(direction.north(3));
        // South
        assert!(direction.south(0));
        assert!(!direction.south(1));
        assert!(!direction.south(2));
        assert!(!direction.south(3));
        // East
        assert!(direction.east(0));
        assert!(!direction.east(1));
        assert!(!direction.east(2));
        assert!(!direction.east(3));
        // West
        assert!(!direction.west(0));
        assert!(!direction.west(1));
        assert!(!direction.west(2));
        assert!(direction.west(3));
    }

    #[test]
    fn even_by_even_by_odd() {
        let height = 4;
        let width = 4;
        let length = 3;
        let direction = Direction::new(height, width, length);
        // North
        assert!(!direction.north(0));
        assert!(!direction.north(1));
        assert!(direction.north(2));
        assert!(direction.north(3));
        // South
        assert!(direction.south(0));
        assert!(direction.south(1));
        assert!(!direction.south(2));
        assert!(!direction.south(3));
        // East
        assert!(direction.east(0));
        assert!(direction.east(1));
        assert!(!direction.east(2));
        assert!(!direction.east(3));
        // West
        assert!(!direction.west(0));
        assert!(!direction.west(1));
        assert!(direction.west(2));
        assert!(direction.west(3));
    }

    #[test]
    fn even_by_odd_by_even() {
        let height = 4;
        let width = 5;
        let length = 4;
        let direction = Direction::new(height, width, length);
        // North
        assert!(!direction.north(0));
        assert!(!direction.north(1));
        assert!(!direction.north(2));
        assert!(direction.north(3));
        // South
        assert!(direction.south(0));
        assert!(!direction.south(1));
        assert!(!direction.south(2));
        assert!(!direction.south(3));
        // East
        assert!(direction.east(0));
        assert!(direction.east(1));
        assert!(!direction.east(2));
        assert!(!direction.east(3));
        assert!(!direction.east(4));
        // West
        assert!(!direction.west(0));
        assert!(!direction.west(1));
        assert!(!direction.west(2));
        assert!(direction.west(3));
        assert!(direction.west(4));
    }

    #[test]
    fn odd_by_even_by_even() {
        let height = 5;
        let width = 4;
        let length = 4;
        let direction = Direction::new(height, width, length);
        // North
        assert!(!direction.north(0));
        assert!(!direction.north(1));
        assert!(!direction.north(2));
        assert!(direction.north(3));
        assert!(direction.north(4));
        // South
        assert!(direction.south(0));
        assert!(direction.south(1));
        assert!(!direction.south(2));
        assert!(!direction.south(3));
        assert!(!direction.south(4));
        // East
        assert!(direction.east(0));
        assert!(!direction.east(1));
        assert!(!direction.east(2));
        assert!(!direction.east(3));
        // West
        assert!(!direction.west(0));
        assert!(!direction.west(1));
        assert!(!direction.west(2));
        assert!(direction.west(3));
    }

    #[test]
    fn even_by_odd_by_odd() {
        let height = 6;
        let width = 5;
        let length = 5;
        let direction = Direction::new(height, width, length);
        // North
        assert!(!direction.north(0));
        assert!(!direction.north(1));
        assert!(!direction.north(2));
        assert!(!direction.north(3));
        assert!(direction.north(4));
        assert!(direction.north(5));
        // South
        assert!(direction.south(0));
        assert!(direction.south(1));
        assert!(!direction.south(2));
        assert!(!direction.south(3));
        assert!(!direction.south(4));
        assert!(!direction.south(5));
        // East
        assert!(direction.east(0));
        assert!(!direction.east(1));
        assert!(!direction.east(2));
        assert!(!direction.east(3));
        assert!(!direction.east(4));
        // West
        assert!(!direction.west(0));
        assert!(!direction.west(1));
        assert!(!direction.west(2));
        assert!(!direction.west(3));
        assert!(direction.west(4));
    }

    #[test]
    fn odd_by_even_by_odd() {
        let height = 5;
        let width = 6;
        let length = 5;
        let direction = Direction::new(height, width, length);
        // North
        assert!(!direction.north(0));
        assert!(!direction.north(1));
        assert!(!direction.north(2));
        assert!(!direction.north(3));
        assert!(direction.north(4));
        // South
        assert!(direction.south(0));
        assert!(!direction.south(1));
        assert!(!direction.south(2));
        assert!(!direction.south(3));
        assert!(!direction.south(4));
        // East
        assert!(direction.east(0));
        assert!(direction.east(1));
        assert!(!direction.east(2));
        assert!(!direction.east(3));
        assert!(!direction.east(4));
        assert!(!direction.east(5));
        // West
        assert!(!direction.west(0));
        assert!(!direction.west(1));
        assert!(!direction.west(2));
        assert!(!direction.west(3));
        assert!(direction.west(4));
        assert!(direction.west(5));
    }

    #[test]
    fn odd_by_odd_by_odd() {
        let height = 3;
        let width = 3;
        let length = 3;
        let direction = Direction::new(height, width, length);
        // North
        assert!(!direction.north(0));
        assert!(!direction.north(1));
        assert!(direction.north(2));
        // South
        assert!(direction.south(0));
        assert!(!direction.south(1));
        assert!(!direction.south(2));
        // East
        assert!(direction.east(0));
        assert!(!direction.east(1));
        assert!(!direction.east(2));
        // West
        assert!(!direction.west(0));
        assert!(!direction.west(1));
        assert!(direction.west(2));
    }

    #[test]
    fn cant_ever_fit() {
        let height = 4;
        let width = 4;
        let length = 5;
        let direction = Direction::new(height, width, length);
        // North
        assert!(!direction.north(0));
        assert!(!direction.north(1));
        assert!(!direction.north(2));
        assert!(!direction.north(3));
        // South
        assert!(!direction.south(0));
        assert!(!direction.south(1));
        assert!(!direction.south(2));
        assert!(!direction.south(3));
        // East
        assert!(!direction.east(0));
        assert!(!direction.east(1));
        assert!(!direction.east(2));
        assert!(!direction.east(3));
        // West
        assert!(!direction.west(0));
        assert!(!direction.west(1));
        assert!(!direction.west(2));
        assert!(!direction.west(3));
    }
}
