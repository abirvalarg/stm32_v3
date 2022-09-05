fn main() {
	let mut build = cc::Build::new();
	#[cfg(feature = "async")]
	build.define("ASYNC", "1");
	#[cfg(feature = "heap")]
	build.define("HEAP", "1");

	#[cfg(feature = "f4")]
	build.define("F4", "1");
	#[cfg(feature = "f40")]
	build.define("F40", "1");
	#[cfg(feature = "f401")]
	build.define("F401", "1");
	#[cfg(feature = "f405")]
	build.define("F405", "1");
	#[cfg(feature = "f407")]
	build.define("F407", "1");

	build.files([
		"src/init.c",
		"src/irq.c"
	]);
	build.compile("c-part.a");
}
