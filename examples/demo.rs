use pixelring::{PixelRing, Volume};
use std::{thread, time};

fn main() {
    let pixelring = PixelRing::new().expect("Failed to open pixelring");
    for v in 0..12 {
        println!("{v}");
        pixelring.set_volume(v).expect("no volume");
        thread::sleep(time::Duration::from_secs(1));
    }
}
