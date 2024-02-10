//!
//! アプリケーション本体の実装
//!

use crate::services;
use crate::util;

/// アプリケーションのトレイト
pub trait Application {
	/// アプリケーションのエントリーポイント
	fn run(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

/// アプリケーションの実装
struct ApplicationImpl;

impl Application for ApplicationImpl {
	/// アプリケーションのエントリーポイント
	fn run(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// パッケージファイルの分析
		let mut analyzer = services::PackageFileAnalyzer {};
		let mut handler = move |s: &str| -> Result<(), Box<dyn std::error::Error>> {
			return analyzer.analyze(s);
		};

		// ディレクトリーを再帰的に探索します。
		let search = util::DirectorySearch::new();
		search.find_dir(path, &mut handler)?;

		return Ok(());
	}
}

/// アプリケーションを初期化します。
pub fn create_application() -> Box<dyn Application> {
	let instance = ApplicationImpl;
	return Box::new(instance);
}
