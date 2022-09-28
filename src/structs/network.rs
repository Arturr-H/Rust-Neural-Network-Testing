/*- Imports -*/
use super::neuron::Neuron;

/*- Structs, enums & unions -*/
#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    pub input: Vec<Neuron>,
    pub hidden:Vec<Vec<Neuron>>,
    pub output:Vec<Neuron>,
}

/*- Quick setup methods for neural network struct -*/
impl NeuralNetwork {
    pub fn new(input_num:usize, hidden:(usize,usize), output_num:usize) -> NeuralNetwork {
        NeuralNetwork {
            input: vec![Neuron::new(); input_num],
            hidden: vec![
                vec![Neuron::new(); hidden.1]; hidden.0
            ],
            output: vec![Neuron::new(); output_num]
        }
    }

    /*- Convert vec of layers to network -*/
    pub fn from_layers(layers:Vec<Vec<Neuron>>) -> NeuralNetwork {
        NeuralNetwork {
            input: layers.get(0).unwrap_or(&Vec::new()).to_vec(),
            hidden: layers.get(1..layers.len() - 1).unwrap_or(&Vec::new()).to_vec(),
            output: layers.get(layers.len() - 1).unwrap_or(&Vec::new()).to_vec(),
        }
    }
}

/*- Implement iterator for NeuralNetwork for quickly being able to iterate over layers -*/
impl IntoIterator for NeuralNetwork {
    type Item = Vec<Neuron>;
    type IntoIter = NNIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        NNIntoIterator {
            network: self,
            index: 0,
        }
    }
}

/*- Struct iterator to keep track of index -*/
pub struct NNIntoIterator {
    network: NeuralNetwork,
    index: usize,
}

/*- Implement iterator for NNIntoIterator -*/
impl Iterator for NNIntoIterator {
    type Item = Vec<Neuron>;
    fn next(&mut self) -> Option<Vec<Neuron>> {

        /*- Get total amount of layers in network -*/
        let total_length:usize = self.network.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/;

        /*- Output layer -*/
        if self.index == total_length - 1 {
            /*- Increase index -*/
            self.index += 1;
            Some(self.network.output.clone())
        }else {
            /*- Input & Hidden layers -*/
            let result:Option<Vec<Neuron>> = match self.index {
                0 => Some(self.network.input.clone()),
                _ => self.network.hidden.get(self.index - 1).map(|e| e.to_vec()),
            };
            
            /*- Increase index -*/
            self.index += 1;
            result.clone()
        }
    }
}
