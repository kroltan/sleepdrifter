use super::prelude::*;

fn pow2(n: f32) -> f32 { n.powf(2.0) }

#[test]
fn complex_expression() {
    let (x, mut xs) = Parameter::<f32>::empty();
    let (y, mut ys) = Parameter::<f32>::empty();
    let magnitude = (x.map(pow2) + y.map(pow2)).map(f32::sqrt);
    let magnitude2 = magnitude.clone();

    xs.set(5.0);
    ys.set(12.0);
    assert_eq!(magnitude.evaluate(), 13.0);

    xs.set(5.0);
    ys.set(3.0);
    assert_eq!(magnitude2.evaluate(), 5.8309518948453004708741528775456);
}