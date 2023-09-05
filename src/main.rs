/*

Comments!!

// This is similar to java comments.. block and single line.

/// Doc comments are parsed into HTML library documentation. Generate library documents .. Ends with //! 

This is my very first rust program.

single quotation is for characters

double quotation is for strings

*/


pub mod songs {
    pub fn play(name: String) {
        println!("track selection: {}",name)
    }
}

use songs::play;

pub mod tracks {
    pub mod rock {
        pub mod indie {
            pub fn select (x:String) {
                println!("The selected song is here.. {}",x);
            }
        }
    }
}

use tracks::rock::indie::select;

use std::collections::HashMap;

use std::fs::File;

use std::io::{Error, Read, BufReader};

fn main() {
    // Data types: what type of value a variable has
    // like ints , strings, chars , arrays
    
    // Rust is a statically typed language. Every value has a data type.
    // the compiler automatically infers the data type of a veriable based on the value assigned to it.
    
    // The type system verifies the supported values, before they are stored or manipulated.
    
    // How do we declare variables..
    
    play("Kissed by a rose".to_string());
    select("Serenade".to_string());
    let switch = false;
    let volume = 10;
    
    // float type takes decimal places but not int
    
    println!("Switch {switch} {volume}");
    
    eprintln!("{x}, {y}", x = "hello" , y = "my friend");
    
    //integers in rust numbers without decimal / fractionalized.. 
    
    // there can be signed and unsigned.. 
    
    // unsigned will only store positive.
    
    let v:i64 = 1 ; let x:i64 = 4; //i32 by default
    
    println!("{}",v + x); println!("{}",v - x);
    
    
    let i:i64 = -24324234;
    
    println!("{i}");
    
    // exercise : 
    
    let mut j:u16 = 65535;
    println!("{j}");
    // let overtime_1:u16 = 65536;
    // let overtime_2:u16 = 65537;
    
    j = 1500;
    println!("{j}");
    // println!("{overtime_1} {overtime_2}");
    
    // String literals are found in module std::str
    
    let bank:&'static str = "Citi Bank";
    let currency:&str = "Bitcoin";
    
    // string literal is &str type
    println!("{bank}{currency}");
    
    let great_movie = String::from("The Big Panda");  
    println!("{}",great_movie.len());
    
    // new() in string. to_string()
    
    // replace() as_str() push() push_str() len() 
    
    let mut greeting = String::from("Hello World!");
    
    greeting.push_str("Hi How are you");
    
    println!("{greeting}");
    
    
    let mut password:String = "pokemon,".to_string();
    password.push_str(" gotta catch them all!");
    println!("{}",password);
    
    /*
    arithmetic : +, - , * , /, % 
    bitwise : << , >>  
    comparison : < , > , != , == , <= , >=  
    */
    

    
    let microbiome = "xc12";
    let body_part = match microbiome {
        "xc12" => { println!("This is xc12, it has a microbiome"); "Tummy Biome" },
        "mp1a" => "Eyebiome",
        _ => "Unknown"
    };
    println!("{body_part}");
    
    
    let x:i32 = 3;
    let y:i32 = 4;
    if x < y || x > 6 {
        println!("x is less than y");
        println!("x is greater than 6");
    } else {
        println!("Try again");
    }
    
    // loops in rust
    
    // while
    let mut i = 0;
    while i < 15 {
        i+=1;
        if i == 3 {
            continue;
        }
        if i == 12 {
            break;
        }
        println!("i:{i}");
    }
    
    // for 
    for j in 10..15 {
        println!("j:{j}");
    }
    
    loop {
        println!("asdf");
        break;
    }
    
    let sdf = lads(10,15);
    println!("{sdf}");
    
    // tuples in rust
    let tuple:(u32, u32, u32) = (2,5,9);
    // tuples have a fixed length, index starting from 0..
    // is tuples an array??
    println!("{:?}",tuple.0) ;
    
    let mut arr:[u32;7] = [ 0 , 1 , 2 , 3 , 5 , 0 , 54 ];
    println!("{:?}",arr);
    println!("total length of arr {}", arr.len()); 
    
    arr[3] = 54; 
    println!("{:?}",arr);
    
    for value in arr.iter() {
    println!("value is {value}");
    }
    
    // excited for vectors in array now. 
    
    let admin:User =  User { name: "Vibhas".to_string(),age: 27 , hat: Hats::SoftHands }; 
    println!("{:?}",admin.age);
    println!("{:?}",admin.name);
    println!("{:?}",admin.ageplus5plusx(5));
    
    let admin2:User =  User { name: "Sakshi".to_string(),age: 27 , hat: Hats::Leather }; 
    println!("{:?}",admin2.age);
    println!("{:?}",admin2.name);
    println!("{:?}",admin2.ageplus5plusx(5));
    
    
    let vector1 = vec![0 , 34 , 34 ,34343 , 34];
    // vector vector1 owns the object in the heap.
    display(&vector1);
    
    let mut car:String = String::from("Ferrari");
    display2(&mut car);
    println!("{}",car);
    
    let slice = &great_movie[1..4];
    println!("{:?}",slice);
    
    match admin.hat {
        Hats::SoftHands => { println!("SoftHands!!"); },
        Hats::HardHats => { println!("HardHats"); },
        _ => { println!("not softhands :("); }
    }
    
    match admin2.hat {
        Hats::SoftHands => { println!("SoftHands!!"); },
        Hats::HardHats => { println!("HardHats"); },
        _ => { println!("not softhands :("); }
    }
    
    // insert() , get() , len() , remove() 
    let mut m:HashMap<String,String> = HashMap::new();
    m.insert("John".to_string(),"GOod".to_string());
    
    println!("{:?}",m);
    
    let solution = is_seven(7);
    println!("{:?}",solution);
    
    let t1:GenericStruct<i32> = GenericStruct{value:100};
    println!("{}",t1.value);
    // panic!("This will cause the program to abruptly end");
    let game1 = Game {power_level: 32, weapon: "Axe" };
    println!("{:?}",game1.character_stats());
    
    let file = "cat.txt";
    
    match File::create(file) {
        Ok(filecb) => println!("{:?}",filecb),
        Err(_) => println!("Unable to create file")
    }
    
    let f = File::open("cat.txt").expect("Unable to open the file");
    println!("{:?}",f);
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    
    reader.read_to_string(&mut contents);
    
    println!("{:?}",contents);
    
}

fn is_seven(x:i32) -> Result<bool,String> {
    if x == 7 {
        return Ok(true);
    } else {
        return Err("It is not seven".to_string());
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    hat: Hats,
}

impl User {
    fn ageplus5plusx(&self, x:i32) -> i32 {
        return &self.age + x + 5
    }
}

struct GenericStruct<T> {
    value: T,
}

trait Stats {
    fn character_stats(&self);
}

struct Game {
    weapon:&'static str,
    power_level:u32
}

impl Stats for Game {
    fn character_stats(&self) {
        println!(
            "Printing power level: {} and weapon: {} ",
            self.power_level,
            self.weapon
        );
    }
}

#[derive(Debug)]
enum Hats {
    SoftHands,
    HardHats,
    Leather
}


fn display(x:&Vec<i32>) {
    println!("{:?}",x);
}

fn display2(s:&mut String) {
    s.push_str("F8 Tributo");
    println!("{:?}",s);
}

fn lads(x:i32, y:i32) -> i32 {
    x + y + 5
}

