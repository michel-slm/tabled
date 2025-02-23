//! The example can be run by this command
//! `cargo run --example col_row_macros --features="macros"`

use tabled::{col, row, Alignment, Style, Table, Tabled};

#[derive(Tabled)]
struct Person {
    name: String,
    age: u8,
    is_validated: bool,
}

impl Person {
    fn new(name: &str, age: u8, is_validated: bool) -> Self {
        Self {
            name: name.into(),
            age,
            is_validated,
        }
    }
}

fn main() {
    let validated = [Person::new("Sam", 31, true), Person::new("Sarah", 26, true)];

    let not_validated = [
        Person::new("Jack Black", 51, false),
        Person::new("Michelle Goldstein", 44, true),
    ];

    let unsure = [
        Person::new("Jon Doe", 255, false),
        Person::new("Mark Nelson", 13, true),
        Person::new("Terminal Monitor", 0, false),
        Person::new("Adam Blend", 17, true),
    ];

    let table_a = Table::new(&validated).with(Style::ascii()).to_string();
    let table_b = Table::new(&not_validated).with(Style::modern()).to_string();
    let table_c = Table::new(&unsure).with(Style::ascii_rounded()).to_string();

    let row_t = row![table_c, table_b];
    let col_t = col![table_c; 3];
    let mut row_col_t = col![row![table_a, table_b].with(Style::empty()), table_c];
    row_col_t.with(Alignment::center());

    println!("{row_t}");
    println!();
    println!("{col_t}");
    println!();
    println!("{row_col_t}");
}
