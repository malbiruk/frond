use frond_core::Dialogue;

fn main() {
    println!("Hello from `frond`!");
    let dialogue = Dialogue::new("My Dialogue");
    println!("Dialogue name: {}", dialogue.name());
}
