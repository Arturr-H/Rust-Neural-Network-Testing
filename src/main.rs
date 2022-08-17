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
    iter::Iterator
};
use rand::{ Rng, thread_rng, distributions::uniform::SampleRange };

/*- Constants -*/
const ACTIVATION_FNS:&'static [(&'static str, fn(f32) -> f32)] = &[
    ("sigmoid", sigmoid)
];

/*- Structs, enums & unions -*/
#[derive(Debug)]
struct NeuralNetwork<'lf> {
    input: &'lf [Neuron],
    hidden:&'lf [&'lf [Neuron] ],
    output:&'lf [Neuron],
    learning_rate: f32,
}

#[derive(Clone)]
struct Neuron {
    inner:f32,
    bias:f32,

    // These weights are connected to the neurons in
    // the next layer, in the same order. It's an option
    // because ex the output nodes won't have any weights
    weights:Option<Vec<f32>>
}

/*- Traits -*/
trait NeuronDefaultTraits {
    fn new() -> Neuron; // Initialize with all values being 0.0f32
    fn initialize_weights(network:NeuralNetwork<'static>, index:usize) -> Neuron;
}

/*- Initialize -*/
fn main() -> () {

    /*- Create the layers -*/
    let network:NeuralNetwork = NeuralNetwork {
        input: &[Neuron::new(), Neuron::new()],
        hidden: &[
            &[Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new()],
            &[Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new()],
        ],
        output: &[Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new(), Neuron::new()],
        learning_rate: 0.1,
    };

    /*- Print the layers -*/
    println!("{network:#?}");
    println!("{:#?}", sum_layer(&network, 3, 0));
}


/*- Functions -*/
fn get_layer<'lf>(network:&'lf NeuralNetwork<'static>, index:usize) -> &'static [Neuron] {
    let total_layers = network.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/;

    /*- Input layer -*/
    if index == 0 {
        return network.input;
    }else if index == total_layers - 1 {
        return network.output;
    }else {
        return match network.hidden.get(index - 1) {
            Some(e) => *e,
            None => &[]
        };
    }
}
fn sigmoid(input:f32) -> f32 { 1.0 / (1.0 + f32::exp(-input)) }
fn ReLU_leak(input:f32) -> f32 { if input > 0.0 { input } else { 0.01 * input } }
fn random_weights(len:usize) -> Vec<f32> {
    let mut vec:Vec<f32> = Vec::with_capacity(len);

    /*- Add random weights to the vec -*/
    for i in 0..len+1 {
        vec.push(thread_rng().gen_range::<f32, _>(-0.3..0.3))
    };

    /*- Return -*/
    vec
}
fn sum_layer(network:&NeuralNetwork, layer_index:usize, sum_for:usize) -> f32 {
    /*- If user wants to sum input -*/
    if layer_index == 0 {
        let mut sum:f32 = 0.0f32;
        for a in network.input { sum += a.inner; };
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
        Neuron { inner: 0.0, bias: 0.0, weights: Some(Vec::new()) }
    }

    /*- Create all the weights of a neuron - returns a
        neuron with weights depending on its output neurons -*/
    fn initialize_weights(network:NeuralNetwork<'static>, index:usize) -> Neuron {
        /*- Get the amount of layers in the network -*/
        let network_layer_len:usize = network.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/;

        /*- Check if index is output layer -*/
        if index == network_layer_len-1 {
            Neuron { inner: 0.0, bias: 0.0, weights:None }
        }else {
            let mut weights:&mut Vec<f32> = &mut Vec::new();

            /*- We'll be using the "he-et-al-initialization" algorithm to initialize all weights -*/
            /*- It's done by this formula: sqrt(2 / (amount of neurons in the next layer)) -*/
            let sqrt_2_div_next_layer_len = (2.0 / (get_layer(&network, index+1).len() as f32)).sqrt();

            /*- Add the weights to the vec -*/
            for i in 0..get_layer(&network, index+1).len() { weights.push(sqrt_2_div_next_layer_len); };

            /*- Return the neuron -*/
            Neuron { inner: 0.0, bias: 0.0, weights: Some(weights.to_vec()) }
            // /*- Check if index is input layer -*/
            // if index == 0 {
            //     let fan_in:u16 = 0;

            //     /*- Get how many output neurons the current neuron has -*/
            //     let fan_out:f32 = match network.hidden.get(0) {
            //         Some(e) => e.len() as f32,

            //         /*- If there are no hidden layers, we'll
            //             check how many neurons are in the output layer -*/
            //         None => {
            //             network.output.len() as f32
            //         }
            //     };

            //     /*-
            //      *- Algoritm, but not the real one. The real algorithm
            //      *- is like this: sqrt(2 / fan_in), but the input layer
            //      -* doesn't have any input neurons, so well use the outputs
            //     -*/
            //     weight = f32::sqrt(2.0 / fan_out)
            // }else {

            // }

            // Neuron {}
        }
    }
}

/*- For keeping output tidy, derive Debug
    impl will cause all keywords to display -*/
impl fmt::Debug for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*- ":.3" will format the numbers so that they are rounded with 3 decimals -*/
        write!(f, "Nc({:.3}s ~ {:.3}b)", self.inner, self.bias)
    }
}
