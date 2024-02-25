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

#[test]
fn hello() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}