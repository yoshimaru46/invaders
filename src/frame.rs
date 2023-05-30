use crate::NUM_COLS;

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_COLS);
        for _ in 0..NUM_COLS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
