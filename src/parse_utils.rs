use crate::array2d::Array2D;

pub fn char_grid(input: &str) -> Array2D<char> {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.len() / height;
    let buf: Vec<char> = input.lines().flat_map(str::chars).collect();
    Array2D::new(width, height, buf)
}
