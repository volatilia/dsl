extern crate dsl_macro;
use dsl_macro::AnswerFn;

#[derive(AnswerFn)]
struct Struct;

macro_rules! add_as{
    (
  $($a:expr)
   *
   )=>{
       {
   0
   $(+$a)*
     }
    }
}

fn main() {
    assert_eq!(42, answer());
}
