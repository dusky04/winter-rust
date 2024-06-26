mod back;
mod scalar;
mod topo_sort;

use std::collections::HashSet;

use scalar::Scalar;

fn main() {
    let mut a = Scalar::from(4.0, None, None);
    let mut b = Scalar::from(6.0, None, None);
    let mut c = a + b;

    let mut d = Scalar::from(4.0, None, None);
    let mut e = c.clone() * d.clone();

    // println!("{}", c._operation.unwrap());

    c.grad = 1.0;
    // println!("Gradient of c: {}", c.grad);
    // println!("Parent of c: {:?}", c._children.unwrap());

    // c.backward()

    let mut sorted_nodes: Vec<Scalar> = Vec::new();
    let mut visited: HashSet<Scalar> = HashSet::new();

    topo_sort::topological_sort(&e, &mut visited, &mut sorted_nodes);

    println!("{:?}", sorted_nodes);
}
