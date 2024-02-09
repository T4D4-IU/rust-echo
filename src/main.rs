use clap::{App, Arg};

fn main() {
    let matches = App::new("echor") // make new App
        .version("0.1.0") // set version
        .author("Asaki Tada <t4d4.icp@gmail.com>") // set author
        .about("echo written in Rust") // set explanation
        .arg(
            Arg::with_name("text") // make new Arg with name "text"
                .value_name("TEXT")  // 引数の値の名前
                .help("Input text") // ヘルプメッセージ
                .required(true) // 必須引数
                .min_values(1), // 最低1つの引数を受け付ける
        )
        .arg(
            Arg::with_name("omit_newline") // make new Arg with name "omit_newline"
                .short("n")
                .help("Do not print newline")
                .takes_value(false), // 引数を取らない nという名前だけを持つフラグ
        )
        .get_matches(); // 引数を解析

    let text = matches.values_of_lossy("text").unwrap(); // values_of_lossyはOption<Vec<String>>を返す unwrapで中身を取り出す
    let omit_newline = matches.is_present("omit_newline"); // omit_newlineが指定されたかどうか boolで返す
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" }); // joinでVec<String>をStringに変換して出力 末尾に改行を出力するかどうかを判別するif式はここでしか使わないので変数に代入せず直接書いている
}
