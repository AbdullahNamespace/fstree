use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static FILE_ICONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert("rs", "🦀");
    m.insert("rlib", "📚");
    m.insert("py", "🐍");
    m.insert("pyc", "⚡");
    m.insert("pyo", "⚡");
    m.insert("pyd", "🔧");
    m.insert("pyx", "🚀");
    m.insert("whl", "🎡");
    m.insert("js", "📜");
    m.insert("ts", "💠");
    m.insert("jsx", "⚛️");
    m.insert("tsx", "⚛️💠");
    m.insert("vue", "🟢");
    m.insert("svelte", "🌀");
    m.insert("java", "☕");
    m.insert("class", "⚙️");
    m.insert("jar", "🏺");
    m.insert("gradle", "🐘");
    m.insert("kt", "🎯");
    m.insert("kts", "🎯");
    m.insert("dart", "🎯");
    m.insert("c", "🔤");
    m.insert("cpp", "🔷");
    m.insert("cc", "🔷");
    m.insert("cxx", "🔷");
    m.insert("h", "📄");
    m.insert("hpp", "📄");
    m.insert("hxx", "📄");
    m.insert("go", "🐹");
    m.insert("mod", "📦");
    m.insert("rb", "💎");
    m.insert("erb", "💎🌐");
    m.insert("gem", "💎📦");
    m.insert("php", "🐘");
    m.insert("phtml", "🐘🌐");
    m.insert("swift", "🍎");

    m.insert("xcconfig", "🍎⚙️");
    m.insert("plist", "🍎📋");
    m.insert("strings", "🍎🔤");

    m.insert("sh", "🐚");
    m.insert("bash", "🐚");
    m.insert("zsh", "🐚");
    m.insert("fish", "🐟");
    m.insert("ps1", "💻");
    m.insert("sql", "🗄️");
    m.insert("db", "💾");
    m.insert("sqlite", "💾");
    m.insert("mdb", "💾");
    m.insert("html", "🌐");
    m.insert("htm", "🌐");
    m.insert("css", "🎨");
    m.insert("scss", "🎨");
    m.insert("sass", "🎨");
    m.insert("less", "🎨");
    m.insert("styl", "🎨");
    m.insert("ejs", "📝");
    m.insert("handlebars", "✋");
    m.insert("hbs", "✋");
    m.insert("pug", "🐶");
    m.insert("jade", "🐶");
    m.insert("htaccess", "⚙️");
    m.insert("htpasswd", "🔒");
    m.insert("lock", "🔒");
    m.insert("cs", "🔷");
    m.insert("vb", "🔤");
    m.insert("fs", "🎯");
    m.insert("m", "🔷");
    m.insert("mm", "🔷");
    m.insert("hs", "λ");
    m.insert("lhs", "λ");
    m.insert("elm", "🌳");
    m.insert("clj", "🟣");
    m.insert("cljs", "🟣");
    m.insert("scala", "⚡");
    m.insert("erl", "🐇");
    m.insert("pl", "🐪");
    m.insert("pm", "🐪");
    m.insert("lua", "🌙");
    m.insert("r", "📊");
    m.insert("jl", "🔷");
    m.insert("asm", "💾");
    m.insert("s", "💾");
    m.insert("inc", "📄");
    m.insert("json", "📋");
    m.insert("yaml", "📝");
    m.insert("yml", "📝");
    m.insert("toml", "⚙️");
    m.insert("xml", "📋");
    m.insert("env", "🔐");
    m.insert("conf", "⚙️");
    m.insert("ini", "⚙️");
    m.insert("cfg", "⚙️");
    m.insert("properties", "⚙️");
    m.insert("csv", "📊");
    m.insert("tsv", "📊");
    m.insert("xlsx", "📊");
    m.insert("xls", "📊");
    m.insert("ods", "📊");
    m.insert("parquet", "📊");
    m.insert("proto", "📄");
    m.insert("avro", "📊");
    m.insert("md", "📝");
    m.insert("markdown", "📝");
    m.insert("txt", "📄");
    m.insert("rtf", "📄");
    m.insert("tex", "📜");
    m.insert("bib", "📚");
    m.insert("pdf", "📕");
    m.insert("docx", "📘");
    m.insert("doc", "📘");
    m.insert("pptx", "📽️");
    m.insert("ppt", "📽️");
    m.insert("odt", "📄");
    m.insert("epub", "📖");
    m.insert("mobi", "📖");
    m.insert("azw3", "📖");
    m.insert("png", "🖼️");
    m.insert("jpg", "🖼️");
    m.insert("jpeg", "🖼️");
    m.insert("gif", "🖼️");
    m.insert("webp", "🖼️");
    m.insert("bmp", "🖼️");
    m.insert("ico", "🖼️");
    m.insert("svg", "🎨");
    m.insert("ai", "🎨");
    m.insert("eps", "🎨");
    m.insert("psd", "🎨");
    m.insert("sketch", "🎨");
    m.insert("blend", "🎨");
    m.insert("obj", "🎨");
    m.insert("fbx", "🎨");
    m.insert("stl", "🎨");
    m.insert("mp4", "🎬");
    m.insert("mkv", "🎬");
    m.insert("avi", "🎬");
    m.insert("mov", "🎬");
    m.insert("wmv", "🎬");
    m.insert("flv", "🎬");
    m.insert("webm", "🎬");
    m.insert("m4v", "🎬");
    m.insert("mp3", "🎵");
    m.insert("wav", "🎵");
    m.insert("flac", "🎵");
    m.insert("aac", "🎵");
    m.insert("ogg", "🎵");
    m.insert("m4a", "🎵");
    m.insert("wma", "🎵");
    m.insert("srt", "📜");
    m.insert("ass", "📜");
    m.insert("vtt", "📜");
    m.insert("zip", "📦");
    m.insert("tar", "📦");
    m.insert("gz", "📦");
    m.insert("bz2", "📦");
    m.insert("xz", "📦");
    m.insert("rar", "📦");
    m.insert("7z", "📦");
    m.insert("deb", "📦");
    m.insert("rpm", "📦");
    m.insert("apk", "📦");
    m.insert("pkg", "📦");
    m.insert("msi", "📦");
    m.insert("npmignore", "📦");
    m.insert("gitignore", "📦");
    m.insert("gitattributes", "📦");
    m.insert("dockerignore", "🐳");
    m.insert("dockerfile", "🐳");
    m.insert("makefile", "🔨");
    m.insert("mk", "🔨");
    m.insert("cmake", "🔧");
    m.insert("bazel", "🔧");
    m.insert("git", "📦");
    m.insert("po", "🌍");
    m.insert("mo", "🌍");
    m.insert("chm", "📚");
    m.insert("hlp", "📚");
    m.insert("pem", "🔐");
    m.insert("crt", "🔐");
    m.insert("key", "🔐");
    m.insert("csr", "🔐");
    m.insert("pfx", "🔐");
    m.insert("p12", "🔐");
    m
});

