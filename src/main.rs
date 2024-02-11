mod conf;
mod services;
mod util;

/// エラーを報告します。
fn report_error(error: Box<dyn std::error::Error>) {
	let message = error.to_string();
	if message == "" {
		return;
	}
	eprintln!("Error: {}", message);
}

/// Rust アプリケーションのエントリーポイント
fn main() {
	// コンフィギュレーション
	let result = conf::configure();
	if result.is_err() {
		report_error(result.err().unwrap());
		std::process::exit(1);
	}
	let conf = result.unwrap();
	let conf = &(*conf);

	// アプリケーションを実行
	let mut app = conf.create_application();
	let result = app.run(conf);
	if result.is_err() {
		report_error(result.err().unwrap());
		std::process::exit(1);
	}
}
