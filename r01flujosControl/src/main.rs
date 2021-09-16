
fn main() {
    let number = 5;

    if number == 5 {
        println!("es cinco");
    } else if number == 3 {
        println!("es tres")
    } else {
        println!("no es ni cinco ni tres");
    }

    let resultado = if number >= 5 { 1000 } else { 0 };

    println!("Resultado: {}", resultado);

    let mut counter = 0;
    // Loop
    let result = loop {
        if counter == 10 {
            break counter;
        }
        counter += 1;
    };

    println!("Result: {}", result);

    // While
    while counter > 0 {
        println!("Counter: {}", counter);
        counter -= 1;
    }

    // for
    let arreglo = [0,1,2,3,4];

    for element in arreglo.iter() {
        println!("element: {}", element);
    }

    // if-let
    let edad: Option<i32> = Some(20);

    match edad {
        Some(value) => println!("edad: {}", value),
        _ => ()
    }

    if let Some(value) = edad {
        println!("edad: {}", value);
    }

    // While let
    let mut mensajes_no_leidos = Some(100);

    // loop {
    //     match mensajes_no_leidos {
    //         Some(value) => {
    //             if value > 0 {
    //                 println!("Tienes mensajes no leidos");
    //                 mensajes_no_leidos = Some(value - 1);
    //             } else {
    //                 println!("no hay mensjes nuevos");
    //                 mensajes_no_leidos = None;
    //             }
    //         }
    //         _ => { break; }
    //     }
    // }

    while let Some(value) = mensajes_no_leidos {
        if value > 0 {
            println!("Tienes mensajes no leidos");
            mensajes_no_leidos = Some(value - 1);
        } else {
            println!("no hay mensjes nuevos");
            mensajes_no_leidos = None;
        }
    }
}
