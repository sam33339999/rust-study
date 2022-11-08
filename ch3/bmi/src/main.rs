use std::io;

fn main() {

    let mut guess = String::new();

    println!(" 請輸入身高(單位_公分_cm): ");
    io::stdin().read_line(&mut guess).expect("msg");
    let h: f32 = guess.trim().parse().expect("error input h");

    guess =  "".to_string();

    println!(" 請輸入體重(單位_公斤_kg): ");
    io::stdin().read_line(&mut guess).expect("msg");
    let w: f32 = guess.trim().parse().expect("error input w");


    let bmi = w / ((h/100.0) * (h/100.0));
    
    // 為什麼下面這句運算很奇怪
    // let bmi = w / (h/100.0) * (h/100.0);

    println!("您輸入的身高為: {h} 體重為: {w}");
    
    let score_msg = if bmi < 18.5 {
        "過輕"  
    } else if bmi >= 24.0 && bmi < 27.0 {
        "過重"
    } else if bmi >= 27.0 && bmi < 30.0 {
        "輕度肥胖"
    } else if bmi >= 30.0 && bmi < 35.0 {
        "中度肥胖"
    } else if bmi >= 35.0 {
        "重度肥胖"
    } else {
        "正常"
    };

    println!("計算出來的 bmi 為: {bmi} , 評分為: {score_msg}");

}
