mod application;
mod services;
mod util;

/// 使用方法
fn usage() {
	let pkg_name = env!("CARGO_PKG_NAME");
	eprintln!("Usage: {}", pkg_name);
}

/// Rust アプリケーションのエントリーポイント
fn main() {
	let args = std::env::args().skip(1).collect::<Vec<String>>();
	if args.len() != 1 {
		usage();
		std::process::exit(1);
	}

	// アプリケーションを実行
	let mut app = application::create_application();
	let result = app.run(&args[0]);
	if result.is_err() {
		let error = result.err().unwrap();
		let message: String = error.to_string();
		if message == "" {
			std::process::exit(1);
		}
		eprintln!("Error: {:?}", error);
		std::process::exit(1);
	}
}
