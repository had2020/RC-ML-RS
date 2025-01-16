use rcmlrs::*;

/*
fn forward(tensor: Tensor) -> Tensor {
}
*/

fn main() {
    let mut memory = Memory::new("test");

    let weights: Tensor = Tensor::new_layer_zeros(&mut memory, Shape { x: 1, y: 1 }, 2);
    let bias: Tensor = Tensor::new_layer_zeros(&mut memory, Shape { x: 6, y: 2 }, 1);

    print_tensor(&memory, weights);
}
