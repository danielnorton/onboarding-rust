pub fn sorrounded_regions(board: &mut Vec<Vec<char>>) {
    if board.is_empty() {
        return;
    }

    // looking 'O' in border by rows
    (0..board[0].len()).for_each(|i| {
        if board[0][i] == 'O' {
            search_recursive(board, 0, i as i32, '0');
        }
        if board[board.len() - 1][i] == 'O' {
            search_recursive(board, (board.len() - 1) as i32, i as i32, '0' );
        }
    });

    // looking 'O' in border by col
    (0..board.len()).for_each(|j| {
        if board[j][0] == 'O' {
            search_recursive(board, j as i32, 0, '0');
        }
        if board[j][board[0].len() - 1] == 'O' {
            search_recursive(board, j as i32, (board[0].len() - 1) as i32, '0');
        }
    });

    //replacing all 'O' no-in-border with 'X' and all the '0' by 'O'
    board.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|ch| {
            if *ch == 'O' {
                *ch = 'X'
            } else if *ch == '0' {
                *ch = 'O'
            }
        })
    });
}

fn search_recursive(regions: &mut Vec<Vec<char>>, i: i32, j: i32, ch: char) {

    regions[i as usize][j as usize] = ch;
    let cord = vec![(-1, 0), (0, 1), (0, -1), (1, 0)];

    for (x, y) in cord.iter(){
        if i + y >= 0
            && j + x< regions[0].len() as i32
            && i + y < regions.len() as i32
            && j + x >= 0
            && regions[(i + y) as usize][(j + x) as usize] == 'O'
        {
            search_recursive(regions, i + y, j + x, ch);
        }
    }
}
