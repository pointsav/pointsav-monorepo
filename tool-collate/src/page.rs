// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! One page as read: its frontmatter state and the structure of its body. A `Page` is a fact about
//! what a file contains, not a verdict on whether it is right.

use crate::frontmatter::{Frontmatter, FrontmatterState};
use crate::markdown::{self, CodeBlock, Heading, MarkdownLink, Table};

#[derive(Debug, Clone)]
pub struct Page {
    /// Path relative to the folder that was scanned, with `/` separators.
    pub path: String,
    pub frontmatter: FrontmatterState,
    pub body: String,
    pub headings: Vec<Heading>,
    /// Links outside fenced code blocks (a link shown as an example is not a link).
    pub links: Vec<MarkdownLink>,
    pub code_blocks: Vec<CodeBlock>,
    pub tables: Vec<Table>,
}

impl Page {
    /// Read one page's text. Never fails: a page with no or bad frontmatter is still read for its
    /// structure, and the state says what was found.
    pub fn parse(path: &str, content: &str) -> Page {
        let (frontmatter, body) = Frontmatter::parse(content);
        let body = body.to_string();
        let code_blocks = markdown::code_blocks(&body);
        let links = markdown::markdown_links(&body)
            .into_iter()
            .filter(|l| {
                !code_blocks
                    .iter()
                    .any(|b| l.offset >= b.offset && l.offset < b.end)
            })
            .collect();
        Page {
            path: path.to_string(),
            frontmatter,
            headings: markdown::headings(&body),
            links,
            tables: markdown::tables(&body),
            code_blocks,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_structure_even_without_frontmatter() {
        let p = Page::parse("n.md", "# Title\n\n[a](b.md)\n");
        assert!(matches!(p.frontmatter, FrontmatterState::Missing));
        assert_eq!(p.headings.len(), 1);
        assert_eq!(p.links.len(), 1);
    }

    #[test]
    fn a_link_inside_a_code_block_is_not_a_link() {
        let p = Page::parse(
            "n.md",
            "```md\n[example](nowhere.md)\n```\n\n[real](here.md)\n",
        );
        assert_eq!(p.links.len(), 1);
        assert_eq!(p.links[0].url, "here.md");
        assert_eq!(p.code_blocks.len(), 1);
    }
}
