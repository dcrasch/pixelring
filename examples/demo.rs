use pixelring::PixelRing;

fn main() {
    let pixelring = PixelRing::new().expect("Failed to open pixelring");
    pixelring.think().expect("no think");
}
