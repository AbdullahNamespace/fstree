use crate::config::TreeConfig;
use crate::icons;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

struct Node {
    name: String,
    is_dir: bool,
    is_symlink: bool,
    children: Vec<Node>,
}

pub struct Generator {
    config: TreeConfig,
}

impl Generator {
    pub fn new(config: TreeConfig) -> Self {
        Self { config }
    }

    pub fn run(&self) -> Result<()> {
        let root_path = &self.config.path;

        if !root_path.exists() {
            anyhow::bail!("Path not found: {}", root_path.display());
        }

        let root = self.build_tree(root_path, 0)?;
        self.print_tree(&root, &"".to_string(), true);

        Ok(())
    }

    fn build_tree(&self, path: &Path, current_depth: usize) -> Result<Node> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?")
            .to_string();

        let metadata = fs::symlink_metadata(path)
            .with_context(|| format!("Failed to read metadata: {}", path.display()))?;

        let file_type = metadata.file_type();
        let is_symlink = file_type.is_symlink();
        let is_dir = file_type.is_dir();

        let mut children = Vec::new();

        if is_dir && !is_symlink {
            if let Some(max_depth) = self.config.depth {
                if current_depth >= max_depth {
                    return Ok(Node {
                        name,
                        is_dir: true,
                        is_symlink: false,
                        children,
                    });
                }
            }

            let entries = fs::read_dir(path)
                .with_context(|| format!("Failed to read directory: {}", path.display()))?;

            let mut nodes: Vec<Node> = Vec::new();
            for entry in entries {
                let entry = entry?;
                let entry_path = entry.path();

                if !self.config.all {
                    if let Some(file_name) = entry_path.file_name() {
                        if file_name
                            .to_str()
                            .map(|s| s.starts_with('.'))
                            .unwrap_or(false)
                        {
                            continue;
                        }
                    }
                }

                match self.build_tree(&entry_path, current_depth + 1) {
                    Ok(node) => nodes.push(node),
                    Err(e) => eprintln!(
                        "{} Skipping {}: {}",
                        "[WARN]".yellow(),
                        entry_path.display(),
                        e
                    ),
                }
            }

            nodes.sort_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            children = nodes;
        }

        Ok(Node {
            name,
            is_dir,
            is_symlink,
            children,
        })
    }

    fn print_tree(&self, node: &Node, prefix: &str, is_last: bool) {
        let connector = if is_last { "└── " } else { "├── " };

        let icon = if node.is_symlink {
            "🔗"
        } else {
            icons::get_icon_for_filename(&node.name, node.is_dir)
        };

        let name_display = if node.is_symlink {
            node.name.cyan()
        } else {
            node.name.white()
        };

        println!("{}{}{} {}", prefix, connector, icon, name_display);

        let new_prefix = format!("{}{} ", prefix, if is_last { "   " } else { "│  " });

        let children_count = node.children.len();
        for (index, child) in node.children.iter().enumerate() {
            let is_last_child = index == children_count - 1;
            self.print_tree(child, &new_prefix, is_last_child);
        }
    }
}
