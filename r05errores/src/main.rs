use std::fs::File;

fn main() {
    println!("Hello, world!");
    // errores recuperabes y no recuperrables
    // mno existen exceptions
    // result<T, E> y panic
    
    // panic!("Explota"); se rompe si o si
    let archivo = File::open("errorpath");
    match archivo {
        Ok(file) => leer_archivo(file),
        Err(error) => println!("No pude abrir el archivo"),
    }

    // uso de assert
}

fn leer_archivo(file: File) {

}

fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn sumar_bien() {
    assert_eq!(sumar(2, 2), 4);
}
