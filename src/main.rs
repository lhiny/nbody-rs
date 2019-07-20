mod surface;
use surface::Surface;

fn main() {
    let mut s = Surface::new(800, 600);
    s.run();
}
