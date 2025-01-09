use std::fs; // ファイル操作のための標準ライブラリ
use std::io::Read; // ファイルの内容を読み取るための標準ライブラリ
use serde::{Serialize, Deserialize}; // シリアライズとデシリアライズのためのライブラリ

// 質問を表す構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub question: String, // 質問文
    pub answer: String,   // 正解の答え
    pub point: u32,       // 質問のポイント（得点）
}

// クイズ全体を表す構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeMP {
    pub title: String, // クイズのタイトル
    pub questions: Vec<Question>, // 質問のリスト
}

// JSONファイルからクイズデータを読み込み、`TeMP` オブジェクトとして返す関数
// PlayMode と MakeMode の両方で使用される
pub fn import(filename: &str) -> Result<TeMP, String> {
    // 指定されたファイルを開く
    match fs::File::open(filename) {
        Ok(mut file) => {
            let mut buffer = String::new(); // ファイル内容を格納するための文字列バッファ
            let _ = file.read_to_string(&mut buffer); // ファイル内容を文字列として読み込む

            // JSON文字列を `TeMP` 構造体にデシリアライズ
            match serde_json::from_str::<TeMP>(&buffer) {
                Ok(temp) => {
                    // 成功時には `TeMP` オブジェクトを返す
                    Ok(temp)
                },
                Err(_) => {
                    // JSONデコード失敗時のエラー処理
                    Err("Decode failed.".into())
                }
            }
        }, 
        Err(_) => {
            // ファイルを開けなかった場合のエラー処理
            Err("File cannot be created.".into())
        }
    }
}
