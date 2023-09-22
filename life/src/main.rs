use rand::Rng;
use std::thread;


macro_rules! dimx {
    () => {113}
}


macro_rules! dimy {
    () => {30}
}


fn print_board(board : [[bool; dimx!()]; dimy!()]) {
    for row in board {
        for c in row {
            let k = if c {'x'} else {' '};
            print!("{}", k);
        }
        println!();
    }
}


fn count_neighbors(board : [[bool; dimx!()]; dimy!()], x : usize, y : usize) -> u32 {

    let arry : [usize; 3] = [(y + dimy!() - 1) % dimy!(), y, (y + 1) % dimy!()];
    let arrx : [usize; 3] = [(x + dimx!() - 1) % dimx!(), x, (x + 1) % dimx!()];
    
    let mut count = if board[y][x] {0} else {1};
    for k in arry {
        for j in arrx {
            if board[k][j] {
                count = count + 1;
            }
        }
    }
    
    count = count - 1;

    return count;
}


fn make_glider() -> (Vec<usize>, usize, usize)
{
    let pat = vec![
        1, 0, 0,
        0, 1, 1,
        1, 1, 0
    ];

    return (pat, 3, 3);
}

fn place_in_board(board : &mut [[bool; dimx!()]; dimy!()], x : usize, y : usize, pattern : (Vec<usize>, usize, usize))
{
    let (pat, h, w) = pattern;
    for py in 0..h {
        for px in 0..w {
            board[py+x][px+y] = pat[py*w+px] == 1;
        }
    }
}


fn main() {
    //let mut rng = rand::thread_rng();
    
    let mut board = [[false; dimx!()]; dimy!()];
    // for y in 0..dimy!() {
    //     for x in 0..dimx!() {
    //         board[y][x] = rng.gen_range(0..10) < 5;
    //     }
    // }

    place_in_board(&mut board, 1, 1, make_glider());
    
    loop {
        let cpy = board.clone();
        print_board(board);
        for y in 0..dimy!() {
            for x in 0..dimx!() {
                let nn = count_neighbors(cpy, x, y);
                board[y][x] = (board[y][x] && nn >= 2 && nn <=3) || (!board[y][x] && nn == 3);
            }
        }

        thread::sleep(std::time::Duration::from_millis(300));
        println!();
        println!();
    }
    
}
