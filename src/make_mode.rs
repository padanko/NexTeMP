use serde_json;
use regex;
use std::{fs, io::{stdin, stdout, Write}, str::FromStr};
use std::sync::{Arc,Mutex};
use std::collections::HashMap;

use crate::define;

fn export(question: define::TeMP) -> Result<(), String> {
    match serde_json::to_string(&question) {
        Ok(json_data) => {
            match fs::File::create(&format!("{}.json", question.title)) {
                Ok(mut file) => {
                    let _ = file.write_all(json_data.as_bytes());
                    Ok(())
                }, 
                Err(_) => {
                    Err(String::from_str("File cannot be created.").unwrap())
                }
            }
        },
        Err(_) => {
            Err(String::from_str("Encoding Failure").unwrap())
        } 
    }
}



fn parse(text: &str, var: &Arc<Mutex<HashMap<String, define::TeMP>>>) {
    match var.lock() {
        Ok(mut var) => {
            let com_make_quiz: regex::Regex = regex::Regex::new("makq ([a-zA-Z0-9_]+) \"([^\"]+)\"").unwrap();
            let com_new_question: regex::Regex = regex::Regex::new("newq ([a-zA-Z0-9_]+) \"([^\"]+)\" \"([^\"]+)\" ([0-9]+)").unwrap();
            let com_del_question: regex::Regex = regex::Regex::new("delq ([a-zA-Z0-9_]+) ([0-9]+)").unwrap();
            let com_export: regex::Regex = regex::Regex::new("export ([a-zA-Z0-9_]+)").unwrap();
            let com_import: regex::Regex = regex::Regex::new("import ([a-zA-Z0-9_]+) (.+)").unwrap();
            
            if let Some(match_items) = com_make_quiz.captures(text) {
                let varname = String::from_str(match_items.get(1).unwrap().as_str()).unwrap();
                let title = String::from_str(match_items.get(2).unwrap().as_str()).unwrap();

                let temp = define::TeMP { title: title, questions: vec![] };

                var.insert(varname, temp);

            }

            if let Some(match_items) = com_new_question.captures(text) {
                let varname = String::from_str(match_items.get(1).unwrap().as_str()).unwrap();
                let question = String::from_str(match_items.get(2).unwrap().as_str()).unwrap();
                let answer = String::from_str(match_items.get(3).unwrap().as_str()).unwrap();
                let point = match_items.get(4).unwrap().as_str().parse::<u32>().unwrap();

                match var.get_mut(&varname) {
                    Some(quiz) => {
                        quiz.questions.push(
                            define::Question {
                                question: question,
                                answer: answer,
                                point: point
                            }
                        );
                    },
                    None => {
                        println!("not quiz exists");
                    }
                }
            }

            if let Some(match_items) = com_del_question.captures(text) {
                let varname = String::from_str(match_items.get(1).unwrap().as_str()).unwrap();
                let mut index = match_items.get(2).unwrap().as_str().parse::<u32>().unwrap();

                match var.get_mut(&varname) {
                    Some(quiz) => {
                        if index > 0 && index as usize <= quiz.questions.len() {
                            index -= 1;
                            quiz.questions.remove(index as usize);
                        }
                    }, 
                    None => {
                        println!("not quiz exists");
                    }
                }
            }            
            
            if let Some(match_items) = com_export.captures(text) {
                let varname = String::from_str(match_items.get(1).unwrap().as_str()).unwrap();

                match var.get(&varname) {
                    Some(quiz) => {
                        match export(quiz.clone()) {
                            Ok(_) => {  },
                            Err(e) => { println!("{}", e) }
                        } 
                    },
                    None => {
                        println!("not quiz exists");
                    }
                }

            }

            if let Some(match_items) = com_import.captures(text) {
                let varname = String::from_str(match_items.get(1).unwrap().as_str()).unwrap();
                let filename = String::from_str(match_items.get(2).unwrap().as_str()).unwrap();

                match define::import(&filename) {
                    Ok(temp) => { var.insert(varname, temp); },
                    Err(e) => { println!("{}", e) }
                }


            }
        },
        Err(_) => {
            println!("Error: Cannot access variable");
        }
    }
}


pub fn start() {
    let var: Arc<Mutex<HashMap<String, define::TeMP>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        print!(">>>  ");
        let _ = stdout().flush();
        
        let mut command = String::new();

        let _ = stdin().read_line(&mut command);

        command = command.trim().to_string();

        if &command == "exit" {
            break;
        }

        parse(&command, &var);
    }       
}