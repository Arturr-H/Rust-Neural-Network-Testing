/*- Imports -*/
use std::fmt;

/*- Main -*/
#[derive(Clone)]
pub struct Neuron {
    pub inner:f32,
    pub bias:f32,

    // These weights are connected to the neurons in
    // the next layer, in the same order.
    pub weights:Vec<f32>
}

/*- Implementations -*/
impl Neuron {

    /*- Default the neuron to 0.0f32 -*/
    pub fn new() -> Neuron {
        Neuron { inner: 0.0, bias: 0.0, weights: Vec::new() }
    }

    /*- Create a neuron with a specified inner value -*/
    pub fn with_inner(inner:f32) -> Neuron {
        Neuron { inner, bias: 0.0, weights: Vec::new() }
    }

    /*- Actual functionality -*/
    pub fn update_weights(&mut self, learning_rate:f32, delta:f32, prev_layer:&Vec<Neuron>) {

        /*- Update weights -*/
        for (i, neuron) in prev_layer.iter().enumerate() {
            self.weights[i] -= learning_rate * delta * neuron.inner;
        };
    }
    pub fn update_bias(&mut self, learning_rate:f32, delta:f32) {
        self.bias -= learning_rate * delta;
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
    