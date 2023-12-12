use std::io;

const WIDTH: usize = 50;
const HEIGHT: usize = 10;
const QUIT_COMMANDS: [&str; 4] = ["q", "quit", "e", "exit"];

fn move_cursor_to_top_left() {
    print!("\x1B[H");
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_board(board: [[bool; HEIGHT]; WIDTH]) {
    move_cursor_to_top_left();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if board[x][y] {
                print!("#");
            } else {
                print!(".")
            }

            if x == WIDTH - 1 {
                println!();
            }
        }
    }
}

fn step_board(board: &mut[[bool; HEIGHT]; WIDTH]) {
    let old_board = board.clone();

    for (x, col) in board.iter_mut().enumerate() {
        for (y, cell) in col.iter_mut().enumerate() {
            let neighbours = (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (x as i32 + dx, y as i32 + dy)))
                .map(|(nx, ny)| (nx.rem_euclid(WIDTH as i32), ny.rem_euclid(HEIGHT as i32)))
                .filter(|&(nx, ny)| !(nx == (x as i32) && ny == (y as i32)))
                .filter(|&(nx, ny)| old_board[nx as usize][ny as usize])
                .count();

            if *cell && (neighbours < 2 || neighbours > 3) {
                *cell = false;
            } else if !*cell && neighbours == 3 {
                *cell = true;
            }
        }
    }
}

fn create_glider(board: &mut[[bool; HEIGHT]; WIDTH]) {
    let center_x = WIDTH / 2;
    let center_y = HEIGHT / 2;

    board[center_x][center_y] = true;
    board[center_x][center_y - 1] = true;
    board[center_x][center_y - 2] = true;

    board[center_x - 1][center_y] = true;
    board[center_x - 2][center_y - 1] = true;
}

fn main() {
    clear_terminal();

    let mut board = [[false; HEIGHT]; WIDTH];

    create_glider(&mut board);

    loop {
        print_board(board);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to get user input");

        if QUIT_COMMANDS.iter().any(|&command| command == user_input.trim()) {
            break;
        }

        step_board(&mut board);
    }
}
