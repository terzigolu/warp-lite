// warp-lite: ai inert stub (v0.5 phase 2)
//
// Original: tree-sitter driven semantic chunker (with a naive line-based
// fallback). The lightweight fork has no embedding pipeline, so we keep just
// the `Fragment<'a>` shape — `merkle_tree::hash::ContentHash::from_fragment`
// references it. `chunk_code` always returns an empty vec.

use std::path::Path;

use string_offset::ByteOffset;

#[derive(Debug, Clone)]
pub struct Fragment<'a> {
    pub content: &'a str,
    pub start_line: usize,
    pub end_line: usize,
    pub start_byte_index: ByteOffset,
    pub end_byte_index: ByteOffset,
    pub file_path: &'a Path,
}

#[allow(dead_code)]
pub fn chunk_code<'a>(_code: &'a str, _path: &'a Path) -> Vec<Fragment<'a>> {
    Vec::new()
}
