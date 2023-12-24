use std::{fmt::Debug, ops::AddAssign};

use crate::vec2d::Vec2D;

#[derive(Clone)]
pub struct Array2D<T: Debug + Clone> {
    buf: Vec<T>,
    width: usize,
    height: usize,
}

impl Debug for Array2D<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (w, h) = (self.width, self.height);
        writeln!(f, "Array2D width={w}, height={h} :")?;
        for y in 0..h {
            for x in 0..w {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for Array2D<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (w, h) = (self.width, self.height);
        writeln!(f, "Array2D width={w}, height={h} :")?;
        for y in 0..h {
            let idx = y * w;
            writeln!(f, "{:?}", &self.buf[idx..(idx + w)])?;
        }
        Ok(())
    }
}

impl<T: Debug + Copy + Default> Array2D<T> {
    pub fn new_default(width: usize, height: usize) -> Self {
        Array2D {
            buf: vec![T::default(); width * height],
            width,
            height,
        }
    }
}

impl<T: Debug + Copy + Eq> Array2D<T> {
    pub fn find_index(&self, element: T) -> Option<(usize, usize)> {
        let idx = self.buf.iter().position(|&x| x == element)?;
        Some((idx % self.width, idx / self.width))
    }

    pub fn count<F: Fn(&&T) -> bool>(&self, predicate: F) -> usize {
        self.buf.iter().filter(predicate).count()
    }
}

impl<T: Debug + Copy + AddAssign> Array2D<T> {
    #[inline]
    pub fn increment(&mut self, x: usize, y: usize, val: T) {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        self.buf[y * self.width + x] += val;
    }
    #[inline]
    pub fn incrementv(&mut self, coord: Vec2D, val: T) {
        self.increment(coord.x as usize, coord.y as usize, val)
    }
}
impl<T: Debug + Copy> Array2D<T> {
    pub fn new(width: usize, height: usize, buf: Vec<T>) -> Self {
        assert_eq!(
            width * height,
            buf.len(),
            "Width * Height supposed to eq buf len"
        );
        Array2D { buf, width, height }
    }
    pub fn new_filled(width: usize, height: usize, fill: T) -> Self {
        Array2D {
            buf: vec![fill; width * height],
            width,
            height,
        }
    }

    pub fn is_valid_idxv(&self, coord: Vec2D) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && (coord.x as usize) < self.width
            && (coord.y as usize) < self.height
    }

    pub fn dimensions(&self) -> Vec2D {
        Vec2D {
            x: self.width as i64,
            y: self.height as i64,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
    /// X is the 1 strided axis
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> T {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        self.buf[y * self.width + x]
    }

    #[inline]
    pub fn getv(&self, coord: Vec2D) -> T {
        let Vec2D { x, y } = coord;
        self.get(x as usize, y as usize)
    }
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        self.buf[y * self.width + x] = val;
    }

    #[inline]
    pub fn setv(&mut self, coord: Vec2D, val: T) {
        let Vec2D { x, y } = coord;
        self.set(x as usize, y as usize, val)
    }

    pub fn neighbors8_indexed(&self, x: usize, y: usize) -> Vec<(usize, usize, T)> {
        let min_x = x.saturating_sub(1);
        let min_y = y.saturating_sub(1);
        let max_x = (self.width - 1).min(x + 1);
        let max_y = (self.height - 1).min(y + 1);
        (min_x..=max_x)
            .flat_map(move |xi| {
                (min_y..=max_y)
                    .filter(move |&yi| (xi != x || yi != y))
                    .map(move |yi| (xi, yi, self.get(xi, yi)))
            })
            .collect()
    }

    pub fn neighbors4_indices(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let min_x = x.saturating_sub(1);
        let min_y = y.saturating_sub(1);
        let max_x = (self.width - 1).min(x + 1);
        let max_y = (self.height - 1).min(y + 1);
        (min_x..=max_x).flat_map(move |xi| {
            (min_y..=max_y)
                .filter(move |&yi| ((xi == x) ^ (yi == y)))
                .map(move |yi| (xi, yi))
        })
    }
}
