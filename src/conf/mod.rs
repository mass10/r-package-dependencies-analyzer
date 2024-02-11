mod application;

fn usage() {
	let pkg_name = env!("CARGO_PKG_NAME");
	eprintln!("Usage: {}", pkg_name);
}

/// コンフィギュレーション
pub trait Configuration {
	/// アプリケーションを初期化します。
	fn create_application(&self) -> Box<dyn application::Application>;

	/// ディレクトリーのパス
	fn location(&self) -> &str;

	/// キーワード
	fn keywords(&self) -> &Vec<String>;
}

struct ConfigurationImpl {
	/// ディレクトリーのパス
	location: String,

	/// キーワード
	keywords: Vec<String>,
}

impl Configuration for ConfigurationImpl {
	/// アプリケーションを初期化します。
	fn create_application(&self) -> Box<dyn application::Application> {
		let instance = application::ApplicationImpl;
		return Box::new(instance);
	}

	/// ディレクトリーのパス
	fn location(&self) -> &str {
		return &self.location;
	}

	/// キーワード
	fn keywords(&self) -> &Vec<String> {
		return &self.keywords;
	}
}

/// コンフィギュレーションを行います。
pub fn configure() -> Result<Box<dyn Configuration>, Box<dyn std::error::Error>> {
	let args = std::env::args().skip(1).collect::<Vec<String>>();

	let mut options = getopts::Options::new();

	// --help
	options.opt("h", "help", "Show this help.", "bool", getopts::HasArg::No, getopts::Occur::Optional);

	// --location
	options.opt("l", "location", "Location of the directory.", "string", getopts::HasArg::Yes, getopts::Occur::Optional);

	let result = options.parse(&args);
	if result.is_err() {
		usage();
		std::process::exit(1);
	}

	let matches = result.unwrap();
	if matches.opt_present("h") {
		usage();
		return Err("".into());
	}

	// 探索開始位置
	let location = if matches.opt_present("l") { matches.opt_str("l").unwrap() } else { ".".to_string() };

	// キーワード
	let keywords = if matches.free.len() > 0 { matches.free } else { vec![] };

	let instance = ConfigurationImpl {
		location: location,
		keywords: keywords,
	};

	return Ok(Box::new(instance));
}
