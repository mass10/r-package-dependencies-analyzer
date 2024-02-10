mod application;
mod service;

/// Rust アプリケーションのエントリーポイント
fn main() {
	let args = std::env::args().skip(1).collect::<Vec<String>>();
	if args.len() != 1 {
		eprintln!("Usage: ");
		std::process::exit(1);
	}

	// アプリケーションを実行
	let app = application::create_application();
	let result = app.run(&args[0]);
	if result.is_err() {
		eprintln!("Error: {:?}", result.err());
	}
}
