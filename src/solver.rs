// 数独ソルバー
// 9x9のグリッドを表現する型（Noneは未入力を表す）
pub type Grid = [[Option<u8>; 9]; 9];

// 数独の盤面を文字列としてフォーマットする（テスト可能）
pub fn format_grid(grid: &Grid) -> String {
  let mut result = String::new();
  result.push_str("+-------+-------+-------+\n");
  for (i, row) in grid.iter().enumerate() {
    if i > 0 && i % 3 == 0 {
      result.push_str("+-------+-------+-------+\n");
    }
    result.push_str("| ");
    for (j, cell) in row.iter().enumerate() {
      if j > 0 && j % 3 == 0 {
        result.push_str("| ");
      }
      match cell {
        None => result.push_str("  "),
        Some(value) => result.push_str(&format!("{} ", value)),
      }
    }
    result.push_str("|\n");
  }
  result.push_str("+-------+-------+-------+\n");
  result
}

// 指定された位置に値を配置できるかチェック
// 注意: この関数は指定された位置が空（None）であることを前提としている
pub fn is_valid(grid: &Grid, row: usize, col: usize, num: u8) -> bool {
  // 行のチェック
  for cell in &grid[row] {
    if matches!(cell, Some(v) if *v == num) {
      return false;
    }
  }

  // 列のチェック
  for i in 0..9 {
    if matches!(grid[i][col], Some(v) if v == num) {
      return false;
    }
  }

  // 3x3ボックスのチェック
  let box_row = (row / 3) * 3;
  let box_col = (col / 3) * 3;
  for i in box_row..box_row + 3 {
    for j in box_col..box_col + 3 {
      if matches!(grid[i][j], Some(v) if v == num) {
        return false;
      }
    }
  }

  true
}

// バックトラッキングアルゴリズムで数独を解く
pub fn solve_sudoku(grid: &mut Grid) -> bool {
  // 空のセルを探す
  let empty_cell = (0..9)
    .flat_map(|i| (0..9).map(move |j| (i, j)))
    .find(|&(i, j)| grid[i][j].is_none());

  // 空のセルがない場合、解が見つかった
  let (row, col) = match empty_cell {
    None => return true,
    Some(pos) => pos,
  };

  // 1から9までの数字を試す
  for num in 1..=9 {
    if is_valid(grid, row, col, num) {
      grid[row][col] = Some(num);

      // 再帰的に次のセルを解く
      if solve_sudoku(grid) {
        return true;
      }

      // 解が見つからなかった場合、元に戻す
      grid[row][col] = None;
    }
  }

  false
}

// 数独の解が正しいか検証する
pub fn is_valid_solution(grid: &Grid) -> bool {
  // 各行に1-9が1回ずつ現れるかチェック
  for row in grid.iter() {
    let mut seen = [false; 10];
    for cell in row.iter() {
      let value = match cell {
        Some(v) if (1..=9).contains(v) => *v,
        _ => return false,
      };
      if seen[value as usize] {
        return false;
      }
      seen[value as usize] = true;
    }
  }

  // 各列に1-9が1回ずつ現れるかチェック
  for col in 0..9 {
    let mut seen = [false; 10];
    for row in 0..9 {
      let value = match grid[row][col] {
        Some(v) if (1..=9).contains(&v) => v,
        _ => return false,
      };
      if seen[value as usize] {
        return false;
      }
      seen[value as usize] = true;
    }
  }

  // 各3x3ボックスに1-9が1回ずつ現れるかチェック
  for box_row in 0..3 {
    for box_col in 0..3 {
      let mut seen = [false; 10];
      for i in 0..3 {
        for j in 0..3 {
          let value = match grid[box_row * 3 + i][box_col * 3 + j] {
            Some(v) if (1..=9).contains(&v) => v,
            _ => return false,
          };
          if seen[value as usize] {
            return false;
          }
          seen[value as usize] = true;
        }
      }
    }
  }

  true
}
