// Importamos la libreria standar input
use std::io; 

fn main() {
   
    println!("Ingresa el nombre de usuario");

    let mut username = String::new(); // Static -> ""

    // Result -> Exito o Error
    io::stdin().read_line(&mut username); // Referencia y permiso de mutabilidad

    let username = username.trim(); // Metodo trim() elimina los saltos de linea

    println!("Ingresa la edad del usuario");

    let mut edad = String::new(); 

    io::stdin().read_line(&mut edad);

    let edad = edad.trim();

    // Result
    let edad: i32 = edad.parse().unwrap(); // parse str -> numero | unwrap -> el valor ya convertido
      
    println!("Hola {} con edad {}", username, edad);


}
