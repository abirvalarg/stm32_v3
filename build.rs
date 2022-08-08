fn main() {
	let mut build = cc::Build::new();
	#[cfg(feature = "async")]
	build.define("ASYNC", "");
	#[cfg(feature = "heap")]
	build.define("HEAP", "");
	build.files([
		"src/init.c",
		"src/irq.c"
	]);
}
