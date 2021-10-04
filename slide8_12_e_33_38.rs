//slides de 8 a 12 e 33 a 38

fn main() {
    let  a: u32= 0x6DB7;
    let  b: u32= 0xA726;

    println!(" Operadores Bitwise Lógicos ");
    println!("\n");

    println!("Suponha que a e b sejam variáveis \n inteiras sem sinal de valores 0x6DB7 e 0xA726\n Qual o valor de ~a");
    println!("\n");
    println!(" a = {:032b } = {:0x} ", a, a);
    println!("~a = {:032b } = {:0x} ", !a, !a);
    println!("\n");
    println!("----------------------------------------------------------------------------");
    println!("Suponha que a e b sejam variáveis \n inteiras sem sinal de valores 0x6DB7 e 0xA726\n Qual o valor de ~b");
    println!("\n");
    println!(" b = {:032b } = {:0x} ", b, b);
    println!("~b = {:032b } = {:0x} ", !b, !b);
    println!("\n");
    println!("----------------------------------------------------------------------------");
    println!("Suponha que a e b sejam variáveis \n inteiras sem sinal de valores 0x6DB7 e 0xA726\n Qual o valor de a & b");
    println!("\n");
    let c: u32 = a&b;
    println!(" a = {:032b } = {:0x} ", a, a);
    println!(" b = {:032b } = {:0x} ", b, b);
    println!(" c = {:032b } = {:0x} ", c, c);
    println!("\n");
    println!("----------------------------------------------------------------------------");
    println!("Suponha que a e b sejam variáveis \n inteiras sem sinal de valores 0x6DB7 e 0xA726\n Qual o valor de a ^ b");
    println!("\n");
    let c: u32 = a^b;
    println!(" a = {:032b } = {:0x} ", a, a);
    println!(" b = {:032b } = {:0x} ", b, b);
    println!(" c = {:032b } = {:0x} ", c, c);
    println!("\n");
    println!("----------------------------------------------------------------------------");
    println!("Suponha que a e b sejam variáveis \n inteiras sem sinal de valores 0x6DB7 e 0xA726\n Qual o valor de a | b=0xEFB7");
    println!("\n");
    let c: u32 = a|b;
    println!(" a = {:032b } = {:0x} ", a, a);
    println!(" b = {:032b } = {:0x} ", b, b);
    println!(" c = {:032b } = {:0x} ", c, c);
    println!("\n");
    println!("----------------------------------------------------------------------");

   let mut x: i32= 1; //0000 0001
    let mut i: i32= 1; 

    println!(" Deslocamento de Bite a Esquerda ");
    println!("\n");


    println!(" x0 = (x << 0); // {:08b }",x);
  
    while i <= 7 {
    println!(" x{} = (x << {}); // {:08b }", i, i, x << i);
    i += 1;
   
   
    }
    println!("\n");
    println!("----------------------------------------------------------------------");

    let mut x: i32= 128; //1000 0000
    let mut i: i32= 1;
     println!("\n");
    println!(" Deslocamento de Bite a Direita ");
    println!("");

    println!(" x0 = (x << 0); // {:08b }", x);
    while i <= 7 {
    println!(" x{} = (x << {}); // {:08b }", i, i, x >> i);
    i += 1;
   
    }
    println!("\n");
    println!("----------------------------------------------------------------------");
    println!(" \nOperadores de deslocamento ");
    println!("");

    println!(" Assumindo que a e b são short int \n -b = a << 6\n deslocará todos os bits de a seis posições à \nesquerda e o resultado será atribuida à variável b.\n -b = 0x6DC0\n\n Vamos ver o resultado final");
    println!("");
    let a=28087;
    let b= a << 6;
    println!(" a = {:16b } => {:X}", a, a);
    println!(" b = {:16b } => {:X}", b, b,);
    println!("\n");
    println!("----------------------------------------------------------------------");
    println!(" \nOperadores de deslocamento\n Exemplo:\n-b = a >> 6\n\nDeslocará todos os bits de a seis posições a direita e o resultado será atribuído à variavel b.\n -b = 0x1B6 \n\n Vamos ver o resultado final");
    println!("");
    let a=28087;
    let b= a >> 6;
    println!(" a = {:016b } => {:X}", a, a);
    println!(" b = {:016b } => {:X}", b, b,);
    

    
}