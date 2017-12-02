// TODO:
// * One call classify multiple problems
// * Use SIMD
// * Use parallelism
#![feature(toowned_clone_into)]
#![feature(test)]
#![feature(conservative_impl_trait)]    // to "return impl FnMut" 

#[macro_use] extern crate nom;
extern crate faster;
extern crate rand;
extern crate test;

pub mod types;
pub mod matrix;
pub mod csvm;
pub mod parser;



//
//fn benchmark() {
//    let model_str: &str = include_str!("test.model");
//    let model = parse_model(model_str).unwrap();
//    let mut problem = produce_problem(1, 10);
//
//    let mut csvm = CSVM::new(&model).unwrap();
//
//    // 256 0:0.3093766 1:0 2:0 3:0 4:0 5:0.1764706 6:0 7:0 8:1 9:0.1137485
//    problem.set_vector(0, &[0.3093766, 0.0, 0.0, 0.0, 0.0, 0.1764706, 0.0, 0.0, 1.0, 0.1137485]);
//    csvm.predict_probability_csvm(&problem);
//
//    // -256 0:0.3332312 1:0 2:0 3:0 4:0.09657142 5:1 6:0 7:0 8:1 9:0.09917226
//    problem.set_vector(0, &[0.3332312, 0.0, 0.0, 0.0, 0.09657142, 1.0, 0.0, 0.0, 1.0, 0.09917226]);
//    csvm.predict_probability_csvm(&problem);
//
//}
