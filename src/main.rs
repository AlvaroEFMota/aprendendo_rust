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
//mod enums;
//mod cli;
//mod generics;
mod sort;


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
    //enums::run();
    //cli::run();
    //generics::run();

    let mut v: [i32; 9] = [3, 1, 4, 5, 2, 9, 7, 8, 6];
    let end = v.len();
    sort::merge_sort(&mut v, 0, end);
    println!("{:?}", v);
}
