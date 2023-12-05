use std::{fmt::Debug, ops::AddAssign};

#[derive(Clone)]
pub struct Array2D<T: Debug + Clone> {
    buf: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Debug + Copy> Debug for Array2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (w, h) = (self.width, self.height);
        writeln!(f, "Array2D width={w}, height={h} :")?;
        for y in 0..h {
            let linej = y * w;
            writeln!(f, "{:?}", self.buf[linej..(linej + w)].to_vec())?;
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

impl<T: Debug + Copy> Array2D<T> {
    pub fn new(width: usize, height: usize, buf: Vec<T>) -> Self {
        assert_eq!(
            width * height,
            buf.len(),
            "Width * Height supposed to eq buf len"
        );
        Array2D { buf, width, height }
    }

    /// X is the 1 strided axis
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> T {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        self.buf[y * self.width + x]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        self.buf[y * self.width + x] = val;
    }

    pub fn neighbors8(&self, x: usize, y: usize) -> Vec<T> {
        let min_x = x.saturating_sub(1);
        let min_y = y.saturating_sub(1);
        let max_x = self.width.min(x + 1);
        let max_y = self.height.min(y + 1);
        (min_x..=max_x)
            .flat_map(move |xi| {
                (min_y..=max_y).filter_map(move |yi| (xi != x || yi != y).then(|| self.get(xi, yi)))
            })
            .collect()
    }

    pub fn neighbors8_indexed(&self, x: usize, y: usize) -> Vec<(usize, usize, T)> {
        let min_x = x.saturating_sub(1);
        let min_y = y.saturating_sub(1);
        let max_x = self.width.min(x + 1);
        let max_y = self.height.min(y + 1);
        (min_x..=max_x)
            .flat_map(move |xi| {
                (min_y..=max_y)
                    .filter_map(move |yi| (xi != x || yi != y).then(|| (xi, yi, self.get(xi, yi))))
            })
            .collect()
    }
}

impl<T: Debug + Clone + AddAssign> Array2D<T> {
    #[inline]
    pub fn increment(&mut self, x: usize, y: usize, val: T) {
        self.buf[y * self.width + x] += val;
    }
}
