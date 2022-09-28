/*- Imports -*/
use crate::{ NeuralNetwork, Neuron };
use std::io::Read;
use serde::{ Deserialize, Serialize };

/*- Each epoch will go through training data -*/
#[derive(Debug)]
pub struct TrainingData<V1,V2> {
    pub label: Vec<V1>,
    pub data: Vec<V2>
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonData {
    items: Vec<(Vec<u8>, Vec<u8>)>
}

/*- Quick methods for training data -*/
impl TrainingData<u8, u8> {
    pub fn new(label:&Vec<u8>, data:&Vec<u8>) -> TrainingData<u8, u8> {
        TrainingData { label: label.to_vec(), data: data.to_vec() }
    }
}

/*- Functions -*/
pub fn get_layer<'lf>(network:&'lf NeuralNetwork, index:usize) -> Vec<Neuron> {
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


/*- Get labels and training data -*/
pub fn get_training_data(file_path:&str) -> Vec<TrainingData<u8, u8>> {
    /*- Load training data and labels -*/
    let data_string:String = match std::fs::File::open(file_path) {
        Ok(mut e) => {
            let mut content_string:String = String::new();
            match e.read_to_string(&mut content_string) {
                Ok(e) => e,
                Err(_) => panic!("Couldn't load data! (12)")
            };

            content_string
        },
        Err(_) => panic!("Couldn't load data! (11)")
    };
    let json_data:JsonData = serde_json::from_str(&data_string).unwrap();
    let training_data:Vec<TrainingData<u8, u8>> = json_data.items.iter().map(|e| TrainingData::new(&e.0, &e.1)).collect();

    training_data
}
