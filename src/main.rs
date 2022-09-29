/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_assignments,
    unused_variables,
    non_snake_case
)]

/*- Modules & Imports -*/
mod calculate_inners;
mod network_utils;
mod network_init;
#[path = "structs/neuron.rs"] mod neuron;
#[path = "structs/network.rs"] mod network;
use neuron::Neuron;
use network::{ NeuralNetwork, NNIntoIterator };
use calculate_inners::calculate_all_inners;
use network_utils::{ get_layer, get_training_data, TrainingData };
use network_init::initialize_weights;
use rand::{ Rng, thread_rng };
use std::{ fmt, io::Read };

/*- Constants -*/
static mut ERRS:u32 = 0;
const EPOCHS:usize = 3usize;
const LEARNING_RATE:f32 = 0.1f32;

/*- Activation functions -*/
fn sigmoid(input:f32) -> f32 { 1.0 / (1.0 + f32::exp(-input)) }
fn ReLU_leak(input:f32) -> f32 { if input > 0.0 { input } else { 0.01 * input } }
fn large_step(input:f32) -> bool { input > 0.5 }

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

/*- Initialize -*/
fn main() -> () {

    /*- Create the layers -*/
    let mut network:NeuralNetwork = NeuralNetwork::new(10, (2, 5), 2);

    // TODO removing this line will cause mainloop not to update any neurons, whilst it should have set it as training data.
    network.input = vec![Neuron::with_inner(1.0); 10];

    /*- Initialize weights for all layers exept output (not needed) -*/
    network = initialize_weights(&network);
    
    /*- Grab training data & labels from json file -*/
    let training_data = get_training_data("./data.json");

    /*- Train -*/
    for epoch in 0..1 {
        /*- Calculate the cost -*/
        let cost:f32 = mean_squared_error(vec![0, 1], &network);

        /*- Print the cost -*/
        println!("epoch: {:?} cost: {:?}", epoch, cost);

        /*- If the cost is low enough -*/
        if cost < 0.0001 {
            break;
        };

        /*- Iterate over data and labels -*/
        for TrainingData { label, data } in &training_data {

            /*- Change all input neurons inner values, but we won't change their weights yet. -*/
            for (index, mut input_neuron) in network.input.clone().iter().enumerate() {
                match network.input.get(index) {
                    Some(mut e) => {
                        let change_to = match data.get(index) {
                            Some(e) => *e as f32,
                            None => {
                                /*- Unsafe is actually safe here because
                                    we don't use any multithreading -*/
                                unsafe { ERRS += 1; };
                                0.
                            },
                        };

                        /*- Change inner -*/
                        network.input[index].inner = change_to;
                    },
                    None => {
                        unsafe { ERRS += 1; };
                        ()
                    },
                };
            };

            /*- Calculate all inners -*/
            network = calculate_all_inners(&network);

            println!("BE {}", network.input[0].weights[0]);

            /*- Loop through all the neurons in the output layer -*/
            for (neuron_index, mut neuron) in network.output.clone().iter_mut().enumerate() {
                /*- Get the neuron to compare to -*/
                let compare_neuron = match label.get(neuron_index) {
                    Some(e) => e,
                    None => panic!("AWBDOIAWBDOIAWOIBDOAIWD")
                };

                /*- Calculate the error & delta -*/
                let error:f32 = *compare_neuron as f32 - neuron.inner;
                let delta:f32 = error * neuron.inner * (1.0 - neuron.inner);

                /*- Backpropagate / iterate backwards -*/
                let mut layers_iter = network.clone().into_iter().prepare_back_iteration();
                println!("{}", layers_iter.len());

                // TODO Loop through all layers
                while let (index, Some(layer)) = layers_iter.prev_with_index() {
                    /*- Loop through all neurons in the layer -*/
                    for mut neuron in layer {
                        /*- Loop through all weights in the neuron -*/
                        for weight_index in 0..neuron.weights.len() {
                            /*- Calculate the weight change -*/
                            let weight_change:f32 = delta * neuron.weights[weight_index];

                            /*- Change the weight -*/
                            neuron.weights[weight_index] += weight_change;
                        };
                    };
                };

                /*- Loop through all the weights in the neuron -*/
                // for (weight_index, mut weight) in network.input[neuron_index].weights.clone().iter_mut().enumerate() {
                //     /*- Get the neuron to compare to -*/
                //     let compare_neuron = match network.input.get(weight_index) {
                //         Some(e) => e,
                //         None => panic!("AWBDOIAWBDOIAWOIBDOAIWD")
                //     };

                //     /*- Calculate the weight change -*/
                //     let weight_change:f32 = LEARNING_RATE * delta * compare_neuron.inner;

                //     /*- Change the weight -*/
                //     network.input[neuron_index].weights[weight_index] += weight_change;
                // };
            };

            println!("AF {}", network.input[0].weights[0]);

            /*- Calculate the inner values of the neurons -*/
            network = calculate_all_inners(&network);
        };

        println!("Main loop errors: {}", unsafe { ERRS });
    };

    /*- Print the layers -*/
    println!("{:#?}", network);
}
