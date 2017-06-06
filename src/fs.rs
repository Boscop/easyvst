use std::path::{Path, PathBuf};

#[cfg(windows)]
pub fn get_folder_path() -> Option<PathBuf> {
	use winapi::*;
	use kernel32::*;

	use std::ptr::null_mut;
	use std::mem;
	use std::ffi::OsString;
	use std::os::windows::ffi::OsStringExt;

	const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: DWORD = 0x00000004;
	const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: DWORD = 0x00000002;

	let mut hm: HMODULE = null_mut();

	unsafe {
		if GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, &get_folder_path as *const _ as LPCWSTR, &mut hm as *mut _) == 0 {
			error!("GetModuleHandleExW() failed with {}", GetLastError());
			None
		} else {
			let mut path: [WCHAR; MAX_PATH + 1] = mem::zeroed();
			let len = GetModuleFileNameW(hm, path.as_mut_ptr(), path.len() as u32) as usize;
			if len == 0 {
				error!("GetModuleFileNameW() failed with {}", GetLastError());
				None
			} else {
				let file_path = OsString::from_wide(&path[..len]);
				Some(Path::new(&file_path).parent().unwrap().into())
			}
		}
	}
}