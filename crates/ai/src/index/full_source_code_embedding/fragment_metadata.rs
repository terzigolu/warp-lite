// warp-lite: ai inert stub (v0.5 phase 2)
//
// Original: persisted metadata mapping leaf merkle hashes to on-disk fragment
// locations. The lightweight fork has no merkle tree, so we keep just the
// `FragmentMetadata` shape — it leaks into `CodebaseIndexingError` and may
// appear in serialized error reporting.

use std::{ops::Range, path::PathBuf};

use serde::{Deserialize, Serialize};
use string_offset::ByteOffset;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentLocation {
    pub start_line: usize,
    /// End line number (inclusive).
    pub end_line: usize,
    /// The range of byte indices into the original source string for this fragment.
    pub byte_range: Range<ByteOffset>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentMetadata {
    pub absolute_path: PathBuf,
    pub location: FragmentLocation,
}

impl FragmentMetadata {
    /// Returns the estimated content size in bytes, derived from the stored byte range.
    pub fn content_byte_size(&self) -> usize {
        self.location
            .byte_range
            .end
            .as_usize()
            .saturating_sub(self.location.byte_range.start.as_usize())
    }
}
