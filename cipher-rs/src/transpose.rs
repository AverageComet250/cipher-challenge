use itertools::Itertools;

pub fn decipher(ciphertext: &str) -> Option<String> {
    let cleaned: String = ciphertext.chars().filter(|c| c.is_alphabetic()).collect();
    let len = cleaned.chars().count();
    let max_factor = 26usize.max(len + 1);
    for factor in (1..max_factor).filter(|v| len % v == 0) {
        let rows = len / factor;
        let row_grid = build_row_grid(factor, rows, ciphertext);
        let mut row_text: Vec<char> = Vec::with_capacity(len);
        for permutation in row_grid.iter().permutations(factor) {
            for (i, column) in row_grid.iter().enumerate() {
                for (j, c) in column {
                    row_text.insert()
                }
            }
        }
        let column_grid = build_column_grid(factor, rows, ciphertext);
    }
    None
}

fn build_row_grid(columns: usize, rows: usize, ciphertext: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![Vec::with_capacity(rows); columns];
    for (i, char) in ciphertext.chars().enumerate() {
        grid[i % columns][i / columns] = char;
    }

    grid
}

fn build_column_grid(columns: usize, rows: usize, ciphertext: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![Vec::with_capacity(rows); columns];
    for (i, char) in ciphertext.chars().enumerate() {
        grid[i / rows][i % rows] = char;
    }

    grid
}

fn crib_key_length(ciphertext: &str) -> Vec<usize> {
    let cribs = ["the"];
    for crib in cribs {
        let mut char_distances: usize = vec![];
        for crib_char in crib {
            for char in ciphertext.chars() {}
        }
    }
    vec![]
}
