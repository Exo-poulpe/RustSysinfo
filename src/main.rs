#![crate_type = "bin"]

extern crate sys_info;  // Memory info + os info
extern crate clap;      // Arg parsing
extern crate raw_cpuid; // Cpu info

use std::convert::*;
use std::mem;
use std::string::String;
use std::io::{self, BufRead, Write};

use sys_info::*;
use clap::{Arg, App, SubCommand};
use raw_cpuid::CpuId;

static GB : f32 = 1000000.;


struct CPU {
    brand: String,
    cores: i32,
}
impl CPU {
    fn new() -> CPU{
        let cpuid = CpuId::new();
        let Strcores = sys_info::cpu_num().unwrap();
        let tmp = cpuid.get_extended_function_info().unwrap();
        let Strbrand = tmp.processor_brand_string().unwrap();
        CPU {brand:Strbrand.to_string(),cores:(Strcores as i32)}
    }

    fn to_string(&self) -> String{
        let tmp = format!("Processor : {}\nProcessor cores : {}\n",self.brand,self.cores);
        return String::from(tmp);
    }
}


fn main() {


let mut app = App::new("RustSysInfo")
                            .version("0.0.1.0")
                            .author("Exo-poulpe")
                            .about("Show system info from rust binarie")
                            .arg(Arg::with_name("INFO")
                                .short("i")
                                .long("info")
                                .required(false)
                                .help("Show all system info"))
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
                                .help("Show only os info"));

    let mut matches = app.clone().get_matches();

    if matches.is_present("INFO"){
        println!("{}", CPU::new().to_string() ); 
        println!("{}", MemoryInfo() );
        println!("{}", OsInfo() );
    } else if matches.is_present("CPU") {
        println!("{}", CPU::new().to_string() );
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

fn OsInfo() -> String {
    let os = sys_info::os_type().unwrap();
    let os_version = sys_info::os_release().unwrap();
    let tmp = format!("OS  \t\t: {} {}",os,os_version);
    let result = String::from(tmp);
    return result;
}
