use junk;

const FIXTURE: &'static [&'static str] = &[
  ".DS_Store",
  ".AppleDouble",
  ".LSOverride",
  "Icon\r",
  "._test",
  ".Spotlight-V100",
  ".Spotlight-V100/Store-V2/C6DBF25D-81D4-4B57-907E-B4A555E72C90/0.directoryStoreFile",
  ".Trashes",
  "__MACOSX",
  "test~",
  "Thumbs.db",
  "ehthumbs.db",
  "Desktop.ini",
  "npm-debug.log",
  ".test.swp",
  "@eaDir",
];

const NOT_FIXTURE: &'static [&'static str] =
  &["test", "Icon", "Icons.woff", ".Spotlight-V100-unicorn"];

#[test]
fn is_junk() {
  for element in FIXTURE.into_iter() {
    assert!(junk::is(&element))
  }
}

#[test]
fn is_not_junk() {
  for element in NOT_FIXTURE.into_iter() {
    assert!(junk::not(&element))
  }
}

#[test]
fn assert_regex() {
  assert_eq!(
    junk::REGEX,
    "^npm-debug\\.log$|^\\..*\\.swp$|^\\.DS_Store$|^\\.AppleDouble$|^\\.LSOverride$|^Icon\\r$|^\\._.*|^.Spotlight-V100(?:$|/)|\\.Trashes|^__MACOSX$|~$|^Thumbs\\.db$|^ehthumbs\\.db$|^Desktop\\.ini$|@eaDir$"
  );
}
