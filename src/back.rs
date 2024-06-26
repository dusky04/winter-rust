use crate::scalar::Scalar;

fn backprop_add_op(x: &mut Scalar, y: &mut Scalar) {
    x.grad += 1.0 * out.grad;
}
