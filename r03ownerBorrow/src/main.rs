fn main() {
    let mut edad = 20;
    aumentar_edad(&mut edad);
    println!("{}", edad);

    let mut nombre = String::from("neimv");
    // enviar_nombre(nombre.clone()); // muy costoso usar clone()
    enviar_nombre(&mut nombre); // muy costoso usar clone()
    println!("{}", nombre);
}

fn enviar_nombre(nombre: &mut String) {
    // agregar fecha
    nombre.push_str("20210621");
    println!("{}", nombre);
}

fn aumentar_edad(edad_copia: &mut i32) {
    *edad_copia += 1;
}
