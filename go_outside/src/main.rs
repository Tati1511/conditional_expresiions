fn can_go_outside(is_raining: bool, temp: i32) -> bool {
    !is_raining || temp >= 10
}

fn tell_kids(is_raining: bool, temp: i32) {
    let msg: &str = if can_go_outside(is_raining, temp) {
        "You can go outside"
    } else {
        "Sorry, it's too cold and it's raining"
    };

    println!("{}", msg);
}

fn main() {
    tell_kids(true, 9);
    tell_kids(true, 10);
    tell_kids(false, 9);
    tell_kids(false, 10);
}