
extern crate rand;
extern crate clap;

use rand::os::OsRng;
use rand::Rng;
use clap::{Arg, App};

use std::char;

const DEFAULT_LENGTH: usize = 12;

fn main() {

    let mut excluded = vec![CharType::Invalid];

    let matches = App::new("password-generator-rs")
        .version("1.0")
        .author("Jonathan Fok kan <jfokkan@gmail.com>")
        .about("Generates a random password from the OS source of randomness. If all character \
                types are excluded, the command will not return.")
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .value_name("Length of password")
            .help("Sets the length of password. (Default 16)")
            .takes_value(true))
        .arg(Arg::with_name("exclude")
            .short("e")
            .long("exclude")
            .value_name("Excluded Characters")
            .help("Sets characters to be excluded")
            .takes_value(true))
        .arg(Arg::with_name("number")
            .long("number")
            .short("n")
            .help("Exclude numbers"))
        .arg(Arg::with_name("special")
            .long("special")
            .short("s")
            .help("Exclude special characters: !\"#$%&()*+,-./:;<=>?@[\\]^_`{|}~"))
        .arg(Arg::with_name("uppercase")
            .long("uppercase")
            .short("u")
            .help("Exclude upper case characters"))
        .arg(Arg::with_name("lowercase")
            .long("lowercase")
            .short("o")
            .help("Exclude lower case characters"))
        .get_matches();

    let excluded_chars = matches.value_of("exclude").unwrap_or("");
    println!("Excluded characters: {}", excluded_chars);

    let length = matches.value_of("length")
        .map(|x| x.parse::<usize>().unwrap_or(DEFAULT_LENGTH))
        .unwrap_or(DEFAULT_LENGTH);

    match matches.occurrences_of("number") {
        1 => excluded.push(CharType::Number),
        _ => {}
    }
    match matches.occurrences_of("special") {
        1 => excluded.push(CharType::Special),
        _ => {}
    }
    match matches.occurrences_of("uppercase") {
        1 => excluded.push(CharType::UpperCase),
        _ => {}
    }
    match matches.occurrences_of("lowercase") {
        1 => excluded.push(CharType::LowerCase),
        _ => {}
    }


    let password = generate_random_password(length, excluded);

    println!("{}", password);
}

fn generate_random_password(length: usize, excluded: Vec<CharType>) -> String {

    let mut os_rng = OsRng::new().unwrap();
    let rand_gen = os_rng.gen_iter::<u32>();

    let rand_nums = rand_gen.map(|x| (x % 95) + 32)
        .filter(|x| !excluded.contains(&CharType::to_type(*x)));
    rand_nums.take(length).map(|x| char::from_u32(x).unwrap()).collect::<String>()
}

#[derive(Eq,PartialEq)]
enum CharType {
    Number,
    Special,
    UpperCase,
    LowerCase,
    NonVisible,
    Invalid,
}

impl CharType {
    fn to_type(num: u32) -> CharType {
        match num {
            0...31 => CharType::NonVisible,
            32...47 => CharType::Special,
            48...57 => CharType::Number,
            58...64 => CharType::Special,
            65...90 => CharType::UpperCase,
            91...96 => CharType::Special,
            97...122 => CharType::LowerCase,
            123...126 => CharType::Special,
            _ => CharType::Invalid,
        }
    }
}
