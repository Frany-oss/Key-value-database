
use std::collections::HashMap;

fn main() {
    
    // to read in the comand line what we pass 
    let mut arguments = std::env::args().skip(1); // We skip the first one, and start from 1
    let key = arguments.next().expect("Key was not there"); // if there is no string, then crash the program
    let value= arguments.next().expect("value was not there"); // if there is no string, then crash the program

    println!("The key is {} and the value is {}", key, value); // printing the key and the value
    
    // file will look like 
    // mykey \t myvalue \n mykey2 \t myvalue2

    // the key value pare to store in the database and then wirte it in a file
    let contents = format!("|{}|\t|{}|\n", key, value); // format returns a string
    std::fs::write("kv.db", contents).unwrap(); // storing the pair on a separete file 

    let database = Database::new().expect("Creating Database failed!");

}

struct Database {
    map: HashMap<String, String>, // hashMap for key-value storage

}

impl Database {

    fn new() -> Result<Database, std::io::Error> {

        // hashMap that let us store pairs 
        let mut  map = HashMap::new();

        // read the kv.db file ----------
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Result::Err(error);
        //     }
        // };
        let contents = std::fs::read_to_string("kv.db")?; // the ? is to handle errors (like above)

        for line in contents.lines() {

            let mut chunks = line.splitn(2, '\t'); // spliting into 2 chuncks

            let key = chunks.next().expect("No key!"); // key is the fisrt chunk of the variable
            let value = chunks.next().expect("No value!"); // value is the second chunk of the variable

            map.insert(key.to_owned(), value.to_owned()); // .to_owned() Creates owned data from borrowed data, usually by cloning
        }

        // parse the string
        
        // populate the file
        Ok(Database { map: map })
    }

}

//TODO: see if the file exists, if not then create it. use std::path::exist
