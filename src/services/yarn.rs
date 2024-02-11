//!
//! Yarn ファイルの分析
//!

use std::io::BufRead;

use crate::util;

fn summary_package_tree(
	ancestor: &Vec<String>,
	package_tree: &std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
	package: &str,
	depth: usize,
	keywords: &Vec<String>,
) -> Result<bool, Box<dyn std::error::Error>> {
	// 検索キーワードによるマッチング
	for keyword in keywords {
		if package.contains(keyword) {
			let mut index = 0;
			for dep in ancestor {
				println!("{}{}", "\x09".repeat(index), dep);
				index += 1;
			}
			println!("{}{}", "\x09".repeat(index), package);
			return Ok(true);
		}
	}

	// ループを検出したら即時終了
	if ancestor.contains(&package.to_string()) {
		// let indent = "\x09".repeat(depth);
		// println!("{}{}: (LOOP DETECTED)", indent, package);
		return Ok(false);
	}

	// 20 レベル以上の深さは表示しない
	if 50 <= depth {
		let prefix = "\x09".repeat(depth);
		println!("{}... (too deep)", &prefix);
		return Err("".into());
	}

	if package == "" {
		for (package, dependencies) in package_tree.iter() {
			// println!("- {}:", package);
			// 依存パッケージを表示
			for dep in dependencies.iter() {
				// summary_package_tree には正しい祖先情報を渡す
				let mut ancestor: Vec<String> = Vec::new();
				ancestor.push(package.to_string());

				let _found = summary_package_tree(&ancestor, package_tree, dep, depth + 1, keywords)?;

				// 結果は無視してすべて走査
			}
		}

		return Ok(true);
	} else {
		// let prefix = "\x09".repeat(depth);
		// println!("{}{}:", prefix, package);
		let result = package_tree.get(package);
		if result.is_none() {
			return Ok(false);
		}
		let dependencies = result.unwrap();
		// 依存パッケージを表示
		for dep in dependencies.iter() {
			// summary_package_tree には正しい祖先情報を渡す
			let mut ancestor = ancestor.clone();
			ancestor.push(package.to_string());

			let _result = summary_package_tree(&ancestor, package_tree, dep, depth + 1, keywords)?;
		}

		return Ok(false);
	}
}

/// yarn.lock の分析
pub fn analyze_yarn_lock(path: &str, keywords: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
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
	let path: Vec<String> = Vec::new();
	let _result = summary_package_tree(&path, &package_tree, "", 0, &keywords)?;
	// summary_package_tree(&package_tree, "keyv@^3.0.0", 0);

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
