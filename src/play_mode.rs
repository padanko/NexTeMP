use std::io::stdin; // ユーザー入力を受け取るための標準ライブラリ
use crate::define; // "define" モジュールをインポート

pub fn start() {
    // プレイモードの開始関数

    let mut score: u32 = 0; // 合計スコアを格納する変数
    let mut filename = String::new(); // インポートするクイズファイルの名前を格納する変数

    let _ = stdin().read_line(&mut filename); // ユーザーからファイル名を入力
    filename = filename.trim().to_string(); // 入力値の前後の空白を削除

    // ファイルからクイズをインポート
    match define::import(&filename) {
        Ok(temp) => { 
            let mut index = 1; // 問題番号をカウント
            let mut vertics: Vec<bool> = Vec::new(); // 回答の正誤を記録するベクター

            let temp_clone = temp.clone(); // クイズデータのクローンを作成（スコア計算用）

            // クイズの各質問に対して処理
            for question in temp.questions {
                println!("{})  {}   (score: +{})", index, &question.question, &question.point);
                // 質問と得点を表示
                
                let mut user_answer = String::new();
                let _ = stdin().read_line(&mut user_answer); // ユーザーの回答を取得
                user_answer = user_answer.trim().to_string(); // 前後の空白を削除
                
                // 正答かどうかを判定
                let correct_answer = user_answer == question.answer;
                vertics.push(correct_answer); // 正誤を記録
                
                if correct_answer {
                    // 正答の場合、スコアを加算
                    score += question.point;
                }

                println!("\n"); // 改行
                index += 1; // 次の問題番号
            }

            // 最大得点を計算
            let max_score: u32 = temp_clone.questions.iter()
                .map(|q| q.point) // 各質問のポイントを取得
                .sum(); // 合計ポイントを計算
            
            // 結果を表示
            println!("--------=+< Result >+=--------");            
            println!("Your Score: {}/{}\n", score, max_score);

            // 各質問の正誤を表示
            for (i, &correct) in vertics.iter().enumerate() {
                if correct {
                    println!("{} O", i + 1); // 正解の場合
                } else {
                    println!("{} X", i + 1); // 不正解の場合
                }
            }
            println!("( O = correct X = incorrect )");
            
        },
        Err(e) => { 
            // クイズのインポートに失敗した場合、エラーメッセージを表示
            println!("{}", e); 
        }
    }
}
