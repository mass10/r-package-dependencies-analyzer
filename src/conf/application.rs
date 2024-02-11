//!
//! アプリケーション本体の実装
//!

use crate::conf;
use crate::services;
use crate::util;

/// アプリケーションのトレイト
pub trait Application {
	/// アプリケーションのエントリーポイント
	fn run(&mut self, conf: &dyn conf::Configuration) -> Result<(), Box<dyn std::error::Error>>;
}

/// アプリケーションの実装
pub struct ApplicationImpl;

impl Application for ApplicationImpl {
	/// アプリケーションのエントリーポイント
	fn run(&mut self, conf: &dyn conf::Configuration) -> Result<(), Box<dyn std::error::Error>> {
		// パッケージファイルの分析
		let mut analyzer = services::PackageFileAnalyzer {
			keywords: conf.keywords().clone(),
		};
		let mut handler = move |s: &str| -> Result<(), Box<dyn std::error::Error>> {
			return analyzer.analyze(s);
		};

		// ディレクトリーを再帰的に探索します。
		let search = util::DirectorySearch::new();
		search.find_dir(&conf.location(), &mut handler)?;

		return Ok(());
	}
}
