use hello_cargo::{format_grid, is_valid, is_valid_solution, solve_sudoku, Grid};

#[test]
fn test_is_valid_row() {
    let grid: Grid = [
        [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
    ];
    // 同じ行に既に1があるので、1は配置できない
    assert!(!is_valid(&grid, 0, 0, 1));
    // 行0には既に1があるので、行0の他の位置に1は配置できない
    assert!(!is_valid(&grid, 0, 1, 1));
    // 行1には1がない、列3にも1がない、ボックス(0,1)にも1がないので、1は配置できる
    assert!(is_valid(&grid, 1, 3, 1));
}

#[test]
fn test_is_valid_column() {
    let grid: Grid = [
        [Some(1),    None,    None,    None,    None,    None,    None,    None, None],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
    ];
    // 同じ列に既に1があるので、1は配置できない
    assert!(!is_valid(&grid, 1, 0, 1));
    // 列0には既に1があるので、列0の他の位置に1は配置できない
    assert!(!is_valid(&grid, 2, 0, 1));
    // 列3には1がない、行3にも1がない、ボックス(1,1)にも1がないので、1は配置できる
    assert!(is_valid(&grid, 3, 3, 1));
}

#[test]
fn test_is_valid_box() {
    let grid: Grid = [
        [Some(1),    None,    None,    None,    None,    None,    None,    None, None],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
        [None; 9],
    ];
    // 同じ3x3ボックスに既に1があるので、1は配置できない
    assert!(!is_valid(&grid, 1, 1, 1));
    // ボックス(0,1)には1がない、行3にも1がない、列3にも1がないので、1は配置できる
    assert!(is_valid(&grid, 3, 3, 1));
    // ボックス(0,0)には既に1があるので、ボックス(0,0)の他の位置に1は配置できない
    assert!(!is_valid(&grid, 2, 2, 1));
}

#[test]
fn test_solve_easy_sudoku() {
    let mut grid: Grid = [
        [Some(5), Some(3),    None,    None, Some(7),    None,    None,    None, None],
        [Some(6),    None,    None, Some(1), Some(9), Some(5),    None,    None, None],
        [   None, Some(9), Some(8),    None,    None,    None,    None, Some(6), None],
        [Some(8),    None,    None,    None, Some(6),    None,    None,    None, Some(3)],
        [Some(4),    None,    None, Some(8),    None, Some(3),    None,    None, Some(1)],
        [Some(7),    None,    None,    None, Some(2),    None,    None,    None, Some(6)],
        [   None, Some(6),    None,    None,    None,    None, Some(2), Some(8), None],
        [   None,    None,    None, Some(4), Some(1), Some(9),    None,    None, Some(5)],
        [   None,    None,    None,    None, Some(8),    None,    None, Some(7), Some(9)],
    ];
    
    assert!(solve_sudoku(&mut grid));
    assert!(is_valid_solution(&grid));
}

#[test]
fn test_solve_hard_sudoku() {
    let mut grid: Grid = [
        [   None,    None,    None, Some(6),    None,    None, Some(4),    None, None],
        [Some(7),    None,    None,    None,    None, Some(3), Some(6),    None, None],
        [   None,    None,    None,    None, Some(9), Some(1),    None, Some(8), None],
        [None; 9],
        [   None, Some(5),    None, Some(1), Some(8),    None,    None,    None, Some(3)],
        [   None,    None,    None, Some(3),    None, Some(6),    None, Some(4), Some(5)],
        [   None, Some(4),    None, Some(2),    None,    None,    None, Some(6), None],
        [Some(9),    None, Some(3),    None,    None,    None,    None,    None, None],
        [   None, Some(2),    None,    None,    None,    None, Some(1),    None, None],
    ];
    
    assert!(solve_sudoku(&mut grid));
    assert!(is_valid_solution(&grid));
}

#[test]
fn test_solve_already_solved() {
    let mut grid: Grid = [
        [Some(5), Some(3), Some(4), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)],
        [Some(6), Some(7), Some(2), Some(1), Some(9), Some(5), Some(3), Some(4), Some(8)],
        [Some(1), Some(9), Some(8), Some(3), Some(4), Some(2), Some(5), Some(6), Some(7)],
        [Some(8), Some(5), Some(9), Some(7), Some(6), Some(1), Some(4), Some(2), Some(3)],
        [Some(4), Some(2), Some(6), Some(8), Some(5), Some(3), Some(7), Some(9), Some(1)],
        [Some(7), Some(1), Some(3), Some(9), Some(2), Some(4), Some(8), Some(5), Some(6)],
        [Some(9), Some(6), Some(1), Some(5), Some(3), Some(7), Some(2), Some(8), Some(4)],
        [Some(2), Some(8), Some(7), Some(4), Some(1), Some(9), Some(6), Some(3), Some(5)],
        [Some(3), Some(4), Some(5), Some(2), Some(8), Some(6), Some(1), Some(7), Some(9)],
    ];
    
    // 既に解かれている問題は解けるはず
    assert!(solve_sudoku(&mut grid));
    assert!(is_valid_solution(&grid));
}

