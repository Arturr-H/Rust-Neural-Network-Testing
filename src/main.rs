/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_assignments,
    unused_variables,
    non_snake_case
)]

/*- Imports -*/
use std::{fmt, io::Read};
use serde::{Deserialize, Serialize};
use rand::{ Rng, thread_rng };

/*- Constants -*/
const EPOCHS:usize = 100usize;
const ACTIVATION_FNS:&'static [(&'static str, fn(f32) -> f32)] = &[
    ("sigmoid", sigmoid)
];

/*- Structs, enums & unions -*/
#[derive(Debug, Clone)]
struct NeuralNetwork {
    input: Vec<Neuron>,
    hidden:Vec<Vec<Neuron>>,
    output:Vec<Neuron>,
    learning_rate: f32,
}

#[derive(Clone)]
struct Neuron {
    inner:f32,
    bias:f32,

    // These weights are connected to the neurons in
    // the next layer, in the same order.
    weights:Vec<f32>
}

/*- Each epoch will go through training data -*/
#[derive(Debug)]
struct TrainingData<V1,V2> {
    label: Vec<V1>, data: Vec<V2>
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonData {
    items: Vec<(Vec<u8>, Vec<u8>)>
}
/*- Traits -*/
trait NeuronDefaultTraits {
    fn new() -> Neuron; // Initialize with all values being 0.0f32
    fn with_inner(inner:f32) -> Neuron; // Initialize with all values being 0.0f32
}

/*- Functions -*/
fn get_layer<'lf>(network:&'lf NeuralNetwork, index:usize) -> Vec<Neuron> {
    let total_layers = network.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/;

    /*- Input layer -*/
    if index == 0 {
        return network.input.to_vec();
    }else if index == total_layers - 1 {
        return network.output.to_vec();
    }else {
        return match network.hidden.get(index - 1) {
            Some(e) => e.to_vec(),
            None => Vec::with_capacity(0)
        };
    }
}
fn random_weights(len:usize) -> Vec<f32> {
    let mut vec:Vec<f32> = Vec::with_capacity(len);
    if len == 0 { vec }
    else {
        
        /*- Add random weights to the vec -*/
        for i in 0..len+1 {
            vec.push(thread_rng().gen_range::<f32, _>(-0.6..0.6))
        };
        
        /*- Return -*/
        vec
    }
}

/*- Calculating neuron values -*/
fn calculate_inner(network:&NeuralNetwork, layer_index:usize, neuron_index:usize) -> f32 {
    /*- Get the layers -*/
    let curr_layer = get_layer(network, layer_index);
    let prev_layer = get_layer(network, layer_index - 1);

    /*- Get the current neuron -*/
    let neuron = &match curr_layer.get(neuron_index) {
        Some(e) => e,
        None => panic!("AWBDOIAWBDOIAWOIBDOAIWD")
    };
    
    /*- Sum of all weights, biases and neuron inners -*/
    let mut sum = neuron.inner;

    /*- Loop through every neuron in the previous layer -*/
    for previous_neuron in prev_layer {
        /*- < previous_neuron.weights[neuron_index] > will grab the corresponding weight to this neuron -*/
        sum += previous_neuron.inner * previous_neuron.weights[neuron_index];
    }

    sum
}
fn calculate_all_inners(mut network:&NeuralNetwork) -> NeuralNetwork {
    /*- Get the amount of layers in the network -*/
    let network_layer_len:usize = network.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/;
    let mut all_layers:Vec<Vec<Neuron>> = Vec::with_capacity(network_layer_len);
    let mut network = network.clone();

    /*- Iterate over all the layers, skip input
        because their inner should already be set -*/
    for layer_index in 1..network_layer_len {
        /*- Get the layer -*/
        let layer = get_layer(&network, layer_index);
        let mut layer_mut = layer.clone();

        /*- Iterate over all neurons -*/
        for (neuron_index, neuron) in layer.iter().enumerate() {
            /*- If it's the output layer, we'll change the activation function to sigmoid -*/
            if layer_index == network_layer_len - 1 {
                layer_mut[neuron_index].inner = sigmoid(
                    calculate_inner(
                        &network,
                        layer_index,
                        neuron_index
                    ) + layer_mut[neuron_index].bias
                );
            }else {
                layer_mut[neuron_index].inner = ReLU_leak(
                    calculate_inner(
                        &network,
                        layer_index,
                        neuron_index
                    ) + layer_mut[neuron_index].bias
                );
            };
        };

        /*- Set the all neutrons' inners in the new layer -*/
        if layer_index == network_layer_len - 1 {
            network.output = layer_mut.clone();
        }else {
            network.hidden[layer_index - 1] = layer_mut.clone();
        }
    };

    /*- Return -*/
    network
}

/*- Activation functions -*/
fn sigmoid(input:f32) -> f32 { 1.0 / (1.0 + f32::exp(-input)) }
fn ReLU_leak(input:f32) -> f32 { if input > 0.0 { input } else { 0.01 * input } }
fn is_active(input:f32) -> bool { input > 0.5 }

