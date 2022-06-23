const BLACK_CHR: &str = "黒";
const WHITE_CHR: &str = "白";
const CLEAR_CHR: &str = "　";

const TABLE_SIZE: usize = 8;

const CLEAR: u8 = 0;
const BLACK: u8 = 1;
const WHITE: u8 = 2;


fn print_table(table: &[[u8; TABLE_SIZE]; TABLE_SIZE]){
    for line in table {
        for stone in line{
            let chr = match *stone {
                CLEAR => CLEAR_CHR,
                BLACK => BLACK_CHR,
                WHITE => WHITE_CHR,
                _ => "",
            };
            print!("{}", chr)
        }
        println!()
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
