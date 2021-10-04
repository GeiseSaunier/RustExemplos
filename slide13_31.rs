fn main() {
    let a = 0x6DB7;
  
    println!("Máscara");
    let mut m = 0x3F;
    println!("b = a & 0x3F");
    println!("a    = {:016b} = 0x{:x}", a,a);
    println!("M    = {:016b} = 0x{:x}", m,m);
    let mut b = a & m;
    println!("b    = {:016b} = 0x{:x}\n", b,b);
   
    
    println!("b = a & 0xFC00");
    println!("a    = {:016b} = 0x{:x}", a,a);
    m = 0xFC00;
    println!("M    = {:016b} = 0x{:x}", m,m);
    b = a & m;
    println!("b    = {:016b} = 0x{:x}\n", b,b);
    
    
    println!("b = a | 0xFF");
    println!("a    = {:016b} = 0x{:x}", a,a);
    m = 0xFF;
    println!("M    = {:016b} = 0x{:x}", m,m);
    b = a | m;
    println!("b    = {:016b} = 0x{:x}\n", b,b);
    
    
    println!("b = a | 0xFF00");
    println!("a    = {:016b} = 0x{:x}", a,a);
    m = 0xFF00;
    println!("M    = {:016b} = 0x{:x}", m,m);
    b = a | m;
    println!("b    = {:016b} = 0x{:x}\n", b,b);
    
    
    println!("b = a ^ 0xFF");
    println!("a    = {:016b} = 0x{:x}", a,a);
    m = 0xFF;
    println!("M    = {:016b} = 0x{:x}", m,m);
    b = a ^ m;
    println!("b    = {:016b} = 0x{:x}\n", b,b);
    
    
    println!("b = a ^ 0xFF00");
    println!("a    = {:016b} = 0x{:x}", a,a);
    m = 0xFF00;
    println!("M    = {:016b} = 0x{:x}", m,m);
    b = a ^ m;
    println!("b    = {:016b} = 0x{:x}\n", b,b);
    
    println!("b = a ^ 0x4");
    println!("a    = {:016b} = 0x{:x}", a,a);
    m = 0x4;
    println!("M    = {:016b} = 0x{:x}", m,m);
    b = a ^ m;
    println!("b    = {:016b} = 0x{:x}\n", b,b);
    
    b = 0xAF;
    let mut m1 = 0xFC;
    let mut m2 = 0x02;
    
    println!("B = ((B&M1)|M2)\nOnde M1=11111100 e M2=00000010");
    println!("B           = {:08b } = 0x{:x}", b,b);
    println!("M1          = {:08b } = 0x{:x}", m1,m1);
    m = b&m1;
    b = m|m2;
    println!("B&M1        = {:08b} = 0x{:x}", b&m1,b&m1);
    println!("M2          = {:08b} = 0x{:x}", m2,m2);
    println!("((B&M1)|M2) = {:08b} = 0x{:x}\n", b,b);
    
    b = 0xAF;
    m1 = 0xE3;
    m2 = 0x14;
    
    println!("B = ((B&M1)|M2)\nOnde M1=11100011 e M2=00010100");
    println!("B           = {:08b } = 0x{:x}", b,b);
    println!("M1          = {:08b } = 0x{:x}", m1,m1);
    m = b&m1;
    b = m|m2;
    println!("B&M1        = {:08b} = 0x{:x}", b&m1,b&m1);
    println!("M2          = {:08b} = 0x{:x}", m2,m2);
    println!("((B&M1)|M2) = {:08b} = 0x{:x}\n", b,b);
    
    
    
}