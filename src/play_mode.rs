use std::io::stdin;
use crate::define;

pub fn start() {

    let mut score: u32 = 0;

    let mut filename = String::new();

    let _ = stdin().read_line(&mut filename);

    match define::import(filename.trim()) {
        Ok(temp) => { 
            let mut index = 1;
            for question in temp.questions {
                    
                println!("{})  {}   (score: +{})", index, &question.question, &question.point);
                
                let mut user_answer = String::new();
                let _ = stdin().read_line(&mut user_answer);

                if user_answer.trim() == &question.answer {
                    score += question.point;
                }

                println!("\n\n");
                index += 1;

            }
            println!("--------=+< Result >+=--------");
            println!("Your Score: {}", score);

        },
        Err(e) => { println!("{}", e) }
    }
}