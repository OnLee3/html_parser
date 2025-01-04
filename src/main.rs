use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// markdown 형태의 파일을 입력받아 html 태그로 감싼 결과를 출력하는 프로그램입니다.
struct Args {
    /// 입력 파일(들)
    #[arg(value_name = "FILE", required = true)]
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("{args:#?}");
}
