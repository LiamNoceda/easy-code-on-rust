fn check(height: i32, age: i32) -> bool {
    height > 140 && age > 12
}

fn main() {
    let user_hei = 168;
    let user_ag = 23;

    let resulte = check(user_hei, user_ag);

    if resulte == true {
        println!(
            "Данный результат соответствует минимальным требованиям {}: 140 и 12 соответственно",
            resulte
        );
    } else {
        println!(
            "Данный результат не соответствует требованиям {}: 140 и 12 соответственно",
            resulte
        );
    }
}
