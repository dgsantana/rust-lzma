fn main() {
	let target = std::env::var("TARGET").unwrap();
	if target.contains("windows") {
		vcpkg::find_package("liblzma").unwrap();
	} else {
		pkg_config::Config::new().probe("liblzma").unwrap();
	}
}
