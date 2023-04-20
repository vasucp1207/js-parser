const GLOBAL_VAR: i32 = 5;

fn main() {
    let mut x = 5;
    println!("val of x is {x}");
    x = 6;
    println!("val of x is {x}");

    println!("global var is {GLOBAL_VAR}");

    {
        let x = "10";
        println!("val of x in inner scope is {x}");
    }
    println!("val of x in outer scope is {x}");

    let condition = true;

    let number = if condition { "5" } else { "six" };
}
