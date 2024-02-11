pub fn unquote(s: &str) -> &str {
	if s.starts_with("\"") && s.ends_with("\"") {
		return &s[1..s.len() - 1];
	}
	return s;
}

/// ファイルハンドラーの型定義です。
pub type FileHandler = dyn FnMut(&str) -> Result<(), Box<dyn std::error::Error>> + 'static;

pub struct DirectorySearch {
	/// 除外ディレクトリーの定義
	exclude_directories: std::collections::BTreeSet<String>,
}

impl DirectorySearch {
	/// 新しいインスタンスを返します。
	pub fn new() -> DirectorySearch {
		let mut exclude_directories = std::collections::BTreeSet::new();
		exclude_directories.insert(".git".to_string());
		exclude_directories.insert("node_modules".to_string());
		exclude_directories.insert("target".to_string());

		return DirectorySearch {
			exclude_directories: exclude_directories,
		};
	}

	/// ディレクトリーを再帰的に探索します。
	///
	/// # Arguments
	/// * `path` - 探索するディレクトリーのパス
	/// * `handler` - ファイルハンドラー
	pub fn find_dir(&self, path: &str, handler: &mut FileHandler) -> Result<(), Box<dyn std::error::Error>> {
		let metadata = std::fs::metadata(path)?;
		if metadata.is_dir() {
			let name = std::path::Path::new(path).file_name().unwrap_or_default().to_str().unwrap();
			if self.exclude_directories.contains(name) {
				return Ok(());
			}
			let entries = std::fs::read_dir(path)?;
			for entry in entries {
				let entry = entry?;
				let path = entry.path();
				self.find_dir(&path.display().to_string(), handler)?;
			}
		} else if metadata.is_file() {
			handler(path)?;
		}

		return Ok(());
	}
}
