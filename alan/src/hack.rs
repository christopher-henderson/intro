fn solve(puzzle: Vec<Vec<char>>, target: impl AsRef<str>) -> u64 {
    let target: Vec<char> = target.as_ref().chars().collect();
    let target_length = target.len();
    let width = puzzle.first().map_or_else(|| 0, Vec::len);
    let height = puzzle.len();
    let mut count: u64 = 0;
    for (row_index, row) in puzzle.iter().enumerate() {
        let check_north = (row_index as isize) - (target_length as isize) > 0;
        let check_south = (row_index as isize) + (target_length as isize) <= height as isize;
        for (column_index, column) in row.iter().enumerate() {
            if *column != target[0] {
                continue;
            }
            let check_east = (column_index as isize) - (target_length as isize) > 0;
            let check_west = (column_index as isize) + (target_length as isize) <= width as isize;
            if check_east
                && row[column_index - target_length..column_index]
                    .iter()
                    .rev()
                    .eq(target.iter())
            {
                count += 1;
            }
            if check_west && row[column_index..column_index + target_length] == target {
                count += 1;
            }
            if check_north
                && (row_index - target_length..row_index)
                    .rev()
                    .map(|r| &puzzle[r][column_index])
                    .eq(target.iter())
            {
                count += 1
            }
            if check_south
                && (row_index..row_index + target_length)
                    .map(|r| &puzzle[r][column_index])
                    .eq(target.iter())
            {
                count += 1
            }
        }
    }
    count
}
