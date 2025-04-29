enum Color{
    Red,
    Green,
    Blue,
    Yellow,
}

fn print_color(my_color: Color){
    match my_color{
        Color::Red=>println!("this is red"),
        Color::Green=>println!("this is green"),

        Color::Blue=>println!("this is blue"),
        _=>println!("this are other colors"),

    }
}

fn main(){
    print_color(Color::Yellow);
}