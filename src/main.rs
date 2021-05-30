
use rbf;

fn main() {

    let bfi = rbf::BrainFuckInterpreter::new("+++#---++++++++++."); //Print 3\n
    
    bfi.run();
}
