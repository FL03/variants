/*
    appellation: constructors <example>
    authors: @FL03
*/
use variants::VariantConstructors;

fn main() {
    let a = Something::a();
    let b = Something::b(42);
    let c = Something::c(1.0, 2.0);
    
    println!("Variants:\n\t{a:?}\n\t{b:?}\n\t{c:?}");
}

#[derive(Clone, Copy, Debug, VariantConstructors)]
pub enum Something {
    A,
    B(usize),
    C { x: f64, y: f64 },
}
