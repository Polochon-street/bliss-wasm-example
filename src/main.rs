use bliss_audio::Song;

fn main() {
    println!("Analysis some random array...");
    let analysis = Song::analyze(&[0.3; 10000]).unwrap();
    println!("Analysis: {:?}", analysis);
}
