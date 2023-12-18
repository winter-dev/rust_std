use std::process::{Command, Output};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo).route("/", web::get().to(index)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

#[get("/echo")]
async fn echo(req: String) -> impl Responder {
    HttpResponse::Ok().body(req)
}
#[cfg(windows)]
// #[link(name = "user32")]
extern "system" {
    fn SetConsoleOutputCP(code_page: u32) -> bool;
}

#[cfg(windows)]
fn set_console_encoding() {
    unsafe {
        SetConsoleOutputCP(65001);
    }
}

async fn index() -> impl Responder {
    match exec_and_get_result().await {
        Ok(r) => HttpResponse::Ok().body(r),
        Err(cause) => {
            println!("执行失败: {}", cause);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

fn determine_cmd() -> String {
    let cmd = {
        if cfg!(target_os = "windows") {
            "tasklist"
        } else if cfg!(target_os = "linux") {
            "ps -a"
        } else if cfg!(target_os = "macos") {
            "ps -a"
        } else {
            panic!("unknow os")
        }
    }
    .to_string()
    .to_lowercase();
    #[cfg(windows)]
    set_console_encoding();
    return cmd;
}

async fn exec_and_get_result() -> Result<String, std::io::Error> {
    let cmd = determine_cmd();
    let cmd_slice: &str = cmd.as_str();
    let (cmd, args) = match cmd.split_once(' ') {
        Some((c, a)) => (c, a),
        None => (cmd_slice, ""),
    };

    let mut command = Command::new(cmd);
    if "" != args {
        command.args(args.split_whitespace());
    }
    let out: Output = command.output()?;
    let r = String::from_utf8_lossy(&out.stdout).to_string();
    Ok(r)
}

/*
fn get_system_name() -> &'static str {
    match std::env::consts::OS {
        "linux" => "Linux",
        "macos" => "macOS",
        "windows" => "Windows",
        _ => "Unknown",
    }
}
*/

/*
fn std() {
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    for i in c {
        println!("{}", i);
    }

    println!("###########");

    let d = [3; 5];
    for i in d {
        println!("{}", i);
    }

    println!("#########");

    let a = [10, 20, 30, 40, 50];
    for i in 0..a.len() {
        println!("a[{}] = {}", i, a[i]);
    }

    println!("#########");

    println!("{}", add(3, 2));

    println!("{}", add0());

    fn five() -> i32 {
        5
    }
    let n = five();
    if n > 0 {
        println!("{} 大于0", n);
    }

    let x = 5;
    let y = x;
    println!("x==y:{}", x == y);

    let s1 = "hello";
    let s2 = String::from("hello");
    let s3 = s1;
    println!("s1==s2:{}", s1 == s2);
    println!("s1:{}    s3:{}", s1, s3);

    let s4 = String::from("value");
    let s5 = s4.clone();
    println!("s4==s5:{}", s4 == s5);
}

///2个i32类型数据求和
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn add0() -> i32 {

    let a = 7;
    let b = 2;
    let c = 3;
    c * (a + b)
}
*/
