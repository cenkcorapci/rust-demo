mod simple_mod;
mod a_folder;
#[path = "a_folder/module_under_folder.rs"] mod module_under_folder;
mod my;
mod functions_and_shit;

use std::fs::File;
use std::io::ErrorKind;

fn add(a: i32, b: i32) -> i32 {
    let x = 0;
    return a + b + x;
}

fn main() {
    functions_and_shit::function_calls_another_function_from_that_other_module();
    my::nested::function();
    simple_mod::some_func();
    module_under_folder::another_func();
    println!("{} + {} = {}", 1, 2, add(1, 2));
    let s = "this is string".to_string();
    println!("{}", s);
    let slice: &str = &s;
    println!("{}", slice);
    let fixed_size_array: [i32; 4] = [1, 2, 3, 4];
    let mut vec: Vec<i32> = vec![1, 2, 3, 4];
    vec.push(1);
    println!("{:?} {:?}", vec, slice); // ? debug style

    let mut freezable: i32 = 2;
    {
        let freezable = 3;
    }
    freezable = 4;

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
