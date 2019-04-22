
use gnuplot::{Figure, Color};
use sounds::generators::*;


fn main() {
    let mut gen = square(100, 2.0);
    let xs: Vec<f32> = (0..100).map(|_| gen.next_sample()).collect();
    let idx: Vec<usize> = (0..xs.len()).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &xs, &[Color("blue")]);
    fg.show();
}