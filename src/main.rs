use std::io::BufRead;

fn usage() {
	let pkg_name = env!("CARGO_PKG_NAME");
	eprintln!("Usage: {} NAME", pkg_name);
	std::process::exit(1);
}

fn start_with_alpha(s: &str) -> bool {
	// c.is_ascii_alphabetic() || c == '_'

	// 1文字目がアルファベットかどうか
	match s.chars().next() {
		Some(c) => c.is_ascii_alphabetic(),
		None => false,
	}
}

fn starts_with_package(line: &str) -> bool {
	return start_with_alpha(line);
}

fn analyze_packages(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	find_dir(path, None)?;

	return Ok(());
}

fn is_directory(path: &str) -> bool {
	let path = std::path::Path::new(path);
	return path.is_dir();
}

fn is_file(path: &str) -> bool {
	let path = std::path::Path::new(path);
	return path.is_file();
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

type FileHandler = dyn Fn(&str) -> Result<(), Box<dyn std::error::Error>> + 'static;

/// yarn.lock の分析
fn analyze_yarn_lock(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	// println!("$ file: {}", path);

	let file = std::fs::File::open(path)?;
	let reader = std::io::BufReader::new(file);

	// trim handler
	let trimmer = |s: &str| -> String { s.trim().to_string() };

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
		} else if current_section == "" {
			// グローバルなセクション
			if starts_with_package(&line) && line.ends_with(":") {

				let items = line.split(",").map(trimmer).collect::<Vec<String>>();
                current_packages.clear();
                for item in items {
                    current_packages.push(item);
                }

				current_section = "package";
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
                set.insert(item.0.to_string());
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

fn find_dir(path: &str, handler: Option<&FileHandler>) -> Result<(), Box<dyn std::error::Error>> {
	let metadata = std::fs::metadata(path)?;
	if metadata.is_dir() {
		let name = std::path::Path::new(path).file_name().unwrap().to_str().unwrap();
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
		// file name.
		let name = std::path::Path::new(path).file_name().unwrap().to_str().unwrap();
		if name == "yarn.lock" {
			analyze_yarn_lock(path)?;
		}
	}
	Ok(())
}

fn main() {
	let args = std::env::args().skip(1).collect::<Vec<String>>();
	if args.len() != 1 {
	    eprintln!("Usage: hello NAME");
	    std::process::exit(1);
	}

	let result = analyze_packages(&args[0]);
    if result.is_err() {
        eprintln!("Error: {:?}", result.err());
    }
}
