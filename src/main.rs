#![crate_type = "bin"]

extern crate sys_info;
extern crate clap;

use std::convert::*;
use std::mem;
use std::string::String;
use std::io::{self, BufRead, Write};

use sys_info::*;
use clap::{Arg, App, SubCommand};

static GB : f32 = 1000000.;


fn main() {


let mut app = App::new("RustSysInfo")
                          .version("1.0.0.0")
                          .author("Moi")
                          .about("Does awesome things")
                          .arg(Arg::with_name("INFO")
                                .short("i")
                                .long("info")
                               .required(false)
                               .help("Show all info"))
                          .arg(Arg::with_name("CPU")
                               .short("c")
                               .long("cpu")
                               .required(false)
                               .help("Show only cpu info"))
                          .arg(Arg::with_name("MEM")
                                .short("m")
                                .long("mem")
                                .required(false)
                                .help("Show only memory info"))
                            .arg(Arg::with_name("OS")
                                .long("os")
                                .required(false)
                                .help("Show os info"));

    let mut matches = app.clone().get_matches();



    

    if matches.is_present("INFO"){
        println!("{}", CpuInfo() ); 
        println!("{}", MemoryInfo() );
        println!("{}", OsInfo() );
    } else if matches.is_present("CPU") {
        println!("{}", CpuInfo() ); 
    } else if matches.is_present("MEM") {
        println!("{}",MemoryInfo()); 
    } else if matches.is_present("OS") {
        println!("{}", OsInfo() );
    } else {
        app.print_help();
    }

}


fn MemoryInfo() -> String {
    let mem = sys_info::mem_info().unwrap();
    let tmp = format!("Memory  \t: {:.2} GB / {:.2} GB",(mem.free as f32)/GB,(mem.total as f32)/GB);
    let result = String::from(tmp);
    return result;
}

fn CpuInfo() -> String {
    let cpu = sys_info::cpu_speed().unwrap();
    let cores = sys_info::cpu_num().unwrap();
    let tmp = format!("Processor cores : {} \nProcessor speed : {:.2} GHZ",cores,(cpu as f32)/(1000 as f32));
    let result = String::from(tmp);
    return result;
}

fn OsInfo() -> String {
        let os = sys_info::os_type().unwrap();
        let os_version = sys_info::os_release().unwrap();
        let tmp = format!("OS  \t\t: {} {}",os,os_version);
        let result = String::from(tmp);
        return result;
}
