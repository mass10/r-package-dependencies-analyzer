//!
//! Yarn ファイルの分析
//!

use std::io::BufRead;

use crate::util;

/// yarn.lock の分析
pub fn analyze_yarn_lock(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	let file = std::fs::File::open(path)?;
	let reader = std::io::BufReader::new(file);
	let trimmer = &str::trim;

	// 依存パッケージのツリー
	let mut package_tree: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> = std::collections::BTreeMap::new();

	let mut current_packages: Vec<String> = Vec::new();

	// 現在のセクション
	let mut current_section = "";

	for line in reader.lines() {
		let line = line.unwrap();
		if line == "" {
			// セクションの終わり
			current_section = "";
		} else if line.starts_with("#") {
			current_section = "";
		} else if current_section == "" {
			// グローバルなセクション
			if line.starts_with(" ") {
				// NOP (optionalDependencies など)
			} else if line.ends_with(":") {
				let line = line.trim_end_matches(":");
				let items = line.split(",").map(trimmer).map(util::unquote).collect::<Vec<&str>>();
				if items.len() == 0 {
					eprintln!("[ERROR] (analyze_yarn_lock): {}", line);
					return Err("".into());
				}

				current_packages.clear();
				for item in items {
					validate_package_name(item)?;
					current_packages.push(item.to_string());

					// パッケージツリーの親を作る
					if !package_tree.contains_key(item) {
						package_tree.insert(item.to_string(), std::collections::BTreeSet::new());
					}
				}

				current_section = "package";
			} else {
				eprintln!("[ERROR] (analyze_yarn_lock) 不明な行: {}", line);
				return Err("".into());
			}
		} else if current_section == "package" {
			// パッケージのセクション
			if line == "  dependencies:" {
				current_section = "dependencies";
			}
		} else if current_section == "dependencies" {
			if line == "  optionalDependencies:" {
				// 依存パッケージのセクション 終わり
				current_section = "";
				continue;
			}

			// 依存パッケージのセクション
			let item = parse_dependency_line(&line)?;

			for package in current_packages.iter() {
				// キーを持っていない場合は追加
				if !package_tree.contains_key(package) {
					package_tree.insert(package.to_string(), std::collections::BTreeSet::new());
				}

				// dependencies を取得して
				let set = package_tree.get_mut(package).unwrap();

				// 追加
				let value = format!("{}@{}", item.0, item.1);
				set.insert(value);
			}
		}
	}

	// 依存パッケージのサマリーを表示
	for (package, dependencies) in package_tree.iter() {
		println!("{}:", package);
		for dependency in dependencies.iter() {
			println!("    {}", dependency);
		}
	}

	return Ok(());
}

type DependencyPackage = (String, String);

/// 依存パッケージの行を解析
fn parse_dependency_line(line: &str) -> Result<DependencyPackage, Box<dyn std::error::Error>> {
	// 依存パッケージの行を解析
	let regex = regex::Regex::new("^    \"?([@\\/a-zA-Z0-9_\\-\\.]+)\"? \"(.+)\"$")?;

	let result = regex.captures(line);
	if result.is_none() {
		// 即時終了
		eprintln!("[ERROR] (parse_dependency_line): {}", line);
		return Err("".into());
	}

	let captures = result.unwrap();
	let name = captures.get(1).unwrap().as_str();
	let version = captures.get(2).unwrap().as_str();

	return Ok((name.to_string(), version.to_string()));
}

/// パッケージ名のバリデーション
fn validate_package_name(name: &str) -> Result<(), Box<dyn std::error::Error>> {
	let regex = regex::Regex::new("^\"?[a-zA-Z0-9_@\\-\\.~^<>=\\/* |:]+\"?$")?;
	if regex.is_match(name) {
		return Ok(());
	}
	eprintln!("[ERROR] (validate_package_name): {}", name);
	return Err("".into());
}
