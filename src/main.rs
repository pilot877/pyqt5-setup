use std::path::Path;
use std::fs;
use std::env;

mod vars;

fn main(){
    let args: Vec<String> = env::args().collect();
    
    if (args.len() > 1) && (args.len() < 3) {
        if &args[1] == "help" {
            println!("To create a new pyqt5 project, run the command\x1b[1m pyqt5-setup init\x1b[0m");
        } else if &args[1] == "init" {
            init();
        } else {
            println!("Option not found, run the command\x1b[1m pyqt5-setup help\x1b[0m for help");
        }
    } else if args.len() < 3 {
        println!("Error: too little args\nrun the command\x1b[1m pyqt5-setup help\x1b[0m for help");
    } else if args.len() > 2{
        println!("Error: too many args\nrun the command\x1b[1m pyqt5-setup help\x1b[0m for help");
    }
    
    

    
}


fn init() -> std::io::Result<()> {
    

    let mut files = vars::get_vars();

    // let mut file = fs::File::create("main.py")?;
    fs::create_dir("uis");
    fs::create_dir("unconverted_uis");

    if !Path::new("main.py").exists() {
        fs::write("main.py", format!("{}", files[0]))?;
    } else {
        println!("Error: main.py exists, program will not overwrite. If you wish to create a new main.py file, delete the old one");
    }
    if !Path::new("uis/Ui_MainWindow.py").exists() {
        fs::write("uis/Ui_MainWindow.py", format!("{}", files[1]))?;

    } else {
        println!("Error: Ui_MainWindow.py exists, program will not overwrite. If you wish to create a new Ui_MainWindow.py file, delete the old one");
    }
    if !Path::new("unconverted_uis/MainWindow.ui").exists() {
        fs::write("unconverted_uis/MainWindow.ui", format!("{}", files[2]))?;
    } else {
        println!("Error: MainWindow.ui exists, program will not overwrite. If you wish to create a new MainWindow.ui  file, delete the old one");
    }
    Ok(())

}