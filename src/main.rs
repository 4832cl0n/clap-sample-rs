#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};


fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("pa")
            .help("sample positional argument")
            .required(true)
        ).arg(Arg::with_name("flg")
            .help("sample flag")
            .short("f")
            .long("flag")
        ).arg(Arg::with_name("opt")
            .help("sample flag")
            .short("o")
            .long("opt")
            .takes_value(true)
        ).subcommand(SubCommand::with_name("sub")
            .about("sample subcommand")
            .arg(Arg::with_name("subflg")
                .help("sample flag by sub")
                .short("f")
                .long("flag")
            )
        );

        // 引数を解析
        let matches = app.get_matches();

        // paが指定されていれば値を表示
        if let Some(o) = matches.value_of("pa") {
            println!("Value for pa: {}", o);
        }
        // optが指定されていれば値を表示
        if let Some(o) = matches.value_of("opt") {
            println!("Value for opt: {}", o);
        }

        // flgのON/OFFで表示するメッセージを切り替え
        println!("flg is {}", if matches.is_present("flg") {"ON"} else {"OFF"});

        // subサブコマンドの解析結果を取得
        if let Some(ref matches) = matches.subcommand_matches("sub") {
            println!("used sub");
            // subflgのON/OFFで表示するメッセージを切り替え
            println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
        }
}
