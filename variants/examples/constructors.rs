/*
    appellation: constructors <example>
    authors: @FL03
*/
use variants::VariantConstructors;

fn main() -> variants::Result<()> {
    let a = Something::a();
    let b = Something::b(42);
    let c = Something::c(1.0, 2.0);

    println!("Variants:\n\t{a:?}\n\t{b:?}\n\t{c:?}");

    Ok(())
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, VariantConstructors)]
pub enum Something {
    #[default]
    A,
    B(usize),
    C {
        x: f64,
        y: f64,
    },
    D(usize, String, f64),
}
