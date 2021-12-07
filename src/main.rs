pub mod error;
pub mod sim;
use sim::{BfSimu, CoreFuncBfSimu};
pub mod storage;
use storage::BfArrayImplementation;
fn main() {
    let mut simu = BfSimu::<BfArrayImplementation>::new(
        "++++++++++[>+++++++>++++++++++>+++>++++<<<<-]>++.>+.+++++++..+++.>>++++.<++.<++++++++.--------.+++.------.--------.>+.",
        "",
    )
    .expect("Expected a bf simulator");
    println!(
        "Output: {:?}",
        simu.run().expect("Expected the program to work")
    );
}
