
#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    
    // to read in the comand line what we pass 
    let mut arguments = std::env::args().skip(1); // We skip the first one, and start from 1
    let key = arguments.next().expect("Key was not there"); // if there is no string, then crash the program
    let value= arguments.next().expect("value was not there"); // if there is no string, then crash the program

    println!("The key is {} and the value is {}", key, value); // printing the key and the value

    let mut database = Database::new().expect("Creating Database failed!");

    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);

    match database.flush() {

        Ok(()) => println!("Flush executed"),
        Err(err) => println!("Flush couldn't execute"),
    }

}

struct Database {
    map: HashMap<String, String>, // hashMap for key-value storage
    flush: bool

}

impl Database {

    fn new() -> Result<Database, std::io::Error> {

        // hashMap that let us store pairs 
        let mut  map = HashMap::new();
        // read the kv.db file 
        let contents = std::fs::read_to_string("kv.db")?; // the ? is to handle errors

        for line in contents.lines() {

            let mut chunks = line.splitn(2, '\t'); // spliting into 2 chuncks

            let key = chunks.next().expect("No key!"); // key is the fisrt chunk of the variable
            let value = chunks.next().expect("No value!"); // value is the second chunk of the variable

            map.insert(key.to_owned(), value.to_owned()); // .to_owned() Creates owned data from borrowed data, usually by cloning
        }

        // parse the string
        
        // populate the file
        Ok(Database { 
            map,
            flush: false,
        })
    }

    fn insert(&mut self, key: String, value: String) {

        self.map.insert(key, value);

    }

    // Funtion when we want to handle and output the error
    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

// Function when we do not want to handle errors of the write function
impl Drop for Database {
    fn drop(&mut self) {
        
        if !self.flush {
            let _ = do_flush(self); // when you don't need the result 
        }
    }
}

// function to write the values in the map to the kv.db file 
fn do_flush(database: &Database) -> std::io::Result<()> {

    let mut contents = String::new();

        // pushing the pair into the file kv.db 
        for (key, value) in &database.map {

            // let kvpair = format!("{}\t{}", key, value);
            // contents.push_str(&kvpair);

            // better this way for memory managment
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }

        std::fs::write("kv.db", contents) 


}
