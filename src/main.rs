use std::io;

fn main() {

    let mut my_array: [i32; 3] = [0; 3];
    let mas = ["Вік", "Ріст", "Вага"];


    for mas in 0..3 {
        let mas1 = ["Вік", "Ріст", "Вага"];
        println!(" {:?}:", mas1);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");
        let value: i32 = input.trim().parse().expect("Не вдалося конвертувати в число");

        my_array[mas] = value;
    }

    if my_array[0] < 18{
        println!("Ваш вік {:?} Вибачте вам тут не місце допобачення ", my_array[0]);
        return;
    }

    println!("{:?}: {:?}", /*Вік*/mas[0], my_array[0]);
    println!("{:?}: {:?}", /*Ріст*/mas[1], my_array[1]);
    println!("{:?}: {:?}", /*Вага*/mas[2], my_array[2]);

    println!("тепер продовжимо наше тренування");
    
    
}