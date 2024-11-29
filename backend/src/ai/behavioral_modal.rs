use tch::{nn, nn::Module, Device, Tensor};
use tokio;

pub async fn run_analysis() -> String {
    let vs = nn::VarStore::new(Device::Cpu);
    let model = nn::seq()
        .add(nn::linear(&vs.root(), 3, 16, Default::default()))
        .add_fn(|x| x.relu())
        .add(nn::linear(&vs.root(), 16, 8, Default::default()))
        .add_fn(|x| x.relu())
        .add(nn::linear(&vs.root(), 8, 1, Default::default()));

    let input = Tensor::of_slice(&[1.0, 2.0, 3.0]).reshape(&[1, 3]);
    let output = model.forward(&input);
    let output_value = output.double_value(&[0]);

    format!("{{\"Behavioral analysis result\": {:.3}}}", output_value)
}

#[tokio::main]
async fn main() {
    let result = run_analysis().await;
    println!("{}", result);
}
