use std::fs;
use std::path::Path;
use std::{env, error::Error};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

mod compiler;

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Ok(())
    }

    build(&argv)?;

    Ok(())
}

fn build(argv: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let spec_argv = &argv[2..];
    let dont_del_temp: bool = spec_argv.contains(&"-t".to_string());
    let run_exe: bool = spec_argv.contains(&"-r".to_string());

    let sys_time: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let c_file_name = format!("temp_{}.c", sys_time);

    let file_path = &argv[1];
    let source = fs::read_to_string(file_path)?;
    let tokens = compiler::lex(&source);

    let path = Path::new(file_path);
    let output_name = path
        .file_stem()  // Получает имя без расширения
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    compiler::build_c(&tokens, &c_file_name)?;
    compile_c(&c_file_name, output_name)?;

    if run_exe {
        run_program(output_name);
    }

    if !dont_del_temp {
        fs::remove_file(&c_file_name)?
    }

    Ok(())
}

fn run_program(output_name: &str) {
    let _ = Command::new(&format!("./{output_name}")).spawn();
}

fn compile_c(file_path: &str, output_name: &str) -> Result<i32, Box<dyn Error>> {
    

    let code = Command::new("gcc")
        .arg(file_path)
        .arg("-o")
        .arg(output_name)
        .output()?
        .status
        .code()
        .ok_or("couldn't get the code")?;

    Ok(code)
}

mod tests {
    use crate::{compile_c, build};

    #[test]
    fn compile_c_test() {
        if let Ok(code) = compile_c("tests/compile.c", "output.bf") {
            assert_eq!(code, 0);
        } else {
            panic!("couldn't call the C compiler");
        }
    }

    #[test]
    fn compile_hello_world() {
        let argv = vec!["rustbf", "tests/hello_world.bf"].iter().map(|x| x.to_string()).collect::<Vec<_>>();
        if let Err(e) = build(&argv) {
            panic!("compile err: {e}");
        }
    }
}