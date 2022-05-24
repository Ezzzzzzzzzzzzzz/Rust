fn main() {

// let <nombre_variable> = <El valor>
// let <nombre_variable>: <T> = <El valor>
// let mut <nombre_variable>: <T> = <El valor>

/*
    Comentarios con salto de linea    
*/

// Definicion de una constante (Constante no es lo mismo que una variable inmutable, las constantes NUNCA serán MUTABLES)
const VALOR: i32 = 10; 

let mut numero_uno = 10; // Definicion de una variable mutable (puede cambiar su valor en tiempo de ejecución)
let numero_dos: i32 = 15; // Definicion de una variable, entero de 32 bits

numero_uno = 100;

let resultado = numero_uno + numero_dos + VALOR;

println!("El resultado es: ({} + {} + {}): {}", numero_uno, numero_dos, VALOR, resultado);

}
