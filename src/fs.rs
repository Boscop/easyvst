use std::path::{Path, PathBuf};

#[cfg(windows)]
pub fn get_folder_path() -> Option<PathBuf> {
	use winapi::*;
	use kernel32;

	use std::ptr::null_mut;
	use std::mem;

	const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: DWORD = 0x00000004;
	const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: DWORD = 0x00000002;

	unsafe {
		let mut hm: HMODULE = null_mut();

		if kernel32::GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, &get_folder_path as *const _ as LPCWSTR, &mut hm as *mut _) == 0 {
			error!("GetModuleHandle returned {}", kernel32::GetLastError());
			None
		} else {
			let mut path: [WCHAR; MAX_PATH + 1] = mem::zeroed();
			kernel32::GetModuleFileNameW(hm, path.as_mut_ptr(), mem::size_of_val(&path) as u32);
			let len = path.iter().position(|&c| c == 0).unwrap();
 			let file_path = String::from_utf16_lossy(&path[..len]);
			let folder_path = Path::new(&file_path).parent().unwrap().into();
			Some(folder_path)
		}
	}
}