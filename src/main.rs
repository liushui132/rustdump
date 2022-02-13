use dump::fockdump::fock_dump;
use dump::directdump::direct_dump;
use std::env;

mod dump;

fn help_dump()  {
    println!("***********************************");
    println!("usage:dump.exe moudle  pid dmpfile");
    println!("module : directdump  fockdump ");
    println!("***********************************");
}

fn main() {

    let args:Vec<String> = std::env::args().collect();
    if args.len()<4 {
      help_dump();  
      return ;
    }
    let pid = args[2].parse::<u32>().unwrap();
    match args[1].as_str() {
        "directdump" =>direct_dump(pid,args[3].as_str()), 
        "fockdump" => fock_dump(pid, args[3].as_str()),               //todo
        _ => help_dump(),

    } 
    //let res =direct_dump(1312,"fdsfds.dmp");
    // match res {
    //     1 =>println!("[+]fockdump success"),
    //     other => println!("[-]fockdump error")
    // }

    println!("[+]dump finish")
}

