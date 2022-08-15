/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_assignments,
    unused_variables
)]

/*- Imports -*/
use std::fmt;
use rand::{ Rng, thread_rng };

/*- Constants -*/


/*- Structs, enums & unions -*/
struct Neuron {
    inner:f32,
}

/*- Traits -*/
trait NeuronDefaultTraits {
    fn rng() -> Neuron;
}

/*- Initialize -*/
fn main() -> () {

    /*- Create the layers -*/
    let layers:&[&[Neuron]] = &[
        //-   INPUT NEURONS   -//
        &[Neuron::rng(), Neuron::rng(), Neuron::rng(), Neuron::rng(), Neuron::rng()],
        //-   -------------   -//
    
        &[Neuron::rng(), Neuron::rng(), Neuron::rng(), Neuron::rng(), Neuron::rng()],
        &[Neuron::rng(), Neuron::rng(), Neuron::rng(), Neuron::rng(), Neuron::rng()],
    
        //-   OUTPUT NEURONS   -//
        &[Neuron::rng(), Neuron::rng(), Neuron::rng(), Neuron::rng(), Neuron::rng()],
        //-   --------------   -//
    ];

    println!("{layers:?}");

}

/*- Functions -*/

/*- Implementations -*/
impl NeuronDefaultTraits for Neuron {

    /*- Default theneuron to a random value -*/
    fn rng() -> Neuron {
        Neuron {
            inner: thread_rng().gen_range::<f32, _>(0.0..1.0)
        }
    }
}
impl fmt::Debug for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nc({})", self.inner)
    }
}