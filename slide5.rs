fn main() {
    println!("Operador complementar (32 bits)");
    println!("{:032b} = ~0x{:x}",0xC5,0xC5);
    println!("{:032b} = 0x{:x}\n",!0xC5,!0xC5);
    
    println!("{:032b} = ~0x{:x}",0x1111,0x1111);
    println!("{:032b} = 0x{:x}\n\n",!0x1111,!0x1111);
    
    println!("{:032b} = ~0x{:x}",0xFFFF,0xFFFF);
    println!("{:032b} = 0x{:x}\n\n",!0xFFFF,!0xFFFF);
    
    println!("{:032b} = ~0x{:x}",0x5B3C,0x5B3C);
    println!("{:032b} = 0x{:x}\n\n",!0x5B3C,!0x5B3C);
}