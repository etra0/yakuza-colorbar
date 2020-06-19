use std::ffi::{CStr, CString};
use std::io;
use std::os::raw::c_char;
use std::ptr;
use winapi;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;

#[derive(PartialEq, Debug)]
enum Style {
    BrawlerFirst,
    Brawler,
    Beast,
    Rush,
    Legend
}

#[derive(Debug)]
struct ColorBar {
    style: Style,
    addr: usize,
    color: Vec<u8>,
}

fn initialize_colors() -> Vec<ColorBar> {
    vec![
        ColorBar { style: Style::BrawlerFirst, addr: 0xEE914, color: vec![0x0A, 0xB4, 0xFF, 0xFF] }
    ]
}

fn write_aob(addr: usize, data: Vec<u8>) {
    use winapi::um::memoryapi::VirtualProtect;
    let s = data.len();
    let mut prot: DWORD = 0x0;
    let mut ptr = addr;
    unsafe {
        VirtualProtect(
            addr as LPVOID,
            s,
            winapi::um::winnt::PAGE_EXECUTE_READWRITE,
            &mut prot,
        );
        let mut target = ptr as *mut u8;
        for x in data {
            println!("{}", x);
            *target = x;
            ptr += 1;
            target = ptr as *mut u8;
        }
        VirtualProtect(addr as LPVOID, s, prot, std::ptr::null_mut());
    }
}

fn parse_data(colors: &mut Vec<ColorBar>) {
    let parsed = vec![ColorBar {
        style: Style::BrawlerFirst,
        addr: 0,
        color: vec![0xFF, 0x00, 0x00, 0xFF]
    }];

    for color in colors {
        println!("{:?}", color);
    }
}

#[no_mangle]
pub unsafe extern "system" fn init(_: LPVOID) -> DWORD {
    let mut _buff = String::new();
    AllocConsole();

    let game_name = CString::new("Yakuza0.exe").unwrap();
    let mba = winapi::um::libloaderapi::GetModuleHandleA(game_name.as_ptr()) as usize;

    let mut colors = initialize_colors();
    parse_data(&mut colors);

    winapi::um::wincon::FreeConsole();

    return 1;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_: HINSTANCE, reason: DWORD, _: LPVOID) -> BOOL {
    unsafe {
        match reason {
            winapi::um::winnt::DLL_PROCESS_ATTACH => {
                winapi::um::processthreadsapi::CreateThread(
                    ptr::null_mut(),
                    0,
                    Some(init),
                    ptr::null_mut(),
                    0,
                    ptr::null_mut(),
                );
            }
            _ => (),
        };
    }

    return true as BOOL;
}
