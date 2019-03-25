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
struct MEM {
    total:f32,
    free:f32,
}
struct OS {
    name: String,
    version: String,
}
struct SysInfo {
    cpu: CPU,
    mem: MEM,
    os: OS,
}
impl SysInfo {
    fn new() -> SysInfo {
        // CPU
        let cpuid = CpuId::new();
        let Strcores = sys_info::cpu_num().unwrap();
        let tmp = cpuid.get_extended_function_info().unwrap();
        let Strbrand = tmp.processor_brand_string().unwrap();

        // MEM
        let mem = sys_info::mem_info().unwrap();

        // OS
        let os_name = sys_info::os_type().unwrap();
        let os_version = sys_info::os_release().unwrap();

        return SysInfo { 
            cpu:CPU{brand:String::from(Strbrand),cores:(Strcores as i32)},
            mem:MEM{total:(mem.total as f32),free:(mem.free as f32)},
            os:OS{name:os_name,version:os_version},
        };        
    }

    fn to_string(&self) -> String{
        let tmp = format!("Processor {}\nProcessor cores : {}\nMemory : {:.2} GB / {:.2} GB\nOS : {} {}",
            self.cpu.brand,self.cpu.cores,self.mem.free/GB,self.mem.total/GB,self.os.name,self.os.version);
        return String::from(tmp);
    }
}


fn main() {

    let mut app = Options();

    let mut matches = app.clone().get_matches();

    let sys = SysInfo::new();

    if matches.is_present("INFO"){

        println!("{}",sys.to_string());

    } else if matches.is_present("CPU") {

        println!("Processor : {}", sys.cpu.brand );
        println!("Processor cores : {}", sys.cpu.cores );

    } else if matches.is_present("MEM") {

        println!("Memory {:.2} GB / {:.2} GB",sys.mem.free/GB,sys.mem.total/GB); 

    } else if matches.is_present("OS") {

        println!("OS : {} {}", sys.os.name,sys.os.version);

    } else {

        app.print_help();

    }

}

fn Options<'a>() -> clap::App<'a,'a> {
    let result = App::new("RustSysInfo")
                            .version("0.0.1.3")
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
                                .short("o")
                                .long("os")
                                .required(false)
                                .help("Show only os info"));

                                return result;

}