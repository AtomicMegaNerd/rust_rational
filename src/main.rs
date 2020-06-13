use rust_rational::Rational;

pub fn main() {
    let r1 = Rational::new(1, 2);
    let r2 = Rational::new(1, 2);
    let r3 = Rational::new(2, 1);

    println!("Let's have some fun with fractions!");

    let r4 = r1 * r2;
    println!("{} * {} == {}", r1, r2, r4);

    let r5 = r1 / r2;
    println!("{} / {} == {}", r1, r2, r5);

    let r7 = r3 - r1;
    println!("{} - {} == {}", r3, r1, r7);

    let r8 = r1 + r3;
    println!("{} + {} == {}", r1, r3, r8);

    let r9 = Rational::new(426, 1881);
    let r10 = Rational::new(117, 444);
    let r11 = r9 + r10;
    println!("{} + {} == {}", r9, r10, r11);

    let r12 = r1.reciprocal();
    println!("The reciprocal of {} is {}", r1, r12);
}
