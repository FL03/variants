/*
    Appellation: derive <test>
    Contrib: @FL03
*/
use variants_derive::VariantConstructors;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, VariantConstructors)]
pub enum TestEnum {
    #[default]
    A,
    B(usize),
    C {
        x: f64,
        y: f64,
    },
    D(usize, String, f64),
}

#[test]
fn test_variant_constructors() {
    assert_eq!(TestEnum::a(), TestEnum::A);
    assert_eq!(TestEnum::b(1), TestEnum::B(1));
    assert_eq!(TestEnum::c(1.0, 2.0), TestEnum::C { x: 1.0, y: 2.0 });
    assert_eq!(
        TestEnum::d(1, "test".to_string(), 3.0),
        TestEnum::D(1, "test".to_string(), 3.0)
    );
}
