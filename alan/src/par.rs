use std::sync::Arc;

pub fn par_solve(puzzle: Vec<Vec<char>>, target: impl AsRef<str>) -> u64 {
    // par setup
    let num_cpus = std::thread::available_parallelism().unwrap();
    let (sender, receiver) = crossbeam::channel::bounded(puzzle.len());
    let mut handles = vec![];
    // Algo setup
    let target: Arc<Vec<char>> = Arc::new(target.as_ref().chars().collect());
    let puzzle = Arc::new(puzzle);
    let width = puzzle.first().map_or_else(|| 0, Vec::len);
    let height = puzzle.len();
    let n = target.len();
    for _ in 0..puzzle.len().min(num_cpus.into()) {
        let receiver = receiver.clone();
        let puzzle = puzzle.clone();
        let target = target.clone();
        let join_handle = std::thread::spawn(move || {
            let i = receiver.recv().unwrap();
            let row: &Vec<char> = &puzzle[i];
            let check_north = (i as isize) - (n as isize) > 0;
            let check_south = (i as isize) + (n as isize) <= height as isize;
            let mut count: u64 = 0;
            for (j, column) in row.iter().enumerate() {
                if *column != target[0] {
                    continue;
                }
                let check_east = (j as isize) - (n as isize) > 0;
                let check_west = (j as isize) + (n as isize) <= width as isize;
                if check_east && row[j - n..j].iter().rev().eq(target.iter()) {
                    count += 1;
                }
                if check_west && row[j..j + n] == *target {
                    count += 1;
                }
                if check_north
                    && (i - n..i)
                        .rev()
                        .map(|r| {
                            let asd: &Vec<char> = puzzle.get(r).unwrap();
                            let c = asd.get(j).unwrap();
                            c
                        })
                        .eq(target.iter())
                {
                    count += 1
                }
                if check_south
                    && (i..i + n)
                        .map(|r| {
                            let asd: &Vec<char> = puzzle.get(r).unwrap();
                            let c = asd.get(j).unwrap();
                            c
                        })
                        .eq(target.iter())
                {
                    count += 1
                }
            }
            count
        });
        handles.push(join_handle);
    }
    for row in 0..puzzle.len() {
        sender.send(row).unwrap();
    }
    let mut count = 0;
    for handle in handles {
        count += handle.join().unwrap();
    }
    count
}
