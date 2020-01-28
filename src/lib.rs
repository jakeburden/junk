//! Filter out OS junk files like .DS_Store and Thumbs.db
//!
//! # Examples
//! ```
//! use junk;
//!
//! junk::is(&path); // Check if filename is junk.
//! junk::not(&path); // Check if filename is not junk.
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

use regex::Regex;

const BLACKLIST: &str = concat!(
    // # All
    "^npm-debug\\.log$|", // Error log for npm
    "^\\..*\\.swp$|",     // Swap file for vim state
    // # macOS
    "^\\.DS_Store$|",           // Stores custom folder attributes
    "^\\.AppleDouble$|",        // Stores additional file resources
    "^\\.LSOverride$|",         // Contains the absolute path to the app to be used
    "^Icon\\r$|", // Custom Finder icon: http://superuser.com/questions/298785/icon-file-on-os-x-desktop
    "^\\._.*|",   // Thumbnail
    "^.Spotlight-V100(?:$|/)|", // Directory that might appear on external disk
    "\\.Trashes|", // File that might appear on external disk
    "^__MACOSX$|", // Resource fork
    // # Linux
    "~$|", // Backup file
    // # Windows
    "^Thumbs\\.db$|",   // Image file cache
    "^ehthumbs\\.db$|", // Folder config file
    "^Desktop\\.ini$|", // Stores custom folder attributes
    "@eaDir$",          // Synology Diskstation "hidden" folder where the server stores thumbnails
);

/// Tests is a filename is a junk file.
pub fn is(filename: &str) -> bool {
    let re: Regex = Regex::new(BLACKLIST).unwrap();
    re.is_match(filename)
}

/// Tests is a filename is not a junk file.
pub fn not(filename: &str) -> bool {
    !is(filename)
}
