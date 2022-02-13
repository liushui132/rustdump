use std::ffi::c_void;
use windows::Win32::{Foundation::*,System::LibraryLoader::*,Security::*,System::Threading::*,System::SystemServices::*,System::Diagnostics::Debug::*,
    Storage::FileSystem::*,
};

use core::ptr;
use std::ptr::null_mut;
use std::mem::{size_of, zeroed};
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

    // let pidprocess = OpenProcess(PROCESS_ALL_ACCESS,BOOL(0),1588);


    let mut status =  OpenProcessToken(GetCurrentProcess(),TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut currenttoken);

    match status {
        BOOL(1) =>print!(""),
        _ =>return 0
    }   

    status =LookupPrivilegeValueA(PSTR(ptr::null_mut()), PSTR("SeDebugPrivilege\0".as_ptr()), &mut currentDebugValue);

    match status {
        BOOL(1) =>print!(""),
        _ =>return 0
    }   
    let new_privileges:TOKEN_PRIVILEGES = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Luid: currentDebugValue,
            Attributes: SE_PRIVILEGE_ENABLED,
        }],
    };

    let mut status2 = AdjustTokenPrivileges(currenttoken,BOOL(0),&new_privileges,std::mem::size_of_val(&new_privileges) as u32,ptr::null_mut(),ptr::null_mut());
    match status2 {
        BOOL(1) =>print!(""),
        _ =>return 0
    }   
    return 1;
    }
}


pub fn direct_dump(pid:u32,fullpathname:&str)  {

unsafe{

    let  debugprivilege = GetDebugPrivilege();
    if debugprivilege ==1 {
        println!("[+]GetDebugPrivilege success ....try to MiniDumpWriteDump process");
    } else {
        println!("[-]GetDebugPrivilege fialed ... check if you are Administrator");
        return 
    }      
    let pidprocess = OpenProcess(PROCESS_ALL_ACCESS,BOOL(0),pid);
    // let res:bool  =pidprocess.is_invalid();
    // match res {
    //     false => println!("[+]openprocess success"),
    //     true => println!("[-]openprocess failed"),
    // }

    //let  hhhnullfile:HANDLE =std::mem::zeroed();
    let hhfile =CreateFileA(PSTR(fullpathname.as_ptr()),FILE_ACCESS_FLAGS(GENERIC_READ |GENERIC_WRITE) ,FILE_SHARE_WRITE,std::ptr::null_mut(), CREATE_ALWAYS,FILE_ATTRIBUTE_NORMAL,None);

    match hhfile.ok() {
        Err(e) => println!("[-]CreateFileA falied ,error is {:?}",e) ,
        _ => println!("[+]CreateFileA success"),
    }
     
    
    let status = MiniDumpWriteDump(pidprocess,pid,hhfile,MiniDumpWithDataSegs|MiniDumpWithFullMemory,ptr::null_mut(),ptr::null_mut(),ptr::null_mut());
    match status {
        BOOL(1) =>println!("[+]MiniDumpWriteDump success "),
        _ =>println!("[-]MiniDumpWriteDump fialed")
    }   
}
    
}