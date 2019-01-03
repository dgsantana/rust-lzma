extern crate vcpkg;

fn main() {
	vcpkg::find_package("liblzma").unwrap();
}
