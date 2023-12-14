use std::process::Command;

fn main() {
    // std();
    let cmd = determine_cmd();
    exec_and_get_result(cmd);
}

#[cfg(windows)]
fn set_console_encoding(){
    unsafe {
        SetConsoleOutputCP(65001);
    }
}

fn determine_cmd() ->String{
    let cmd = {
        if cfg!(target_os = "windows") {
            "tasklist"
        } else if cfg!(target_os="linux") {
            "ps"
        } else if cfg!(target_os="macos") {
            "ps"
        } else {
            panic!("unknow os")
        }
    }.to_string().to_lowercase();
    return cmd;
}

fn exec_and_get_result(cmd : String) {
    match Command::new(cmd).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                println!("Command output:\n{}", stdout);
            } else {
                eprintln!("Command failed: {}", stderr);
            }
        }
        Err(error) => {
            eprintln!("Command execution failed: {}", error);
        }
    }
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
