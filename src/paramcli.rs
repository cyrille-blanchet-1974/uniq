use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Paramcli
//parameters from command line and/or confFile
{
    pub fic: String, //file to work with if specified
    pub count: bool,
    pub not_case_sensitive: bool,
}

impl Default for Paramcli {
    fn default() -> Self {
        Paramcli::new()
    }
}

impl Paramcli {
    pub fn new() -> Paramcli {
        let mut fic = String::new();
        let mut count = false;
        let mut not_case_sensitive = false;

        let args: Vec<String> = env::args().skip(1).collect();
        let name = env::args()
            .take(1)
            .next()
            .unwrap_or_else(|| String::from("sort"));
        /*if args.len() <> 2 {
            println!("Error: not enough parameters");
            help(&name);
        }*/
        for arg in args {
            if arg == "/?"
                || arg == "-?"
                || arg.to_lowercase() == "/help"
                || arg.to_lowercase() == "-help"
            {
                help(&name);
            }
            if arg == "-c" {
                count = true;
                continue;
            }
            if arg == "-i" {
                not_case_sensitive = true;
                continue;
            }
            fic = arg;
        }
        //checks
        if !fic.is_empty() {
            //check if file exists
            if File::open(&fic).is_err() {
                println!("Error file {} doesn't exists or unereadable", &fic);
                help(&name);
            };
        }
        Paramcli {
            fic,
            count,
            not_case_sensitive,
        }
    }
}

fn help(name: &str) {
    println!("{} 1.0 (2022)", name);
    println!("syntax : {} [file] [-r]   ", name);
    println!("parameters between [] are optionnals");
    println!("------------------------------------");
    println!("fic: file to work with (if non use stdin");
    println!("-c : add count");
    println!("-i : not case sensitive");
    std::process::exit(0);
}
