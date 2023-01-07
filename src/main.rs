use std::io;

fn main() {
    println!("\nConversor de temperatura\n");

    loop {
        println!("Converter para:");
        println!("Celcius (C)       Fahrenheit (F)\n");

        let mut escala: String = String::new();

        io::stdin()
            .read_line(&mut escala)
            .expect("Falha ao ler entrada");

        let escala: u8 = match escala.trim() {
            "C" => 1,
            "F" => 2,
            _ => continue,
        };

        println!("\nInforme uma temperatura\n");

        let mut temperatura: String = String::new();

        io::stdin()
            .read_line(&mut temperatura)
            .expect("Falha ao ler entrada");

        let temperatura: f64 = match temperatura.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if escala == 1 {
            converter_para_celcius(temperatura);
        } else {
            converter_para_fahr(temperatura);
        }
    }
}

// Formula  (F − 32) × 5/9 = °C
fn converter_para_celcius(temperatura: f64) {
    let temperatura_convertida: f64 = (temperatura - 32.0) * 5.0 / 9.0;

    println!(
        "\n{} °F para celsius é: {} °C\n\n",
        temperatura, temperatura_convertida
    );
}

// Formula (C × 9/5) + 32 = °F
fn converter_para_fahr(temperatura: f64) {
    let temperatura_convertida: f64 = (temperatura * 9.0 / 5.0) + 32.0;

    println!(
        "\n{} °C para Fahrenheit é: {} °F\n\n",
        temperatura, temperatura_convertida
    );
}
