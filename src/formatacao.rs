pub fn run(){
    let n1 = 5;
    let n2 = 4.1;
    let n3 = 0b00001;

    //Print to Console
    print!("Impressão sem ");
    println!("quebra de linha");

    //Basic Formatting
    println!("{} {} {} {}","Álvaro", "Ernani", "Fonseca", "Mota");
    println!("números: [{}] [{}] [{}] [{}] [{}]", 5, 4.1, 0xff, 0b0001101010101, 0o774);
    
    //Named Arguments
    println!("números: [{inteiro}] [{flutuante}] [{binario}]", inteiro = n1, flutuante = n2, binario = n3);
    
    //Positional Arguments
    println!("{0} {0} {1} {2} {3}","Álvaro", "Ernani", "Fonseca", "Mota");

    //Placeholder Traits
    println!("Binário:{:b}  Hexadecimal:{:x}  Octal:{:o}", 50, 50, 50);
}