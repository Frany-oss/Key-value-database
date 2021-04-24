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

}
