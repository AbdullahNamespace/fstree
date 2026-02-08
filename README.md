<div align="center">

# 🌳 fstree

**File System Tree (fstree) - A blazing fast directory listing command in Rust**

![Crates.io](https://img.shields.io/crates/v/fstree)
![License](https://img.shields.io/crates/l/fstree)
![Rust Edition](https://img.shields.io/badge/edition-2024-blue)
![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Downloads](https://img.shields.io/crates/d/fstree)

</div>

## 🚀 Features

- ⚡ **Blazing Fast:** Built with Rust for maximum performance.
- 🎨 **Smart Icons:** Automatically detects file types (Rust, Python, Images, etc.).
- 🛠️ **Zero Dependencies:** A single static binary.

## 📦 Installation

```bash
cargo install fstree
```

Or build from source:

```bash
git clone https://github.com/your-username/fstree.git
cd fstree
cargo install --path .
```

## 💻 Usage & Examples

### Example 1: Basic Directory Listing

```bash
# List current directory
$ fstree
.
├── 📂 src
│   ├── 🦀 main.rs
│   ├── 🦀 lib.rs
│   └── 📂 utils
├── 📄 Cargo.toml
├── 📝 README.md
├── 📂 tests
│   └── 🧪 test_basic.rs
└── 📦 target
```

---

### Example 2: Limit Depth

```bash
# Show only 2 levels deep
$ fstree -d 2
.
├── 📂 src
│   ├── 📂 config
│   │   └── 📋 settings.json
│   ├── 📄 main.rs
│   └── 🦀 lib.rs
├── 📄 Cargo.toml
├── 📝 README.md
└── 📦 target
```

---

### Example 3: Show Hidden Files

```bash
# Include files starting with .
$ fstree -a
.
├── 📂 .git
│   ├── 📦 HEAD
│   ├── 📦 config
│   └── 📂 hooks
├── 📂 .vscode
│   └── ⚙️ settings.json
├── 📄 .gitignore
├── 📂 src
│   └── 🦀 main.rs
└── 📝 README.md
```

---

### Example 4: Target Specific Path

```bash
# List another directory
$ fstree -p ./my-project
my-project
├── 🌐 index.html
├── 🎨 style.css
├── 📜 app.js
├── 📂 api
│   └── 🦀 main.rs
└── 📄 package.json
```

### Example 5: Exclude Files/Folders

```bash
# Exclude single folder
$ fstree -e node_modules
.
├── 📂 src
│   └── 🦀 main.rs
├── 📄 package.json
└── 📝 README.md

# Exclude multiple items
$ fstree -e node_modules -e .git -e dist
.
├── 📂 src
├── 📄 Cargo.toml
└── 📝 README.md

# Exclude with other options
$ fstree -d 2 -a -e .vscode -e .github
```

## ⚙️ Options Reference

| Option                  | Description           | Example                  |
| ----------------------- | --------------------- | ------------------------ |
| `-p, --path PATH`       | Target directory      | `fstree -p ./src`        |
| `-d, --depth NUM`       | Limit depth levels    | `fstree -d 3`            |
| `-a, --all`             | Show hidden files     | `fstree -a`              |
| `-e, --exclude PATTERN` | Exclude files/folders | `fstree -e node_modules` |
| `-h, --help`            | Show help             | `fstree --help`          |
| `-V, --version`         | Show version          | `fstree --version`       |

## 📄 License

MIT License

---

<div align="center">
  <b>Made with ❤️ and 🦀</b>
</div>