#[test]
fn test_solve_unsolvable() {
    // 解けない問題（同じ行に同じ数字が2つある）
    let mut grid: Grid = [
        [Some(5), Some(5),    None,    None, Some(7),    None,    None,    None, None],
        [Some(6),    None,    None, Some(1), Some(9), Some(5),    None,    None, None],
        [   None, Some(9), Some(8),    None,    None,    None,    None, Some(6), None],
        [Some(8),    None,    None,    None, Some(6),    None,    None,    None, Some(3)],
        [Some(4),    None,    None, Some(8),    None, Some(3),    None,    None, Some(1)],
        [Some(7),    None,    None,    None, Some(2),    None,    None,    None, Some(6)],
        [   None, Some(6),    None,    None,    None,    None, Some(2), Some(8), None],
        [   None,    None,    None, Some(4), Some(1), Some(9),    None,    None, Some(5)],
        [   None,    None,    None,    None, Some(8),    None,    None, Some(7), Some(9)],
    ];
    
    // 解けない問題なのでfalseを返すはず
    assert!(!solve_sudoku(&mut grid));
}

#[test]
fn test_is_valid_solution() {
    // 正しい解
    let valid_grid: Grid = [
        [Some(5), Some(3), Some(4), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)],
        [Some(6), Some(7), Some(2), Some(1), Some(9), Some(5), Some(3), Some(4), Some(8)],
        [Some(1), Some(9), Some(8), Some(3), Some(4), Some(2), Some(5), Some(6), Some(7)],
        [Some(8), Some(5), Some(9), Some(7), Some(6), Some(1), Some(4), Some(2), Some(3)],
        [Some(4), Some(2), Some(6), Some(8), Some(5), Some(3), Some(7), Some(9), Some(1)],
        [Some(7), Some(1), Some(3), Some(9), Some(2), Some(4), Some(8), Some(5), Some(6)],
        [Some(9), Some(6), Some(1), Some(5), Some(3), Some(7), Some(2), Some(8), Some(4)],
        [Some(2), Some(8), Some(7), Some(4), Some(1), Some(9), Some(6), Some(3), Some(5)],
        [Some(3), Some(4), Some(5), Some(2), Some(8), Some(6), Some(1), Some(7), Some(9)],
    ];
    assert!(is_valid_solution(&valid_grid));

    // 不正な解（同じ行に重複がある）
    let invalid_grid: Grid = [
        [Some(5), Some(5), Some(4), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)],
        [Some(6), Some(7), Some(2), Some(1), Some(9), Some(5), Some(3), Some(4), Some(8)],
        [Some(1), Some(9), Some(8), Some(3), Some(4), Some(2), Some(5), Some(6), Some(7)],
        [Some(8), Some(5), Some(9), Some(7), Some(6), Some(1), Some(4), Some(2), Some(3)],
        [Some(4), Some(2), Some(6), Some(8), Some(5), Some(3), Some(7), Some(9), Some(1)],
        [Some(7), Some(1), Some(3), Some(9), Some(2), Some(4), Some(8), Some(5), Some(6)],
        [Some(9), Some(6), Some(1), Some(5), Some(3), Some(7), Some(2), Some(8), Some(4)],
        [Some(2), Some(8), Some(7), Some(4), Some(1), Some(9), Some(6), Some(3), Some(5)],
        [Some(3), Some(4), Some(5), Some(2), Some(8), Some(6), Some(1), Some(7), Some(9)],
    ];
    assert!(!is_valid_solution(&invalid_grid));

    // 空のセルがある
    let incomplete_grid: Grid = [
        [Some(5), Some(3),    None, Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)],
        [Some(6), Some(7), Some(2), Some(1), Some(9), Some(5), Some(3), Some(4), Some(8)],
        [Some(1), Some(9), Some(8), Some(3), Some(4), Some(2), Some(5), Some(6), Some(7)],
        [Some(8), Some(5), Some(9), Some(7), Some(6), Some(1), Some(4), Some(2), Some(3)],
        [Some(4), Some(2), Some(6), Some(8), Some(5), Some(3), Some(7), Some(9), Some(1)],
        [Some(7), Some(1), Some(3), Some(9), Some(2), Some(4), Some(8), Some(5), Some(6)],
        [Some(9), Some(6), Some(1), Some(5), Some(3), Some(7), Some(2), Some(8), Some(4)],
        [Some(2), Some(8), Some(7), Some(4), Some(1), Some(9), Some(6), Some(3), Some(5)],
        [Some(3), Some(4), Some(5), Some(2), Some(8), Some(6), Some(1), Some(7), Some(9)],
    ];
    assert!(!is_valid_solution(&incomplete_grid));
}

