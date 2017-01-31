extern crate gcc;

fn main() {
	let mut base_config = gcc::Config::new();
	base_config.include(".");
	base_config.include("libusb/libusb");

	if cfg!(target_os = "macos") {
		base_config.define("OS_DARWIN", Some("1"));
		base_config.file("libusb/libusb/os/darwin_usb.c");
		link_framework("CoreFoundation");
		link_framework("IOKit");
	}
	if cfg!(target_os = "linux") {
		base_config.define("OS_LINUX", Some("1"));
		base_config.define("DEFAULT_VISIBILITY", Some("__attribute__((visibility(\"default\")))"));
		base_config.file("libusb/libusb/os/linux_netlink.c");
		base_config.file("libusb/libusb/os/linux_udev.c");
		base_config.file("libusb/libusb/os/linux_usbfs.c");
		base_config.file("libusb/libusb/os/poll_posix.c");
		base_config.file("libusb/libusb/os/threads_posix.c");
	}

	if cfg!(unix) {
		base_config.define("HAVE_DLFCN_H", Some("1"));
		base_config.define("HAVE_GETTIMEOFDAY", Some("1"));
		base_config.define("HAVE_INTTYPES_H", Some("1"));
		base_config.define("HAVE_MEMORY_H", Some("1"));
		base_config.define("HAVE_POLL_H", Some("1"));
		base_config.define("HAVE_STDINT_H", Some("1"));
		base_config.define("HAVE_STDLIB_H", Some("1"));
		base_config.define("HAVE_STRINGS_H", Some("1"));
		base_config.define("HAVE_STRING_H", Some("1"));
		base_config.define("HAVE_STRUCT_TIMESPEC", Some("1"));
		base_config.define("HAVE_SYS_STAT_H", Some("1"));
		base_config.define("HAVE_SYS_TIME_H", Some("1"));
		base_config.define("HAVE_SYS_TYPES_H", Some("1"));
		base_config.define("HAVE_UNISTD_H", Some("1"));
		base_config.define("POLL_NFDS_TYPE", Some("nfds_t"));
		base_config.define("STDC_HEADERS", Some("1"));
		base_config.define("THREADS_POSIX", Some("1"));
		base_config.define("DEFAULT_VISIBILITY", Some("__attribute__((visibility(\"default\")))"));

		base_config.file("libusb/libusb/os/poll_posix.c");
		base_config.file("libusb/libusb/os/threads_posix.c");
	}

	if cfg!(windows) {
		base_config.define("OS_WINDOWS", Some("1"));
		base_config.file("libusb/libusb/os/poll_windows.c");
		base_config.file("libusb/libusb/os/threads_windows.c");
		base_config.file("libusb/libusb/os/windows_nt_common.c");
		base_config.file("libusb/libusb/os/windows_usbdk.c");
		base_config.file("libusb/libusb/os/windows_winusb.c");
		//link("Mswsock", false);
	}

	base_config.file("libusb/libusb/core.c");
	base_config.file("libusb/libusb/descriptor.c");
	base_config.file("libusb/libusb/hotplug.c");
	base_config.file("libusb/libusb/io.c");
	base_config.file("libusb/libusb/strerror.c");
	base_config.file("libusb/libusb/sync.c");


	base_config.compile("libusb.a");
}

pub fn link(name: &str, bundled: bool) {
    use std::env::var;
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        println!("cargo:rustc-link-lib=dylib={}", name);
        if bundled && target.get(3) == Some(&"gnu") {
            let dir = var("CARGO_MANIFEST_DIR").unwrap();
            println!("cargo:rustc-link-search=native={}/{}", dir, target[0]);
        }
    }
}

pub fn link_framework(name: &str) {
	println!("cargo:rustc-link-lib=framework={}", name);
}