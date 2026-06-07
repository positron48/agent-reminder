fn main() {
    let svg = include_str!("../dmg-assets/background.svg");
    let mut options = usvg::Options::default();
    options.font_family = "Arial".to_string();
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    options.fontdb = std::sync::Arc::new(fontdb);

    let tree = usvg::Tree::from_str(svg, &options).expect("valid dmg background svg");

    let width = tree.size().width() as u32;
    let height = tree.size().height() as u32;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height).expect("pixmap");
    resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("dmg-assets/background.png");
    pixmap.save_png(&out).expect("write background png");
    println!("Wrote {}", out.display());
}
