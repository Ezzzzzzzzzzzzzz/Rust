fn main() {
                // 0  1  2  3  4
    let numeros = [1, 2, 3, 4, 5]; // Size -> 5
    let mut numeros: [i32; 6] = [1, 2, 3, 4, 5, 6];

    println!("El valor del arreglo es: {:?}", numeros);

    let valores = [5.5; 10];
    println!("El valor del arreglo valores es: {:?}", valores);

    let primer_elemento = valores[0];
    println!("El elemento es: {}", primer_elemento);

    let ultimo_elemento = numeros[numeros.len() - 1];

    numeros[2] = 30; 

    println!("El ultimo elemento de Numeros es: {}", ultimo_elemento);
    println!("El valor del arreglo es: {:?}", numeros);
}
