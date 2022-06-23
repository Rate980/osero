const BLACK_CHR: &str = "黒";
const WHITE_CHR: &str = "白";
const CLEAR_CHR: &str = "　";
const PUTABLE_CHR: &str = "置";

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

fn main() {
    let mut table = [[CLEAR; TABLE_SIZE]; TABLE_SIZE];
    // 初期位置にコマを置く
    table[3][3] = WHITE;
    table[3][4] = BLACK;
    table[4][3] = BLACK;
    table[4][4] = WHITE;
    print_table(&table);
}
