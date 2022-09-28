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
use network::NeuralNetwork;
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
            for (index, mut input_neuron) in network.input.iter().enumerate() {
                match network.input.clone().get_mut(index) {
                    Some(mut e) => {

                        /*- Change inner -*/
                        e.inner = match data.get(index) {
                            Some(e) => *e as f32,
                            None => {
                                /*- Unsafe is actually safe here because
                                    we don't use any multithreading -*/
                                unsafe { ERRS += 1; };
                                0.
                            },
                        }
                    },
                    None => {
                        unsafe { ERRS += 1; };
                        ()
                    },
                }
            }

            /*- Calculate the inner values of the neurons -*/
            network = calculate_all_inners(&network);
        };

        println!("Main loop errors: {}", unsafe { ERRS });
    };

    /*- Print the layers -*/
    println!("{:#?}", network);
}
