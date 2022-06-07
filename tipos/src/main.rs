/* 
 * Tipos de datos en Rush
 * Por: 
 * Fecha:  
 *  
 * Este programa...
 *  
 */ 

fn main() {
    // Tipos de datos

    // i8, i16 i32, i64, i128 -> Con signo + -
    // u8, u16 u32, u64, u128 -> Sin signo + 
    
    let numero_uno: i8 = -10;
    let numero_dos: u8 = 10;

    // Char -> UTF-8
    let caracter = '😁';

    //  f32 o f64
    let real: f32 = 12.5;

    let resultado: bool = false; // true

    println!("{} {} {} {} {}", numero_uno, numero_dos, caracter, real, resultado);
}
