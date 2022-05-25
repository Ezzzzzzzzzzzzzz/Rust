// Importamos la libreria standar input
use std::io; 

fn main() {
   
    println!("Ingresa el nombre de usuario");

    let mut username = String::new(); // Static -> ""

    io::stdin().read_line(&mut username);  // Referencia y permiso de mutabilidad

    let username = username.trim(); // Metodo trim() elimina los saltos de linea
      
    println!("El valor de la veriable es: {}", username);


}