#[test]
fn test_empty_grid() {
    let mut grid: Grid = [[None; 9]; 9];
    // 空のグリッドは解けるはず（ただし複数の解が存在する可能性がある）
    assert!(solve_sudoku(&mut grid));
    assert!(is_valid_solution(&grid));
}

#[test]
fn test_format_grid_empty() {
    let grid: Grid = [[None; 9]; 9];
    let output = format_grid(&grid);
    let expected = "\
+-------+-------+-------+
|       |       |       |
|       |       |       |
|       |       |       |
+-------+-------+-------+
|       |       |       |
|       |       |       |
|       |       |       |
+-------+-------+-------+
|       |       |       |
|       |       |       |
|       |       |       |
+-------+-------+-------+
";
    assert_eq!(output, expected);
}

#[test]
fn test_format_grid_partial() {
    let grid: Grid = [
        [Some(5), Some(3),    None,    None, Some(7),    None,    None,    None, None],
        [Some(6),    None,    None, Some(1), Some(9), Some(5),    None,    None, None],
        [   None, Some(9), Some(8),    None,    None,    None,    None, Some(6), None],
        [Some(8),    None,    None,    None, Some(6),    None,    None,    None, Some(3)],
        [Some(4),    None,    None, Some(8),    None, Some(3),    None,    None, Some(1)],
        [Some(7),    None,    None,    None, Some(2),    None,    None,    None, Some(6)],
        [   None, Some(6),    None,    None,    None,    None, Some(2), Some(8), None],
        [   None,    None,    None, Some(4), Some(1), Some(9),    None,    None, Some(5)],
        [   None,    None,    None,    None, Some(8),    None,    None, Some(7), Some(9)],
    ];
    let output = format_grid(&grid);
    let expected = "\
+-------+-------+-------+
| 5 3   |   7   |       |
| 6     | 1 9 5 |       |
|   9 8 |       |   6   |
+-------+-------+-------+
| 8     |   6   |     3 |
| 4     | 8   3 |     1 |
| 7     |   2   |     6 |
+-------+-------+-------+
|   6   |       | 2 8   |
|       | 4 1 9 |     5 |
|       |   8   |   7 9 |
+-------+-------+-------+
";
    assert_eq!(output, expected);
}

#[test]
fn test_format_grid_complete() {
    let grid: Grid = [
        [Some(5), Some(3), Some(4), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)],
        [Some(6), Some(7), Some(2), Some(1), Some(9), Some(5), Some(3), Some(4), Some(8)],
        [Some(1), Some(9), Some(8), Some(3), Some(4), Some(2), Some(5), Some(6), Some(7)],
        [Some(8), Some(5), Some(9), Some(7), Some(6), Some(1), Some(4), Some(2), Some(3)],
        [Some(4), Some(2), Some(6), Some(8), Some(5), Some(3), Some(7), Some(9), Some(1)],
        [Some(7), Some(1), Some(3), Some(9), Some(2), Some(4), Some(8), Some(5), Some(6)],
        [Some(9), Some(6), Some(1), Some(5), Some(3), Some(7), Some(2), Some(8), Some(4)],
        [Some(2), Some(8), Some(7), Some(4), Some(1), Some(9), Some(6), Some(3), Some(5)],
        [Some(3), Some(4), Some(5), Some(2), Some(8), Some(6), Some(1), Some(7), Some(9)],
    ];
    let output = format_grid(&grid);
    let expected = "\
+-------+-------+-------+
| 5 3 4 | 6 7 8 | 9 1 2 |
| 6 7 2 | 1 9 5 | 3 4 8 |
| 1 9 8 | 3 4 2 | 5 6 7 |
+-------+-------+-------+
| 8 5 9 | 7 6 1 | 4 2 3 |
| 4 2 6 | 8 5 3 | 7 9 1 |
| 7 1 3 | 9 2 4 | 8 5 6 |
+-------+-------+-------+
| 9 6 1 | 5 3 7 | 2 8 4 |
| 2 8 7 | 4 1 9 | 6 3 5 |
| 3 4 5 | 2 8 6 | 1 7 9 |
+-------+-------+-------+
";
    assert_eq!(output, expected);
}
