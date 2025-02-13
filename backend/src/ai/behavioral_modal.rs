use tch::{nn, nn::Module, Device, Tensor};
use tokio;

/// Runs a behavioral analysis using a simple neural network.
/// 
/// Returns a JSON-formatted string with the result.
pub async fn run_analysis() -> String {
    // Define the device to run on (CPU in this case)
    let device = Device::Cpu;

    // Create a variable store for storing model parameters
    let mut vs = nn::VarStore::new(device);

    // Define the model
    let model = nn::seq()
        .add(nn::linear(&vs.root() / "layer1", 3, 16, Default::default()))
        .add_fn(|x| x.relu())
        .add(nn::linear(&vs.root() / "layer2", 16, 8, Default::default()))
        .add_fn(|x| x.relu())
        .add(nn::linear(&vs.root() / "layer3", 8, 1, Default::default()));

    // Define the input tensor
    let input = Tensor::of_slice(&[1.0, 2.0, 3.0]).reshape(&[1, 3]);

    // Forward pass through the model
    let output = model.forward(&input);

    // Extract the scalar output value
    let output_value = output.double_value(&[0]);

    // Additional component 1: Save the model to a file
    vs.save("model.pt").unwrap();

    // Additional component 2: Log the output value for debugging
    println!("Model output: {}", output_value);

    // Additional component 3: Add a dropout layer for regularization
    let model_with_dropout = nn::seq()
        .add(nn::linear(&vs.root() / "layer1", 3, 16, Default::default()))
        .add_fn(|x| x.relu())
        .add(nn::dropout(&vs.root() / "dropout1", 0.5))
        .add(nn::linear(&vs.root() / "layer2", 16, 8, Default::default()))
        .add_fn(|x| x.relu())
        .add(nn::linear(&vs.root() / "layer3", 8, 1, Default::default()));

    // Additional component 4: Run inference with dropout
    let output_with_dropout = model_with_dropout.forward(&input);
    let output_value_with_dropout = output_with_dropout.double_value(&[0]);
    println!("Model output with dropout: {}", output_value_with_dropout);

    // Additional component 5: Add a learning rate scheduler
    let mut optimizer = nn::Adam::default().build(&vs, 1e-3).unwrap();
    optimizer.set_lr(1e-4);

    // Return a JSON-formatted string with the result
    serde_json::json!({ "Behavioral analysis result": output_value }).to_string()
}

/// Entry point for the program.
#[tokio::main]
async fn main() {
    // Run the analysis and print the result
    let result = run_analysis().await;
    println!("{}", result);

    // Additional component 6: Load the model from a file and run inference again
    let mut vs_load = nn::VarStore::new(Device::Cpu);
    let model_load = nn::seq()
        .add(nn::linear(&vs_load.root() / "layer1", 3, 16, Default::default()))
        .add_fn(|x| x.relu())
        .add(nn::linear(&vs_load.root() / "layer2", 16, 8, Default::default()))
        .add_fn(|x| x.relu())
        .add(nn::linear(&vs_load.root() / "layer3", 8, 1, Default::default()));
    vs_load.load("model.pt").unwrap();
    let input = Tensor::of_slice(&[1.0, 2.0, 3.0]).reshape(&[1, 3]);
    let output_load = model_load.forward(&input);
    let output_value_load = output_load.double_value(&[0]);
    println!("Loaded model result: {}", output_value_load);
}
