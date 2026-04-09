use include_dir::{include_dir, Dir};

static BOOTSTRAP_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/bootstrap");

pub fn write_all(dir: &std::path::Path) -> std::io::Result<()> {
    BOOTSTRAP_DIR.extract(dir)
}
