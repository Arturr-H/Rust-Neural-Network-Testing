/*- Imports -*/
use crate::{ NeuralNetwork, Neuron, get_layer };
use rand::{ Rng, thread_rng };

/*- Create all the weights of every neuron - returns a neural network struct
    neuron containing neurons with weights depending on its output neurons -*/
pub fn initialize_weights(network:&NeuralNetwork) -> NeuralNetwork {
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
    NeuralNetwork::from_layers(all_layers)
}

/*- Generate a vector containing random weights -*/
pub fn random_weights(len:usize) -> Vec<f32> {
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