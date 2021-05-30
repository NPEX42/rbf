
use rbf;

fn main() {

    let mut bfi = rbf::BrainFuckInterpreter::new("+++#---++++++++++."); //Print 3\n
    
    bfi.run();

    println!("{}", bfi.get_ref())
}
