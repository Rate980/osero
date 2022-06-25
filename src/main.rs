const BLACK_CHR: &str = "黒";
const WHITE_CHR: &str = "白";
const CLEAR_CHR: &str = "　";
const PUTABLE_CHR: &str = "・";

const TABLE_SIZE: usize = 8;

const CLEAR: u8 = 0;
const BLACK: u8 = 1;
const WHITE: u8 = 2;
const PUTABLE: u8 = 7;

const TOP: &str = "  　0　1　2　3　4　5　6　7\n  ┌──┬──┬──┬──┬──┬──┬──┬──┐";
const MID: &str = "  ├──┼──┼──┼──┼──┼──┼──┼──┤";
const BOTTOM: &str = "  └──┴──┴──┴──┴──┴──┴──┴──┘";

macro_rules! range {
    ($stop:expr) => {
        0..$stop
    };
    ($start:expr, $stop:expr) => {
        $start..$stop
    };
    ($start:expr, $stop:expr, -$step:expr) => {
        ($stop + 1..$start + 1).rev().step_by($step)
    };
    ($start:expr, $stop:expr, $step:expr) => {
        ($start..$stop).step_by($step)
    };
}

fn show_table(table: &[[u8; TABLE_SIZE]; TABLE_SIZE]) {
    print!("{}", TOP);
    for (line, i) in table.iter().zip(0..8) {
        print!("\n{}0│", i);
        for stone in line {
            let chr = match *stone {
                BLACK => BLACK_CHR,
                WHITE => WHITE_CHR,
                PUTABLE => PUTABLE_CHR,
                _ => CLEAR_CHR,
            };
            print!("{}│", chr);
        }
        print!("\n{}", MID);
    }
    println!("\r{}", BOTTOM)
}

fn index_to_cmporce(x: usize, y: usize) -> usize {
    (y << 3) + x
}

fn comporce_to_index(compors: usize) -> (usize, usize) {
    // println!("{}", compors);
    // println!("{}", compors >> 3);
    // println!("{}", compors & 0o7);
    (compors >> 3, compors & 0o7)
}

fn reverce(coler: u8) -> u8 {
    match coler {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => CLEAR,
    }
}
fn get_next_tile<'a>(
    table: &'a [[u8; TABLE_SIZE]; TABLE_SIZE],
    diff: &[i32; 2],
    y: usize,
    x: usize,
) -> Option<(&'a u8, usize, usize)> {
    let diff_y = diff[0];
    let diff_x = diff[1];
    let x1 = diff_x + x as i32;
    let y1 = diff_y + y as i32;
    if x1 < 0 || y1 < 0 {
        return None;
    }
    let y1 = y1 as usize;
    let x1 = x1 as usize;

    let line = match table.get(y1) {
        Some(v) => v,
        None => return None,
    };
    let tile = match line.get(x1) {
        Some(v) => v,
        None => return None,
    };
    Some((tile, y1, x1))
}

fn check_putable(
    table: &[[u8; TABLE_SIZE]; TABLE_SIZE],
    coler: u8,
    diff: &[i32; 2],
    y: usize,
    x: usize,
    is_continuous: bool,
) -> bool {
    let res = match get_next_tile(table, diff, y, x) {
        Some(v) => v,
        None => return false,
    };
    let (tile, y1, x1) = res;
    if *tile == coler {
        is_continuous
    } else if *tile == reverce(coler) {
        check_putable(table, coler, diff, y1 as usize, x1 as usize, true)
    } else {
        false
    }
}

fn get_putables(table: &mut [[u8; TABLE_SIZE]; TABLE_SIZE], coler: u8) -> Vec<usize> {
    let mut putables: Vec<usize> = Vec::new();
    for y in range!(TABLE_SIZE) {
        for x in range!(TABLE_SIZE) {
            if table[y][x] == BLACK || table[y][x] == WHITE {
                continue;
            }
            'l1: for i in 0..3 {
                let i1 = i - 1;
                for j in 0..3 {
                    let j1 = j - 1;
                    if i + j == 0 {
                        continue;
                    }
                    if check_putable(table, coler, &[i1, j1], y, x, false) {
                        table[y][x] = 7;
                        putables.push(index_to_cmporce(x, y));
                        break 'l1;
                    }
                }
            }
        }
    }
    putables
}
fn check_reverce(
    table: &mut [[u8; TABLE_SIZE]; TABLE_SIZE],
    coler: u8,
    diff: &[i32; 2],
    y: usize,
    x: usize,
    is_continuous: bool,
) -> bool {
    // println!("{:o}", index_to_cmporce(x, y));
    let res = match get_next_tile(table, diff, y, x) {
        Some(v) => v,
        None => return false,
    };
    let (tile, y1, x1) = res;
    if *tile == coler {
        is_continuous
    } else if *tile == reverce(coler) {
        let res = check_reverce(table, coler, diff, y1, x1, true);
        if res {
            table[y1][x1] = coler;
        }
        res
    } else {
        false
    }
}

fn reversing(table: &mut [[u8; TABLE_SIZE]; TABLE_SIZE], put_point: usize, coler: u8) {
    remove_putable(table);
    let (y, x) = comporce_to_index(put_point);
    table[y][x] = coler;
    for i in 0..3 {
        let diff_y = i - 1;
        for j in 0..3 {
            let diff_x = j - 1;

            check_reverce(table, coler, &[diff_y, diff_x], y, x, false);
        }
    }
}

fn remove_putable(table: &mut [[u8; 8]; 8]) {
    for y in range!(TABLE_SIZE) {
        for x in range!(TABLE_SIZE) {
            table[y][x] = table[y][x] % 7;
        }
    }
}
fn main() {
    let mut table = [[CLEAR; TABLE_SIZE]; TABLE_SIZE];
    // 初期位置にコマを置く
    // table[3][3] = WHITE;
    // table[3][4] = BLACK;
    // table[4][3] = BLACK;
    // table[4][4] = WHITE;

    table[0][3] = BLACK;
    for i in 1..7 {
        table[i][3] = WHITE;
    }
    let puttables = get_putables(&mut table, BLACK);
    show_table(&table);
    let put_point = puttables[0];
    reversing(&mut table, put_point, BLACK);
    show_table(&table);
    // show_table(&table);
}
