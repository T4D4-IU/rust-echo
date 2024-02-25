use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>; // テスト関数の戻り値の型エイリアスの作成。

#[test]
fn  dies_no_args() -> TestResult {
    Command::cargo_bin("echor")? // Ok値のアンパックやErr値の伝播の為にunwrapの代わりに?を使っている
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(()) // Ok値を返す為に;を省略している
}

fn run(args: &[&str], expected_file: &str) -> TestResult { // argsは&strのスライス expected_fileは&str, 戻り値はTestResult
    let expected = fs::read_to_string(expected_file)?; // ファイルの中身を読み込む
    Command::cargo_bin("echor")? // 与えられた引数でechorを実行し標準出力が期待通りかどうかを確認する
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(()) //正常に実行出来たらOk値を返す
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt") // 1つの引数を与えて実行
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt") // 2つの引数を与えて実行
}

#[test]
fn hello3() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt") // 引数とフラグを与えて実行
}

#[test]
fn hello4() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt") // フラグと2つの引数を与えて実行
}