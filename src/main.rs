pub mod error;
pub mod sim;
use sim::{BfSimu, CoreFuncBfSimu};
pub mod storage;
use storage::BfArrayImplementation;
fn main() {
    let mut simu = BfSimu::<BfArrayImplementation>::new(include_str!("code.bf"), "")
        .expect("Expected a bf simulator");
    print!("{}", simu.run().expect("Expected the program to work"));
}
