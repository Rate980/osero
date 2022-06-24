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

fn show_table(table: &[[u8; TABLE_SIZE]; TABLE_SIZE], is_show_puttable: bool) {
    print!("{}", TOP);
    for (line, i) in table.iter().zip(0..8) {
        print!("\n{}0│", i);
        for stone in line {
            let chr = match *stone {
                BLACK => BLACK_CHR,
                WHITE => WHITE_CHR,
                PUTABLE => {
                    if is_show_puttable {
                        PUTABLE_CHR
                    } else {
                        CLEAR_CHR
                    }
                }
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

fn comporce_to_index(compors: usize) -> [usize; 2] {
    [compors & 0b111000 >> 3, compors & 0b111]
}

fn reverce(coler: u8) -> u8 {
    match coler {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => CLEAR,
    }
}

fn check_putable(
    table: &[[u8; TABLE_SIZE]; TABLE_SIZE],
    coler: u8,
    diff: [i32; 2],
    y: usize,
    x: usize,
    is_continuous: bool,
) -> bool {
    let diff_y = diff[0];
    let diff_x = diff[1];
    let x1 = diff_x + x as i32;
    let y1 = diff_y + y as i32;
    if x1 < 0 || x1 as usize > TABLE_SIZE || y1 < 0 || y1 as usize > TABLE_SIZE {
        return false;
    }

    if table[y1 as usize][x1 as usize] == coler {
        is_continuous
    } else if table[y1 as usize][x1 as usize] == reverce(coler) {
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
                    if check_putable(table, coler, [i1, j1], y, x, false) {
                        putables.push(index_to_cmporce(x, y));
                        break 'l1;
                    }
                }
            }
        }
    }
    putables
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
    table[3][3] = WHITE;
    table[3][4] = BLACK;
    table[4][3] = BLACK;
    table[4][4] = WHITE;
    let x: usize = 4;
    let com = index_to_cmporce(x, x);
    let i = comporce_to_index(com);
    println!("{}", com);
    println!("{}{}", i[0], i[1]);
}
