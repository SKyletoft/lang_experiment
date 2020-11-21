extern "C" {
	fn system(s: *const u8) -> isize;
}

fn main() {
	unsafe {
		if cfg!(windows) {
			system(b"echo %date% %time% > target/date.txt".as_ptr());
		} else if cfg!(unix) {
			system(b"date > target/date.txt".as_ptr());
		}
	}
}
