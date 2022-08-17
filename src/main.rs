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
use std::{
    fmt,
    iter::{
        Iterator,
        IntoIterator
    }
};
use rand::{ Rng, thread_rng, distributions::uniform::SampleRange };

/*- Constants -*/
const ACTIVATION_FNS:&'static [(&'static str, fn(f32) -> f32)] = &[
    ("sigmoid", sigmoid)
];

/*- Structs, enums & unions -*/
#[derive(Debug)]
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

/*- Traits -*/
trait NeuronDefaultTraits {
    fn new() -> Neuron; // Initialize with all values being 0.0f32
    fn with_inner(inner:f32) -> Neuron; // Initialize with all values being 0.0f32
}

/*- Initialize -*/
fn main() -> () {

    /*- Create the layers -*/
    let network:NeuralNetwork = initialize_weights(&NeuralNetwork {
        input: vec![Neuron::with_inner(0.24124f32), Neuron::with_inner(0.51251f32)],
        hidden: vec![
            vec![Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new()],
            vec![Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new()],
        ],
        output: vec![Neuron::new(), Neuron::new()],
        learning_rate: 0.1,
    });


    /*- Print the layers -*/
    println!("{network:#?}");
    println!("{:#?}", sum_layer(&network, 3, 0));
    println!("{:#?}", network);
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
fn sigmoid(input:f32) -> f32 { 1.0 / (1.0 + f32::exp(-input)) }
fn ReLU_leak(input:f32) -> f32 { if input > 0.0 { input } else { 0.01 * input } }
fn random_weights(len:usize) -> Vec<f32> {
    let mut vec:Vec<f32> = Vec::with_capacity(len);
    if len == 0 { vec }
    else {
        
        /*- Add random weights to the vec -*/
        for i in 0..len+1 {
            vec.push(thread_rng().gen_range::<f32, _>(-0.3..0.3))
        };
        
        /*- Return -*/
        vec
    }
}
fn sum_layer(network:&NeuralNetwork, layer_index:usize, sum_for:usize) -> f32 {
    /*- If user wants to sum input -*/
    if layer_index == 0 {
        let mut sum:f32 = 0.0f32;
        for a in network.input.to_vec() { sum += a.inner; };
        sum
    }else {
        let mut sum:f32 = 0.0f32;

        /*- Get the layer -*/
        let layer = network.hidden.get(layer_index - 1);
        println!("layer {layer:?}");
        match layer {
            Some(e) => {
                for a in e.iter() { sum += a.inner; };
                sum
            },
            None => 0.0f32
        }
    }
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
        write!(f, "Nc({:.3}s ~ {:.3}b ~ {:?})", self.inner, self.bias, self.weights)
    }
}
