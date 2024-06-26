mod scalar;
use scalar::Scalar;

fn main() {
    let a = Scalar::from(4.0, None, None);
    let b = Scalar::from(6.0, None, None);
    let c = a + b;

    println!("{}", c._operation.unwrap());
}
