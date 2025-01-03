// train_model.rs
use tch::{nn, Tensor};

fn main() {
    let vs = nn::VarStore::new(tch::Device::Cpu);
    let model = nn::seq().add(nn::linear(&vs.root(), 10, 1, Default::default()));

    let input = Tensor::of_slice(&[1.0, 2.0, 3.0]).reshape(&[1, 3]);
    let output = model.forward(&input);
    println!("Training model output: {:?}", output);
}
