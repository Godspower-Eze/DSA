fn check_group_validity(board: &Vec<Vec<char>>) -> bool {
    for group in board {
        let mut seen = HashSet::new();
        for &ele in group {
            if ele == '.' {
                continue;
            }
            if !ele.is_ascii_digit() || ele == '0' || seen.contains(&ele) {
                return false;
            }
            seen.insert(ele);
        }
    }
    true
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // rows
        let row_validity = check_group_validity(&board);

        // columns (fixed transpose)
        let mut col_board = vec![];
        for i in 0..9 {
            let mut col = vec!['.'; 9];
            for (j, row) in board.iter().enumerate() {
                col[j] = row[i];
            }
            col_board.push(col);
        }
        let column_validity = check_group_validity(&col_board);

        // sub-boxes
        let mut box_board = vec![];
        for i in (0..9).step_by(3) {
            let first = &board[i];
            let second = &board[i + 1];
            let third = &board[i + 2];
            for j in (0..9).step_by(3) {
                let b = vec![
                    first[j], first[j + 1], first[j + 2],
                    second[j], second[j + 1], second[j + 2],
                    third[j], third[j + 1], third[j + 2],
                ];
                box_board.push(b);
            }
        }
        let box_validity = check_group_validity(&box_board);

        row_validity && column_validity && box_validity
    }
}