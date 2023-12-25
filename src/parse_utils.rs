use crate::array2d::Array2D;

pub fn char_grid(input: &str) -> Array2D<char> {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.len() / height;
    let buf: Vec<char> = input.lines().flat_map(str::chars).collect();
    Array2D::new(width, height, buf)
}

pub fn digits_grid(input: &str) -> Array2D<u8> {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.len() / height;
    let buf: Vec<u8> = input
        .lines()
        .flat_map(str::chars)
        .map(|x| x as u8 - ('0' as u8))
        .collect();
    Array2D::new(width, height, buf)
}
