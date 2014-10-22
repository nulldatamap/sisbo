extern crate ears;

use ears::{AudioController, Sound};

fn main() {
  let mut snd = Sound::new( "res/Example.ogg" ).unwrap();
  snd.play();
  while snd.is_playing() {}
  println!("Hello, world!")
}