/*- Cost functions -*/
fn mean_squared_error(compare_to:Vec<u8>, network:&NeuralNetwork) -> f32 {
    let mut sum:f32 = 0.0;

    /*- Loop through all the neurons in the output layer -*/
    for (neuron_index, neuron) in network.output.iter().enumerate() {
        /*- Get the neuron to compare to -*/
        let compare_neuron = match compare_to.get(neuron_index) {
            Some(e) => e,
            None => panic!("AWBDOIAWBDOIAWOIBDOAIWD")
        };

        /*- Add the squared difference to the sum -*/
        sum += f32::powf(neuron.inner - *compare_neuron as f32, 2.0);
    };

    /*- Return avg -*/
    sum / network.output.len() as f32
}

/*- Implementations -*/
impl NeuronDefaultTraits for Neuron {

    /*- Default the neuron to 0.0f32 -*/
    fn new() -> Neuron {
        Neuron { inner: 0.0, bias: 0.0, weights: Vec::new() }
    }

    /*- Create a neuron with a specified inner value -*/
    fn with_inner(inner:f32) -> Neuron {
        Neuron { inner, bias: 0.0, weights: Vec::new() }
    }
}

/*- Create all the weights of every neuron - returns a neural network struct
    neuron containing neurons with weights depending on its output neurons -*/
fn initialize_weights(network:&NeuralNetwork) -> NeuralNetwork {
    /*- Get the amount of layers in the network -*/
    let network_layer_len:usize = network.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/;
    let mut all_layers:Vec<Vec<Neuron>> = Vec::with_capacity(network_layer_len);

    /*- Iterate over all the layers -*/
    for i in 0..network_layer_len {
        /*- Get the layer -*/
        let mut layer = get_layer(network, i);
        let next_layer_len = get_layer(network, i + 1).len();

        /*- Iterate over all the neurons in the layer -*/
        for j in 0..layer.len() {
            /*- Get the neuron -*/
            let mut neuron:Option<&mut Neuron> = layer.get_mut(j);

            /*- If the neuron has weights -*/
            match neuron {
                Some(e) => {
                    /*- Create the weights -*/
                    e.weights = random_weights(next_layer_len.checked_sub(1).unwrap_or(0));
                },
                None => ()
            }
        }

        /*- Add the layer to the all_layers -*/
        all_layers.push(layer);
    };

    /*- Return -*/
    NeuralNetwork {
        input: all_layers.get(0).unwrap_or(&Vec::new()).to_vec(),
        hidden: all_layers.get(1..all_layers.len() - 1).unwrap_or(&Vec::new()).to_vec(),
        output: all_layers.get(all_layers.len() - 1).unwrap_or(&Vec::new()).to_vec(),
        learning_rate: network.learning_rate,
    }
}

/*- For keeping output tidy, derive Debug
    impl will cause all keywords to display -*/
impl fmt::Debug for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*- ":.3" will format the numbers so that they are rounded with 3 decimals -*/
        write!(f, "Nc({:.6}s ~ {:.3}b ~ {:?})", self.inner, self.bias, self.weights)
    }
}

/*- Quick methods for training data -*/
impl TrainingData<u8, u8> {
    pub fn new(label:&Vec<u8>, data:&Vec<u8>) -> TrainingData<u8, u8> {
        TrainingData { label: label.to_vec(), data: data.to_vec() }
    }
}

/*- Initialize -*/
fn main() -> () {

    /*- Create the layers -*/
    let network:NeuralNetwork = calculate_all_inners(&initialize_weights(&NeuralNetwork {
        input: vec![Neuron::new(); 10],
        hidden: vec![
            vec![Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new()],
            vec![Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new()],
        ],
        output: vec![Neuron::new(), Neuron::new()],
        learning_rate: 0.1,
    }));

    /*- Load data and labels -*/
    let data_string:String = match std::fs::File::open("./data.json") {
        Ok(mut e) => {
            let mut content_string:String = String::new();
            match e.read_to_string(&mut content_string) {
                Ok(e) => e,
                Err(_) => {
                    println!("Couldn't load data! (12)");
                    return;
                }
            };

            content_string
        },
        Err(_) => {
            println!("Couldn't load data! (11)");
            return;
        }
    };
    let json_data:JsonData = serde_json::from_str(&data_string).unwrap();
    let data:Vec<TrainingData<u8, u8>> = json_data.items.iter().map(|e| TrainingData::new(&e.0, &e.1)).collect();

    println!("{data:?}");

    /*- Train -*/
    for epoch in 0..EPOCHS {
        /*- Calculate the cost -*/
        let cost:f32 = mean_squared_error(vec![0, 1], &network);

        /*- Print the cost -*/
        println!("epoch: {:?} cost: {:?}", epoch, cost);

        /*- If the cost is low enough -*/
        if cost < 0.01 {
            break;
        };

        /*- Iterate over data and labels -*/
        // TODO

    };

    /*- Print the layers -*/
    println!("{network:#?}");

    for output_neuron in network.output.iter() {
        println!("active: {:?}", is_active(output_neuron.inner));
    }


}
