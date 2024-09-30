use std::io::{self, BufReader, BufRead, Write};
use std::fs::File;

struct Car {
    make: String,
    model: String,
    year: u32
}

fn main() {
    let mut buffer = String::new();

    print!("What's your car's make? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();
    buffer.clear();

    print!("What's your {}'s model? ", make);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("What year is your {} {}? ", make, model);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();
    buffer.clear();

    let car = Car { make, model, year };

    let mut file = File::create("user_info.txt").unwrap();
    writeln!(file, "{}\n{}\n{}", car.make, car.model, car.year).unwrap();

    let file = File::open("user_info.txt").unwrap();
    let mut reader = BufReader::new(file);

    reader.read_line(&mut buffer).unwrap();
    // buffer.pop();    #Another way to get rid of '\n' in the buffer
    // let file_make = buffer.clone();  #'.clone() is necessary if not using 'to_string()'
    let file_make = buffer.trim().to_string();
    buffer.clear();

    reader.read_line(&mut buffer).unwrap();
    let file_model = buffer.trim().to_string();
    buffer.clear();

    reader.read_line(&mut buffer).unwrap();
    let file_year = buffer.trim().parse().unwrap();
    buffer.clear();
    
    let file_car = Car {
        make:   file_make,
        model:  file_model,
        year:   file_year
    };

    println!("Your car is a {} {} {}", file_car.year, file_car.make, file_car.model);

    
}