pub fn get_file_icon(extension: &str) -> &'static str {
    FILE_ICONS.get(extension).copied().unwrap_or("📄")
}

pub fn extract_extension(filename: &str) -> Option<&str> {
    let name = if filename.starts_with('.') {
        &filename[1..]
    } else {
        filename
    };
    name.rsplit('.').next()
}

pub fn get_icon_for_filename(filename: &str, is_dir: bool) -> &'static str {
    if is_dir {
        return get_folder_icon(filename);
    }

    let trimmed_name = filename.trim();
    let lower_name = trimmed_name.to_lowercase();

    match lower_name.as_str() {
        "dockerfile" => "🐳",
        "makefile" => "🔨",
        "cmakelists.txt" => "🔧",
        "gradlew" => "🐘",
        "pubspec.yaml" => "🎯",
        "pubspec.lock" => "🔒",
        "readme" => "📖",
        "license" => "⚖️",
        _ => {
            if lower_name.contains("test") {
                if let Some(ext) = extract_extension(trimmed_name) {
                    match ext {
                        "rs" | "py" | "js" | "ts" | "dart" | "go" | "java" | "kt" | "swift"
                        | "cpp" | "c" | "rb" | "php" => {
                            return "🧪";
                        }
                        _ => {}
                    }
                }
            }

            if let Some(ext) = extract_extension(trimmed_name) {
                get_file_icon(ext)
            } else {
                "📄"
            }
        }
    }
}

pub fn get_folder_icon(folder_name: &str) -> &'static str {
    let lower_name = folder_name.to_lowercase();
    match lower_name.as_str() {
        ".git" | ".github" => "🐙",
        ".vscode" => "💻",
        ".idea" => "💡",
        "node_modules" => "📦",
        "vendor" => "📦",
        "target" => "🎯",
        "build" => "🔨",
        "dist" | "out" => "📤",
        "bin" => "⚙️",
        "lib" => "📚",
        "src" => "📁",
        "tests" | "test" | "__tests__" => "🧪",
        "docs" => "📚",
        "public" => "🌐",
        "static" => "🌐",
        "assets" | "images" | "img" => "🖼️",
        "fonts" => "🔤",
        "migrations" => "🚀",
        "config" | "cfg" => "⚙️",
        _ => "📂",
    }
}
