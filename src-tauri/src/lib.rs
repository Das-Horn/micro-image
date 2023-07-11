
use std::fs;

pub enum convTypes {
    RGB8 = 0,
}

pub fn create_hex_string_array(img: Vec<u8>) -> String {
    // Read the scaffold in and split it to create the array
    let scaf = read_scafhold();
    let mut scaf_arr = scaf.split("|");

    let mut data_form_arr = String::new();

    data_form_arr += scaf_arr.next().unwrap();

    let mut placeholder = String::from("");

    for pixel in img {
        placeholder = String::from("0x");
        placeholder += &format!("{:X}", pixel);
        data_form_arr += &placeholder;
        data_form_arr += ", ";
    }

    data_form_arr += scaf_arr.next().unwrap();

    data_form_arr
}

fn read_scafhold() -> String {
    fs::read_to_string("scaffold.txt")
        .expect("Error reading scaffold file")
}