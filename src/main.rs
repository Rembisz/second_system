use std::io::{self, Write};

struct MetricDuration {
    giga: u64,
    mega: u64,
    kilo: u64,
    hecto: u64,
    deca: u64,
    base: u64,
}

fn scale_metric(unit: u64) -> MetricDuration {
    let mut count = 1;
    let mut conversion = MetricDuration {
        giga: 0,
        mega: 0,
        kilo: 0,
        hecto: 0,
        deca: 0,
        base: 0,
    };
    let mut remainder = unit;
    while count != 7 {
        match count {
            1 => {
                conversion.giga = remainder / 1_000_000_000; // giga
                remainder -= conversion.giga * 1_000_000_000;
            }
            2 => {
                conversion.mega = remainder / 1_000_000; // mega
                remainder -= conversion.mega * 1_000_000;
            }
            3 => {
                conversion.kilo = remainder / 1_000; // kilo
                remainder -= conversion.kilo * 1_000;
            }
            4 => {
                conversion.hecto = remainder / 100; // hecto
                remainder -= conversion.hecto * 100;
            }
            5 => {
                conversion.deca = remainder / 10; // deca
                remainder -= conversion.deca * 10;
            }
            6 => conversion.base = remainder, // base
            _ => println!("Conversion failed."),
        }
        count += 1;
    }
    conversion
}

fn time_second(data: &Vec<u64>, mut valid: bool) -> (u64, bool) {
    let mut seconds: u64 = 0;
    let mut count: u8 = 0;
    for index in data {
        match count {
            0 => match index {
                0 => match data[1] {
                    1 => seconds += 0,          // January
                    2 => seconds += 2_678_400,  // February
                    3 => seconds += 5_097_600,  // March
                    4 => seconds += 7_776_000,  // April
                    5 => seconds += 10_368_000, // May
                    6 => seconds += 13_046_400, // June
                    7 => seconds += 15_638_400, // July
                    8 => seconds += 18_316_800, // August
                    9 => seconds += 20_995_200, // September
                    _ => {
                        if valid {
                            println!("Month invalid");
                            valid = false;
                        }
                    }
                },
                1 => match data[1] {
                    0 => seconds += 23_587_200, // October
                    1 => seconds += 26_265_600, // November
                    2 => seconds += 28_857_600, // December
                    _ => {
                        if valid {
                            println!("Month invalid.");
                            valid = false;
                        }
                    }
                },
                _ => {
                    if valid {
                        println!("Month invalid.");
                        valid = false;
                    }
                }
            },
            2 => seconds += (index * 10) * 86_400, // day x10
            3 => seconds += index * 86_400,        // day
            4 => seconds += (index * 1000) * 31_536_000, // year x1000
            5 => seconds += (index * 100) * 31_536_000, // year x100
            6 => seconds += (index * 10) * 31_536_000, // year x10
            7 => seconds += index * 31_536_000,    // year
            8 => seconds += (index * 10) * 3600,   // hour x10
            9 => seconds += index * 3600,          // hour
            10 => seconds += (index * 10) * 60,    // minute x10
            11 => seconds += index * 60,           // minute
            12 => seconds += index * 10,           // second x10
            13 => seconds += index,                // second
            _ => (),
        }
        count += 1;
    }
    (seconds, valid)
}

fn get_data_vec(instance: &str) -> Vec<u64> {
    let mut data = String::new();
    let mut data_vec: Vec<u64> = Vec::new();
    loop {
        if instance == "start" {
            println!("Start:");
        } else if instance == "end" {
            println!("End:");
        }
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read entry.");

        let chars = data.trim().chars();
        let mut valid = false;
        let mut format_check = 0;
        for item in chars {
            match item {
                '/' => (),
                _ => match item.to_digit(10) {
                    Some(num) => {
                        data_vec.push(num as u64);
                        valid = true;
                        format_check += 1;
                    }
                    None => valid = false,
                },
            }
        }
        if valid && format_check == 14 {
            break;
        }
        println!("That is not a valid date.");
        data.clear();
        data_vec.clear();
    }
    data_vec
}

fn main() {
    println!(
        "Welcome to the Second System Conversion tool.\nPlease enter a starting date and an ending date in the following format:\nmm/dd/yyyy/hh/mm/ss"
    );

    loop {
        let input_data = (get_data_vec("start"), get_data_vec("end"));
        let start_seconds = time_second(&input_data.0, true);
        let end_seconds: (u64, bool) = if !start_seconds.1 {
            time_second(&input_data.1, false)
        } else {
            time_second(&input_data.1, true)
        };

        let mut no = false;

        if start_seconds.0 > end_seconds.0 {
            println!("Ending date must be later than starting date.")
        } else {
            let time = scale_metric(end_seconds.0 - start_seconds.0);
            println!(
                "The duration is as follows:\n{} gigaseconds\n{} megaseconds\n{} kiloseconds\n{} hectoseconds\n{} decaseconds\n{} seconds",
                time.giga, time.mega, time.kilo, time.hecto, time.deca, time.base
            );

            let mut again_choice = String::new();
            println!("Convert another duration?");
            println!("(Enter y or n):");
            loop {
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut again_choice)
                    .expect("Failed to read entry.");
                again_choice = again_choice.trim().to_ascii_lowercase();
                match again_choice.as_ref() {
                    "y" => {
                        println!("Format: mm/dd/yyyy/hh/mm/ss");
                        break;
                    }
                    "n" => {
                        no = true;
                        break;
                    }
                    _ => println!("Invalid."),
                }
            }
        }
        if no {
            break;
        }
    }
}
