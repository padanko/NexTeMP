use std::io::{stdin,stdout,Write};

mod make_mode;
mod play_mode;
mod define;

fn main() {

    println!("TeMP v1.0.0");
    println!("------------------------------");

    let mut make: bool = false;

    loop {
        print!("m(make)/p(play): ");
        let _ = stdout().flush();
            let mut m_or_p = String::new();
        let _ = stdin().read_line(&mut m_or_p);

        if &m_or_p == "m\n" || &m_or_p == "p\n" {
            make = &m_or_p == "m\n";
            break
        }
    }

    if make {
        make_mode::start();
    } else {
        loop {
            play_mode::start();
        }
    }


}