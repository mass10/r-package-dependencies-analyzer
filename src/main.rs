mod application;

/// Rust アプリケーションのエントリーポイント
fn main() {
	let args = std::env::args().skip(1).collect::<Vec<String>>();
	if args.len() != 1 {
		eprintln!("Usage: hello NAME");
		std::process::exit(1);
	}

	// アプリケーションを実行
	let app = application::create_application();
	let result = app.run();
	if result.is_err() {
		eprintln!("Error: {:?}", result.err());
	}
}
