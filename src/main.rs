use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    println!("Игра угадай число!");

    let secret_num= rand::thread_rng().gen_range(1..=100);
    // println!("Секретное число: {secret_num}");
    loop {
        println!("Пожалуйста, введите ваше предположение!");
        let mut guess = String::new();//ассоциативная функция для создания пустой строки
        io::stdin().read_line(&mut guess).expect("Ошибка на чтении числа");
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue
        };

        println!("Вы предположили: {guess}");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Меньше"),
            Ordering::Greater => println!("Больше"),
            Ordering::Equal => {
                println!("Ты угадал");
                break;
            },

        }
    }
}
