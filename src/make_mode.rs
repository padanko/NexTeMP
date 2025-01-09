use serde_json; // JSONデータのシリアライズ/デシリアライズ用ライブラリ
use regex; // 正規表現を扱うためのライブラリ
use std::{fs, io::{stdin, stdout, Write}, str::FromStr}; // 標準ライブラリのファイル操作や入出力機能をインポート
use std::sync::{Arc, Mutex}; // スレッド間で共有するための同期プリミティブ
use std::collections::HashMap; // キーと値のペアを格納するデータ構造
use crate::define; // 外部モジュール "define" をインポート

// JSONファイルにクイズデータをエクスポートする関数
fn export(question: define::TeMP) -> Result<(), String> {
    match serde_json::to_string(&question) {
        // オブジェクトをJSON文字列に変換
        Ok(json_data) => {
            match fs::File::create(&format!("{}.json", question.title)) {
                // ファイルを作成して書き込み
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

// 入力されたテキストを解析し、対応するコマンドを実行する関数
fn parse(text: &str, var: &Arc<Mutex<HashMap<String, define::TeMP>>>) {
    match var.lock() {
        // スレッドセーフな変数にアクセス
        Ok(mut var) => {
            // コマンドを認識するための正規表現
            let com_make_quiz = regex::Regex::new("makq ([a-zA-Z0-9_]+) \"([^\"]+)\"").unwrap();
            let com_new_question = regex::Regex::new("newq ([a-zA-Z0-9_]+) \"([^\"]+)\" \"([^\"]+)\" ([0-9]+)").unwrap();
            let com_del_question = regex::Regex::new("delq ([a-zA-Z0-9_]+) ([0-9]+)").unwrap();
            let com_export = regex::Regex::new("export ([a-zA-Z0-9_]+)").unwrap();
            let com_import = regex::Regex::new("import ([a-zA-Z0-9_]+) (.+)").unwrap();
            
            // makqコマンド：新しいクイズ作成
            if let Some(match_items) = com_make_quiz.captures(text) {
                let varname = String::from_str(match_items.get(1).unwrap().as_str()).unwrap();
                let title = String::from_str(match_items.get(2).unwrap().as_str()).unwrap();
                let temp = define::TeMP { title: title, questions: vec![] }; // 新しいクイズオブジェクトを作成
                var.insert(varname, temp); // ハッシュマップに追加
            }

            // newqコマンド：クイズに新しい質問を追加
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
                        ); // クイズに質問を追加
                    },
                    None => {
                        println!("not quiz exists");
                    }
                }
            }

            // delqコマンド：質問を削除
            if let Some(match_items) = com_del_question.captures(text) {
                let varname = String::from_str(match_items.get(1).unwrap().as_str()).unwrap();
                let mut index = match_items.get(2).unwrap().as_str().parse::<u32>().unwrap();

                match var.get_mut(&varname) {
                    Some(quiz) => {
                        if index > 0 && index as usize <= quiz.questions.len() {
                            index -= 1; // インデックスを0始まりに調整
                            quiz.questions.remove(index as usize); // 質問を削除
                        }
                    }, 
                    None => {
                        println!("not quiz exists");
                    }
                }
            }            
            
            // exportコマンド：クイズをエクスポート
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

            // importコマンド：クイズをインポート
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

// クイズ管理システムを開始する関数
pub fn start() {
    let var: Arc<Mutex<HashMap<String, define::TeMP>>> = Arc::new(Mutex::new(HashMap::new())); // 共有可能なクイズ変数

    loop {
        print!(">>>  "); // プロンプト表示
        let _ = stdout().flush();
        
        let mut command = String::new();

        let _ = stdin().read_line(&mut command); // ユーザー入力を取得

        command = command.trim().to_string(); // 入力の前後の空白を削除

        if &command == "exit" {
            // "exit"コマンドで終了
            break;
        }

        parse(&command, &var); // 入力コマンドを解析
    }       
}
