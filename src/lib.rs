use detour::static_detour;

use std::ops;
use std::mem;
use std::slice;
use broadsword::dll;
use broadsword::runtime;
use broadsword::scanner;

static_detour! {
    static GET_ENGINE_FLAG_HOOK: unsafe extern "system" fn(*const u8) -> bool;
}

#[dll::entrypoint]
pub fn entry(_: usize) -> bool {
    unsafe {
        GET_ENGINE_FLAG_HOOK
            .initialize(
                mem::transmute(find_aob()),
                |flag: *const u8| {
                    if *flag > 0 && *flag < 7 {
                        return false;
                    }

                    GET_ENGINE_FLAG_HOOK.call(flag)
                }
            )
            .unwrap();


        GET_ENGINE_FLAG_HOOK.enable().unwrap();
    }

    true
}

fn find_aob() -> usize {
    let (text_section, pattern) = get_game_specification()
        .expect("Did not recognize game. None of the .text sections have been found.");

    let size = text_section.end - text_section.start;
    let slice = unsafe { slice::from_raw_parts(text_section.start as *const u8, size) };

    let pattern = scanner::Pattern::from_pattern_str(pattern)
        .expect("Could not create pattern from supplied string");

    let result = scanner::simple::scan(slice, &pattern)
        .map(|r| r.location + text_section.start)
        .expect("Could not find AOB");

    result
}

fn get_game_specification() -> Option<(ops::Range<usize>, &'static str)> {
    if let Ok(text) = runtime::get_module_section_range("armoredcore6.exe", ".text") {
        return Some((text, "48 0f be 01 48 8d 0d ?? ?? ?? ?? 48 03 c0 48 ff 24 c1"));
    }

    if let Ok(text) = runtime::get_module_section_range("eldenring.exe", ".text") {
        return Some((text, "48 0f be 01 48 8d 0d ?? ?? ?? ?? 48 ff 24 c1"));
    }

    if let Ok(text) = runtime::get_module_section_range("sekiro.exe", ".text") {
        return Some((text, "48 0f be 01 48 8d 0d ?? ?? ?? ?? 48 ff 24 c1"));
    }

    if let Ok(text) = runtime::get_module_section_range("DarkSoulsIII.exe", ".text") {
        return Some((text, "48 0f be 01 48 8d 0d ?? ?? ?? ?? 48 ff 24 c1"));
    }

    None
}