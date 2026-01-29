use comrak::plugins::syntect::SyntectAdapter;
use comrak::{ComrakOptions, ComrakPlugins, markdown_to_html_with_plugins};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Metadata {
    title: String,
    date: String,
    tagline: String,
}

fn main() {
    _ = fs::remove_dir_all("public");
    _ = fs::create_dir("public");
    fs::copy("style.css", "public/style.css").unwrap();
    fs::create_dir("public/static").unwrap();
    copy_dir::copy_dir(
        "static/mandelbrot-gallery",
        "public/static/mandelbrot-gallery",
    )
    .unwrap();

    render_page(
        "public/index.html",
        &generate_gallery("static/mandelbrot-gallery"),
        "Nic Ball",
    );

    let adapter = SyntectAdapter::new("base16-mocha.dark");
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let paths = fs::read_dir("content").unwrap();
    let mut article_list_html = String::new();

    let mut options = ComrakOptions::default();
    options.extension.footnotes = true;

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = fs::read_to_string(&path).unwrap();
            let parts = content.splitn(3, "---").collect::<Vec<_>>();
            assert_eq!(parts.len(), 3);

            let metadata = serde_yaml::from_str::<Metadata>(parts[1]).unwrap();
            let stem = path.file_stem().unwrap().to_str().unwrap();

            let html = markdown_to_html_with_plugins(parts[2], &options, &plugins)
                .replace(".md\"", ".html\"");

            render_page(&format!("public/{}.html", stem), &html, &metadata.title);
            article_list_html.push_str(&format!(
                r#"<div class="article-item">
                    <div class="article-header">
                        <h2><a href="{}.html">{}</a></h2>
                        <span class="article-date">{}</span>
                    </div>
                    <div class="article-excerpt">
                        <p>{}</p>
                    </div>
                </div>"#,
                stem, metadata.title, metadata.date, metadata.tagline
            ));
        }
    }

    render_page("public/articles.html", &article_list_html, "Articles");
}

fn render_page(filename: &str, content: &str, title: &str) {
    let template = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="color-scheme" content="dark">
            <title>{}</title>
            <link rel="stylesheet" href="style.css">
            </head>
        <body>
            <header>
                <nav>
                    <div class="nav-name">
                        <h3>Nic Ball</h3>
                    </div>
                    <div class="nav-links">
                        <a href="index.html">Home</a>
                        <a href="articles.html">Articles</a>
                    </div>
                    <div class="nav-github">
                        <a href="https://github.com/void-scape" class="github-link" target="_blank">
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.28 1.15-.28 2.35 0 3.5-.73 1.02-1.08 2.25-1 3.5 0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/>
                                <path d="M9 18c-4.51 2-5-2-7-2"/>
                            </svg>
                        </a>
                    </div>
                </nav>
            </header>
            <main>{}</main>
            <footer>
                <div class="footer-content">
                    <p>&copy; Nic Ball 2026</p>
                    <a href="{}" class="back-to-top">Back to Top</a>
                </div>
            </footer>

            <script>
                function copyToml(img) {{
                    const toml = img.getAttribute('data-toml');
                    navigator.clipboard.writeText(toml).then(() => {{
                        const toast = document.getElementById('toast');
                        toast.classList.add('show');
                        setTimeout(() => {{
                            toast.classList.remove('show');
                        }}, 3000);
                    }}).catch(err => {{
                        console.error('Failed to copy text: ', err);
                    }});
                }}
            </script>
        </body>
        </html>"#,
        title, content, "#top"
    );
    fs::write(filename, template).unwrap();
}

fn generate_gallery(root: &str) -> String {
    let mut html = String::new();

    let frames = format!("{root}/frames");
    let configs = format!("{root}/configs");

    html.push_str(r#"<h3 class="gallery-title">Mandelbrot Set Gallery</h2>"#);
    html.push_str(r#"<div class="gallery-grid">"#);
    if let Ok(images) = fs::read_dir(frames) {
        let mut images: Vec<_> = images.filter_map(|e| e.ok()).collect();
        images.sort_by_key(|e| e.path());

        for img in images {
            let img = img.path();
            let config = format!(
                "{configs}/{}.toml",
                img.file_stem().unwrap().to_str().unwrap()
            );

            let toml = fs::read_to_string(&config).unwrap().replace("\"", "&quot;");
            let img = img.to_str().unwrap();

            html.push_str(&format!(
                r#"<img 
                    src="{img}" 
                    class="gallery-item loading" 
                    data-toml="{toml}" 
                    onclick="copyToml(this)" 
                    onload="this.classList.remove('loading')"
                    loading="lazy"
                    alt="Mandelbrot Fractal">"#,
            ));
        }
    }
    html
}
