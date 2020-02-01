//! Filter out OS junk files like .DS_Store and Thumbs.db
//!
//! # Examples
//! ```
//! use junk;
//!
//! junk::is(".test.swp"); // Check if filename is junk.
//! junk::not(".test.swp"); // Check if filename is not junk.
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

use lazy_static::lazy_static;
use regex::Regex as Rgx;

/// Regex used for matching junk files.
pub static REGEX: &str = concat!(
  // # All
  r"^npm-debug\.log$|", // Error log for npm
  r"^\..*\.swp$|",      // Swap file for vim state
  // # macOS
  r"^\.DS_Store$|",            // Stores custom folder attributes
  r"^\.AppleDouble$|",         // Stores additional file resources
  r"^\.LSOverride$|",          // Contains the absolute path to the app to be used
  r"^Icon\r$|", // Custom Finder icon: http://superuser.com/questions/298785/icon-file-on-os-x-desktop
  r"^\._.*|",   // Thumbnail
  r"^.Spotlight-V100(?:$|/)|", // Directory that might appear on external disk
  r"\.Trashes|", // File that might appear on external disk
  r"^__MACOSX$|", // Resource fork
  // # Linux
  r"~$|", // Backup file
  // # Windows
  r"^Thumbs\.db$|",   // Image file cache
  r"^ehthumbs\.db$|", // Folder config file
  r"^Desktop\.ini$|", // Stores custom folder attributes
  r"@eaDir$",         // Synology Diskstation "hidden" folder where the server stores thumbnails
);

/// Returns true if filename matches a junk file.
pub fn is(filename: &str) -> bool {
  lazy_static! {
    static ref RE: Rgx = Rgx::new(REGEX).unwrap();
  }
  RE.is_match(filename)
}

/// Returns true if filename doesn't match a junk file.
pub fn not(filename: &str) -> bool {
  !is(filename)
}
