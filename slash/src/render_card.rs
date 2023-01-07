#[derive(serde::Serialize)]
struct Context {
    level: String,
    rank: String,
    name: String,
    discriminator: String,
}

pub async fn render(
    name: String,
    discriminator: String,
    level: String,
    rank: String,
) -> Result<Vec<u8>, RenderingError> {
    let context = Context {
        level,
        rank,
        name,
        discriminator,
    };
    tokio::task::spawn_blocking(move || do_render(&context)).await?
}

fn do_render(context: &Context) -> Result<Vec<u8>, RenderingError> {
    let opt = resvg::usvg::Options::default();
    let mut fontdb = fontdb::Database::new();
    fontdb.load_font_data(include_bytes!("resources/OpenSans.ttf").to_vec());
    let mut tt = tinytemplate::TinyTemplate::new();
    tt.add_template("svg", include_str!("resources/card.svg"));

    let svg = tt.render("hello", context)?;
    let tree = resvg::usvg::Tree::from_str(&svg, &opt)?;
    let pixmap_size = tree.size.to_screen_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or(RenderingError::PixmapCreation)?;
    resvg::render(
        &tree,
        resvg::usvg::FitTo::Original,
        resvg::tiny_skia::Transform::default(),
        pixmap.as_mut(),
    );
    Ok(pixmap.encode_png()?)
}

#[derive(Debug, thiserror::Error)]
pub enum RenderingError {
    #[error("TinyTemplate error: {0}")]
    Template(#[from] tinytemplate::error::Error),
    #[error("Tokio JoinError: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("uSVG error: {0}")]
    Usvg(#[from] resvg::usvg::Error),
    #[error("Pixmap error: {0}")]
    Pixmap(#[from] png::EncodingError),
    #[error("Pixmap Creation error!")]
    PixmapCreation,
}