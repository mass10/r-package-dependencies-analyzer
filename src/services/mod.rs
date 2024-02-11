//!
//! 各種サービスの実装
//!

pub mod yarn;

/// ファイルハンドラ
pub struct PackageFileAnalyzer {
	/// キーワード
	pub keywords: Vec<String>,
}

impl PackageFileAnalyzer {
	/// パッケージファイルの分析
	pub fn analyze(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		let name = std::path::Path::new(path).file_name().unwrap().to_str().unwrap();
		if name == "yarn.lock" {
			// 分析
			yarn::analyze_yarn_lock(path, &self.keywords)?;
		}

		return Ok(());
	}
}
