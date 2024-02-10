//!
//! アプリケーション本体の実装
//!

use crate::service;

#[allow(unused)]
fn usage() {
	let pkg_name = env!("CARGO_PKG_NAME");
	eprintln!("Usage: {} NAME", pkg_name);
	std::process::exit(1);
}

/// ファイルハンドラ
struct YarnFileAnalyzer;

impl YarnFileAnalyzer {
	/// yarn.lock の分析
	pub fn analyze(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		let name = std::path::Path::new(path).file_name().unwrap().to_str().unwrap();
		if name != "yarn.lock" {
			return Ok(());
		}

		// 分析
		service::analyze_yarn_lock(path)?;

		return Ok(());
	}
}

type FileHandler = dyn FnMut(&str) -> Result<(), Box<dyn std::error::Error>> + 'static;

fn find_dir(path: &str, handler: &mut FileHandler) -> Result<(), Box<dyn std::error::Error>> {
	let metadata = std::fs::metadata(path)?;
	if metadata.is_dir() {
		let name = std::path::Path::new(path).file_name().unwrap().to_str().unwrap();
		if name == ".git" {
			return Ok(());
		}
		if name == "node_modules" {
			return Ok(());
		}
		if name == "tartget" {
			return Ok(());
		}
		let entries = std::fs::read_dir(path)?;
		for entry in entries {
			let entry = entry?;
			let path = entry.path();
			find_dir(&path.display().to_string(), handler)?;
		}
	} else if metadata.is_file() {
		handler(path)?;
	}

	return Ok(());
}

pub trait Application {
	/// アプリケーションのエントリーポイント
	fn run(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

struct ApplicationImpl;

impl Application for ApplicationImpl {
	/// アプリケーションのエントリーポイント
	fn run(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// パッケージファイルの分析
		let mut handler_impl = YarnFileAnalyzer {};
		let mut handler = move |s: &str| -> Result<(), Box<dyn std::error::Error>> {
			return handler_impl.analyze(s);
		};

		// ディレクトリーを再帰的に探索します。
		find_dir(path, &mut handler)?;

		return Ok(());
	}
}

pub fn create_application() -> Box<dyn Application> {
	let instance = ApplicationImpl;
	return Box::new(instance);
}