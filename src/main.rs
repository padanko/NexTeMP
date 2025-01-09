use std::io::{stdin, stdout, Write}; // 入力と出力に関連する標準ライブラリをインポート

mod make_mode; // "make_mode"モジュールをインポート
mod play_mode; // "play_mode"モジュールをインポート
mod define;    // "define"モジュールをインポート

fn main() {
    // プログラムのエントリーポイント

    println!("NexTeMP v1.0.0");
    println!("------------------------------");

    let mut make: bool = false; // モードを示すフラグ変数。初期値はfalse

    loop {
        // モード選択の入力ループ
        print!("m(make)/p(play): ");
        let _ = stdout().flush(); // 標準出力をフラッシュして、プロンプトを即座に表示

        let mut m_or_p = String::new(); // 入力値を格納する文字列
        let _ = stdin().read_line(&mut m_or_p); // ユーザー入力を取得

        let m_or_p: String = m_or_p.trim().into(); // 入力文字列の前後の空白を削除して新しい文字列に変換

        if &m_or_p == "m" || &m_or_p == "p" {
            // "m" または "p" が入力された場合
            make = &m_or_p == "m"; // "m"が入力された場合はtrue、"p"の場合はfalse
            break; // 入力ループを抜ける
        }
    }

    if make {
        // makeモードの場合
        make_mode::start(); // "make_mode"モジュールのstart関数を呼び出す
    } else {
        // playモードの場合
        loop {
            // 無限ループでplay_modeを繰り返し実行
            play_mode::start(); // "play_mode"モジュールのstart関数を呼び出す
        }
    }
}
