use adb_rs::AdbClient;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about="adb unauth scanner", long_about = None)]
struct Args {
   /// adb port open list file path
   #[arg(short, long)]
   input_path: String,

   /// adb unauth out file path
   #[arg(short, long, default_value_t = String::from("adb_unauth.txt"))]
   out_path: String,
}


fn main() {

    let cli = Args::parse();
    // if read_lines();
    let mut result :Vec<String>= Vec::new();
    match read_lines(cli.input_path) {
        Ok(lines) =>{

            for line in lines{
                if let Ok(addr) = line{
                    let mut addr2 = addr.to_string();
                    if !addr.contains(":"){
                        addr2 = format!("{}:5555",addr);
                    }
                    if check(addr2.as_str()){
                        println!("addr:{}",addr2);
                        result.push(addr2);
                    }
                }
            }
        }
        Err(err) =>{
            println!("open file err:{}",err)
        }
    }

    if result.len() > 0{

        let mut file = match File::create(cli.out_path){
            Ok(file)=> file,
            Err(err)=>{
                println!("open file failed:{}",err);
                std::process::exit(-1);
            }
        };
        for mut r in result{
            r.push('\n');
            file.write_all(r.as_bytes()).unwrap();
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P :AsRef<Path>{
     let file = File::open(filename)?;
     Ok(io::BufReader::new(file).lines())
}




fn check(addr:&str) -> bool{
    let ok:bool = match AdbClient::new("host::").connect(addr) {
        Ok(_)=>{ true }
        Err(err)=> match err {
            adb_rs::result::AdbError::AuthNotSupported=>{
                false
            }
            _=>{
                println!("{} connec err {}",addr,err.to_string());
                false
            } 
        }
    }; 
    return ok;
}
