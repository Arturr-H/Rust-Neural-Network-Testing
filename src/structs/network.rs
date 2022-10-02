/*- Imports -*/
use serde::{ Serialize, Deserialize };
use super::neuron::Neuron;
use bincode;
use std::{ path, fs, io::{ Read, Write } };

/*- Structs, enums & unions -*/
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /*- Get layer by index -*/
    pub fn get_layer(&self, index:usize) -> Option<&Vec<Neuron>> {

        /*- Get total amount of layers in network -*/
        let total_length:usize = self.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/;

        /*- Check if index is valid -*/
        let input_and_hidden = match index {
            0 => Some(&self.input),
            1.. => self.hidden.get(index - 1),
            _ => None,
        };

        /*- Check for output layer -*/
        match input_and_hidden {
            Some(e) => Some(e),
            None => {
                if index == total_length - 1 {
                    Some(&self.output)
                } else {
                    None
                }
            }
        }
    }
    pub fn get_layer_mut(&mut self, index:usize) -> Option<&mut Vec<Neuron>> {

        /*- Get total amount of layers in network -*/
        let total_length:usize = self.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/;

        /*- Check if index is valid -*/
        let input_and_hidden = match index {
            0 => Some(&mut self.input),
            1.. => self.hidden.get_mut(index - 1),
            _ => None,
        };

        /*- Check for output layer -*/
        match input_and_hidden {
            Some(e) => Some(e),
            None => {
                if index == total_length - 1 {
                    Some(&mut self.output)
                } else {
                    None
                }
            }
        }
    }

    /*- Save neural network to file via bincode -*/
    pub fn save(&self, path:&str) -> Result<(), Box<bincode::ErrorKind>> {
        let serialized:Vec<u8> = bincode::serialize::<NeuralNetwork>(self)?;
        let path = path::Path::new(&path);

        /*- Write to path -*/
        if path.is_file() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .open(path)?;
            file.set_len(0)?;
            file.write_all(&serialized)?;

            Ok(())
        }else {
            match fs::File::create(path) {
                Ok(mut e) => {
                    match e.write_all(&serialized[..]) {
                        Ok(_) => Ok(()),
                        Err(_) => return Err(
                            Box::new(
                                bincode::ErrorKind::Custom("Write to file error (1)".into())
                            )
                        )
                    }
                },
                Err(_) => return Err(
                    Box::new(
                        bincode::ErrorKind::Custom("Create file error".into())
                    )
                )
            }
        }
    }
    pub fn from_file(path:&str) -> Result<Self, Box<bincode::ErrorKind>> {
        let path = path::Path::new(&path);
        let mut content:Vec<u8> = Vec::new();

        /*- Read from path -*/
        if path.is_file() {
            let mut file = fs::OpenOptions::new()
                .read(true)
                .open(path)?;
            file.read_to_end(&mut content)?;

            /*- Deserialize -*/
            return match bincode::deserialize::<NeuralNetwork>(&content[..]) {
                Ok(e) => Ok(e),
                Err(e) => Err(Box::new(bincode::ErrorKind::Custom(e.to_string())))
            };
        }else {
            return Err(Box::new(bincode::ErrorKind::Custom("File not found".into())))
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
    pub index: usize,
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
impl NNIntoIterator {
    pub fn get(&self, index:usize) -> Option<Vec<Neuron>> {
        let input_n_hidden = match index {
            0 => Some(self.network.input.clone()),
            _ => self.network.hidden.get(index - 1).map(|e| e.to_vec()),
        };

        /*- Might be output -*/
        match input_n_hidden {
            Some(_) => input_n_hidden,
            None => {
                if index == self.network.hidden.len() + 1 {
                    Some(self.network.output.clone())
                }
                else { None }
            }
        }
    }
    pub fn prepare_back_iteration(&self) -> Self  {
        NNIntoIterator {
            network: self.network.clone(),
            index: self.network.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/ + 1,
        }
    }
    pub fn prev(&mut self) -> Option<Vec<Neuron>> {

        /*- Get total amount of layers in network -*/
        let total_length:usize = self.network.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/;

        /*- Decrease index -*/
        match self.index - 1 {
            0 => None,
            _ => {
                self.index -= 1;
                self.get(self.index - 1)
            }
        }
    }
    pub fn len(&self) -> usize {
        self.network.hidden.len() + 1 /*- Input -*/ + 1 /*- Output -*/
    }
    pub fn new(network:&NeuralNetwork) -> NNIntoIterator {
        NNIntoIterator {
            network: network.clone(),
            index: 0,
        }
    }

    /*- Convert NNIntoIterator into a NeuralNetwork struct -*/
    pub fn restore(self) -> NeuralNetwork {
        self.network
    }
}

impl std::fmt::Debug for NNIntoIterator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NNIntoIteratorNetwork: {:?}", self.network)
    }
}
