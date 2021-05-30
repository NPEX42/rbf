
use rbf;

fn main() {

    let bfi = rbf::BrainFuckInterpreter::new("+++#---++++++++++.");
    
    bfi.run();
}
