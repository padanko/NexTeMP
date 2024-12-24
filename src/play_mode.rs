use std::io::stdin;
use crate::define;

pub fn start() {

    let mut score: u32 = 0;

    let mut filename = String::new();

    let _ = stdin().read_line(&mut filename);

    match define::import(filename.trim()) {
        Ok(temp) => { 
            let mut index = 1;
            let mut vertics: Vec<bool> = Vec::new();

            for question in temp.questions {
                    
                println!("{})  {}   (score: +{})", index, &question.question, &question.point);
                
                let mut user_answer = String::new();
                let _ = stdin().read_line(&mut user_answer);

                let correct_answer = user_answer.trim() == &question.answer;
                
                vertics.push(correct_answer);
                if correct_answer {
                    score += question.point;
                }

                println!("\n");
                index += 1;

            }
            println!("--------=+< Result >+=--------");
            println!("Your Score: {}\n", score);
            for (i, &correct) in vertics.iter().enumerate() {
                if correct {
                    println!("{} O", i + 1);
                } else {
                    println!("{} X", i + 1);
                }
            }
            println!("( O = correct X = incorrect )");
            
        },
        Err(e) => { println!("{}", e) }
    }
}