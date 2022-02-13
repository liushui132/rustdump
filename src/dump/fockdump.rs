







use windows::Win32::{Foundation::*,System::LibraryLoader::*,Security::*,System::Threading::*,System::SystemServices::*,System::Diagnostics::Debug::*,
    Storage::FileSystem::*,
};
use core::ptr;
use std::ptr::null_mut;
use std::mem::zeroed;
fn getmodule(dllname:PSTR) -> HINSTANCE {
    let module  = unsafe {LoadLibraryA(dllname)};
    // match module {
    //     null => println!("error"),
    //     other => println!("find dllname okk")
    // }
    module
}

fn getprocaddress(dllnhinstance:HINSTANCE,fnname:PSTR) -> FARPROC {
    let address = unsafe{GetProcAddress(dllnhinstance,fnname)};
    address
}



/*  OpenProcess   OpenProcessToken  LookupPrivilegeValueA  AdjustTokenPrivileges */

fn GetDebugPrivilege() -> i32 {
  unsafe{
    let mut currenttoken:HANDLE =std::mem::zeroed();

    let mut currentDebugValue:LUID = std::mem::zeroed();
    let mut status =  OpenProcessToken(GetCurrentProcess(),TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut currenttoken);

    match status {
        BOOL(1) =>println!("OpenProcessToken success "),
        _ => return 0
        //_ =>println!("OpenProcessToken fialed")
    }   

    status =LookupPrivilegeValueA(PSTR(ptr::null_mut()), PSTR("SeDebugPrivilege\0".as_ptr()), &mut currentDebugValue);

    match status {
        BOOL(1) =>println!("LookupPrivilegeValue success "),
        _ => return 0
        // _ =>println!("LookupPrivilegeValue fialed"),
    }   
    println!("the LookupPrivilegeValueA is {:?}",currentDebugValue);
    let new_privileges:TOKEN_PRIVILEGES = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Luid: currentDebugValue,
            Attributes: SE_PRIVILEGE_ENABLED,
        }],
    };

    status = AdjustTokenPrivileges(currenttoken,BOOL(0),&new_privileges,std::mem::size_of_val(&new_privileges) as u32,ptr::null_mut(),ptr::null_mut());
    match status {
        BOOL(1) =>println!("AdjustTokenPrivileges success "),
        _ => return 0
        // _ =>println!("AdjustTokenPrivileges fialed"),
    }   

  
    return 1;
    }
}


pub fn fock_dump(pid:u32, fullpathname:&str)  {

    let mut  status = GetDebugPrivilege();
    if status ==1 {
        println!("GetDebugPrivilege success ....try to MiniDumpWriteDump process");
    } else {
        println!("GetDebugPrivilege fialed ... check if you are Administrator");
        return 
    }       
    unsafe{
    let pidprocess = OpenProcess(PROCESS_ALL_ACCESS,BOOL(0),pid);
    let res:bool  =pidprocess.is_invalid();
    // match res {
    //     false => println!("openprocess success"),
    //     // true => println!("openprocess failed"),
    // }
    let mut hhhnullfile:HANDLE =std::mem::zeroed();
    let hhfile =CreateFileA(PSTR(fullpathname.as_ptr()),FILE_ACCESS_FLAGS(GENERIC_READ |GENERIC_WRITE) ,FILE_SHARE_MODE(0),std::ptr::null_mut(), CREATE_NEW,FILE_FLAGS_AND_ATTRIBUTES(0),hhhnullfile);
    let boolstatus = MiniDumpWriteDump(pidprocess,pid,hhfile,MiniDumpWithFullMemoryInfo | MiniDumpWithUnloadedModules,ptr::null_mut(),ptr::null_mut(),ptr::null_mut());
    match boolstatus {
        BOOL(1) =>println!("MiniDumpWriteDump success "),
        _ =>println!("MiniDumpWriteDump fialed"),
    }     
    return 
    }
}







