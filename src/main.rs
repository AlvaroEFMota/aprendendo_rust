/**
 * Any comment notation like this [1-5] means the exemple 1-5 of the Rust Book
 */


//mod formatacao;
//mod variaveis_e_constantes;
//mod types;
//mod strings;
//mod tuples;
//mod arrays;
//mod vectors;
//mod conditionals;
//mod loops;
//mod functions;
//mod pointer_ref;
//mod structs;
mod enums;
//mod cli;
//mod generics;
//mod guessing_game;
//mod ownership1;
//mod ownership2;
//mod slices;
//mod sort;

fn main() {
    //formatacao::run();
    //variaveis_e_constantes::run();
    //types::run();
    //strings::run();
    //tuples::run();
    //arrays::run();
    //vectors::run();
    //conditionals::run();
    //loops::run();
    //functions::run();
    //pointer_ref::run();
    //structs::run();
    enums::run();
    //cli::run();
    //generics::run();
    //guessing_game::run();
    //ownership1::run();
    //ownership2::run();
    //slices::run();

    /*let mut v: [i32; 9] = [3, 1, 4, 5, 2, 9, 7, 8, 6];
    let end = v.len();
    //sort::merge_sort(&mut v, 0, end);
    //sort::bubble_sort(&mut v, 0, end);
    //sort::selection_sort(&mut v, 0, end);
    //v.sort();
    println!("sort {:?}", v);
    */


}

#[test]
fn teste() { // Teste exempe. To run, type cargo test
    let a = 10;
    assert_eq!(a, 10);
}