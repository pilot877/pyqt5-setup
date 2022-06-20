use std::path::Path;
use std::fs;
use std::env;
use std::io::stdin;
use std::io;

mod vars;

fn main(){
    let args: Vec<String> = env::args().collect();
    
    if (args.len() > 1) && (args.len() < 3) {
        if &args[1] == "help" {
            println!("To create a new pyqt5 project, run the command\x1b[1m pyqt5-setup init\x1b[0m");
        } else if &args[1] == "init" {
            init();
        } else {
            println!("\x1b[31m\x1b[1mError!\x1b[0m Option not found, run the command\x1b[1m pyqt5-setup help\x1b[0m for help");
        }
    } else if args.len() < 3 {
        println!("\x1b[31m\x1b[1mError!\x1b[0m Too little args\nrun the command\x1b[1m pyqt5-setup help\x1b[0m for help");
    } else if args.len() > 2{
        println!("\x1b[31m\x1b[1mError!\x1b[0m Too many args\nrun the command\x1b[1m pyqt5-setup help\x1b[0m for help");
    }
    // Add more error checking etc. here    
}


fn init() -> std::io::Result<()> {
    let mut files = vars::get_vars();
    
    fs::create_dir("uis");
    fs::create_dir("unconverted_uis");

    let mut real_string = String::new();

    if !Path::new("main.py").exists() {
        fs::write("main.py", format!("{}", files[0]))?;
        println!("\x1b[32;1m\x1b[1mSuccess!\x1b[0m main.py created");
    } else {
        real_string = "".to_string();
        print!("\x1b[33;1m\x1b[1mWarning!\x1b[0m main.py exists, would you like to overwrite it? [y/N] ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        stdin().read_line(&mut real_string).expect("Failed to read line!");
        real_string = real_string.to_string();
        let mut input_string = real_string.trim();
        if input_string == "y" || input_string == "N" {
            fs::write("main.py", format!("{}", files[0]))?;
            println!("\x1b[32;1m\x1b[1mSuccess!\x1b[0m main.py overwritten");
        } else if input_string == "n" || input_string == "N" || input_string == ""{
            println!("\x1b[31;1m\x1b[1mError!\x1b[0m main.py exists, program will not overwrite");
        } else {
            println!("\x1b[31;1m\x1b[1mError!\x1b[0m Unknown input character '{}'. main.py exists, program will not overwrite", input_string);
        }        
    }

    if !Path::new("uis/Ui_MainWindow.py").exists() {
        fs::write("uis/Ui_MainWindow.py", format!("{}", files[1]))?;
        println!("\x1b[32;1m\x1b[1mSuccess!\x1b[0m Ui_MainWindow.py created");
    } else {
        real_string = "".to_string();
        print!("\x1b[33;1m\x1b[1mWarning!\x1b[0m Ui_MainWindow.py exists, would you like to overwrite it? [y/N] ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        stdin().read_line(&mut real_string).expect("Failed to read line!");
        real_string = real_string.to_string();
        let mut input_string = real_string.trim();
        if input_string == "y" || input_string == "N" {
            fs::write("uis/Ui_MainWindow.py", format!("{}", files[1]))?;
            println!("\x1b[32;1m\x1b[1mSuccess!\x1b[0m Ui_MainWindow.py overwritten");
        } else if input_string == "n" || input_string == "N" || input_string == ""{
            println!("\x1b[31;1m\x1b[1mError!\x1b[0m Ui_MainWindow.py exists, program will not overwrite");
        } else {
            println!("\x1b[31;1m\x1b[1mError!\x1b[0m Unknown input character '{}'. Ui_MainWindow.py exists, program will not overwrite", input_string);
        }
    }

    if !Path::new("unconverted_uis/MainWindow.ui").exists() {
        fs::write("unconverted_uis/MainWindow.ui", format!("{}", files[2]))?;
        println!("\x1b[32;1m\x1b[1mSuccess!\x1b[0m MainWindow.ui created");
    } else {
        real_string = "".to_string();
        print!("\x1b[33;1m\x1b[1mWarning!\x1b[0m MainWindow.py exists, would you like to overwrite it? [y/N] ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        stdin().read_line(&mut real_string).expect("Failed to read line!");
        real_string = real_string.to_string();
        let mut input_string = real_string.trim();
        if input_string == "y" || input_string == "N" {
            fs::write("unconverted_uis/MainWindow.ui", format!("{}", files[2]))?;
            println!("\x1b[32;1m\x1b[1mSuccess!\x1b[0m MainWindow.ui overwritten");
        } else if input_string == "n" || input_string == "N" || input_string == ""{
            println!("\x1b[31;1m\x1b[1mError!\x1b[0m MainWindow.ui exists, program will not overwrite");
        } else {
            println!("\x1b[31;1m\x1b[1mError!\x1b[0m Unknown input character '{}'. MainWindow.ui exists, program will not overwrite", input_string);
        }
    }
    Ok(()) // Do something with this lol.
}