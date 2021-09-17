use std::collections::HashMap;

fn main() {
    // Collections
    // vector (del mismo tipo de datos)
    // let v: Vec<i32> = Vec::new();
    algo();

    // string
    // f_string();
    
    // hashmap
    f_hashmap();
}

fn algo() {
    let mut v = vec![1,2,3];
    v.push(9);
    v.push(100);

    let tercer = v.get(1);

    match tercer {
        Some(valor) => println!("Valor elemento: {}", valor),
        None => ()
    }

    for i in &v {
        println!("valor: {}", i)
    }

    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("Valor: {}", i);
    }

    enum Mensaje {
        TEXTO(String),
        ERROR(i32),
    }

    let mensajes = vec![Mensaje::TEXTO("Holaaaa".to_string()), Mensaje::ERROR(404)];

    for m in &mensajes {
        match m {
            Mensaje::TEXTO(texto) => println!("valor: {}", texto),
            Mensaje::ERROR(codigo_err) => println!("valor {}", codigo_err),
        }
    }
}

// fn f_string() {
//     let mut nombre_String = String::from("kiubo");
//     let nombre_str = "kiubo";

//     let mut nombre2 = nombre_str.to_string();
//     nombre2.push('q');

//     let mut nombre_String_a_str = &nombre_String[..nombre_String.len()];

//     nombre_String.push('a');
//     println!("nombre: {}", nombre_String_a_str);
// }

fn f_hashmap() {
    let mut puntajes: HashMap<String, i32> = HashMap::new();
    puntajes.insert(String::from("Azul"), 20);
    puntajes.insert(String::from("Rojo"), 10);

    let ptos_azul = puntajes.get("Amarillo");

    match ptos_azul {
        Some(ptos) => println!("{}", ptos),
        None => println!("El equipo no existe"),
    }

    for (key, value) in &puntajes {
        println!("{} {}", key, value);
    }

    puntajes.insert(String::from("Azul"), 40);

    for (key, value) in &puntajes {
        println!("{} {}", key, value);
    }

    puntajes.entry(String::from("Azul")).or_insert(100);
    for (key, value) in &puntajes {
        println!("{} {}", key, value);
    }

    // Copy
    let ptos_ = 20;
    let verde = String::from("Verde");
    puntajes.insert(verde, ptos_);

    // println!("{} {}", verde, ptos_); No se puede usar por que verde se cambia de referencia
}
