/*- Imports -*/
use crate::{
    NeuralNetwork, Neuron,
    get_layer,

    sigmoid, ReLU_leak,
};

/// /*- Calculating neuron values -*/
pub fn calculate_inner(network:&NeuralNetwork, layer_index:usize, neuron_index:usize) -> f32 {
    /*- Get the layers -*/
    let curr_layer = get_layer(network, layer_index);
    let prev_layer = get_layer(network, layer_index - 1);
    let mut sum:f32 = 0.;

    /*- Loop through every neuron in the previous layer -*/
    for previous_neuron in prev_layer {
        /*- < previous_neuron.weights[neuron_index] > will grab the corresponding weight to this neuron -*/
        sum += previous_neuron.inner * previous_neuron.weights[neuron_index];
    }

    /*- Return the sum -*/
    sum
}
/// Iterate over all neurons from input to output and calculate
/// their inner values using the previous neurons' inner value, weight
/// and the current bias.
pub fn calculate_all_inners(network:&NeuralNetwork) -> NeuralNetwork {
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
