fn main() {
    println!("{:?}", minesweeper(
        vec![vec![true,false,false,true],
             vec![false,false,true,false],
             vec![true,true,false,true]]
    ));
}

fn minesweeper(matrix: Vec<Vec<bool>>) -> Vec<Vec<i32>> {
    let row = matrix.len();
    let col = matrix[0].len();
    let r = (row - 1) as i32;
    let c = (col - 1) as i32;
    let ics: Vec<(i32, i32)> = vec![
        (-1,-1), (-1,0), (-1,1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];
    let mut result = vec![vec![0i32; col]; row];

    for i in 0 .. row {
        for j in 0 .. col {
            let x = i as i32;
            let y = j as i32;
            for (p, q) in ics.clone().into_iter() {
                let idx1 = x + p;
                let idx2 = y + q;
                if in_range(idx1, 0, r) && in_range(idx2, 0, c) {
                    if matrix[idx1 as usize][idx2 as usize] {
                        result[i][j] += 1;
                    }
                }
            }
        }
    }
    result
}

fn in_range(x: i32, min: i32, max: i32) -> bool {
    if x < min || x > max {
        false
    } else {
        true
    }
}
