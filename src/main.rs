use anyhow::{bail, Context, Result};
use chrono::{Datelike, Local, NaiveDate};
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::{
    env, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

const SITE_URL: &str = "https://benletchford.com";
const AUTHOR: &str = "Ben Letchford";
const BASE_TEMPLATE: &str = include_str!("../templates/base.html");
const SITE_CSS: &str = include_str!("../static/styles.css");

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_args(&args)? {
        Command::Build { out_dir } => {
            build_site(&out_dir)?;
            println!("Built {}", out_dir.display());
        }
        Command::Dev {
            out_dir,
            host,
            port,
        } => serve_dev(&out_dir, &host, port)?,
    }

    Ok(())
}

fn build_site(out_dir: &Path) -> Result<()> {
    let site = Site::load(Path::new("."))?;
    site.build(out_dir)?;
    Ok(())
}

enum Command {
    Build {
        out_dir: PathBuf,
    },
    Dev {
        out_dir: PathBuf,
        host: String,
        port: u16,
    },
}

fn parse_args(args: &[String]) -> Result<Command> {
    if args.is_empty() {
        return Ok(Command::Build {
            out_dir: PathBuf::from("dist"),
        });
    }

    let mut out_dir = PathBuf::from("dist");
    let mut host = "127.0.0.1".to_string();
    let mut port = 8000;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let value = args
                    .get(i + 1)
                    .context("--out requires a directory argument")?;
                out_dir = PathBuf::from(value);
                i += 2;
            }
            "--host" => {
                host = args
                    .get(i + 1)
                    .context("--host requires an address argument")?
                    .to_string();
                i += 2;
            }
            "--port" => {
                port = args
                    .get(i + 1)
                    .context("--port requires a port argument")?
                    .parse()
                    .context("parsing --port")?;
                i += 2;
            }
            other => bail!("unknown argument: {other}"),
        }
    }

    match args[0].as_str() {
        "build" => Ok(Command::Build { out_dir }),
        "dev" => Ok(Command::Dev {
            out_dir,
            host,
            port,
        }),
        _ => bail!("usage: cargo run -- build [--out dist]\n       cargo run -- dev [--out dist] [--host 127.0.0.1] [--port 8000]"),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Collection {
    Writing,
}

impl Collection {
    fn as_str(self) -> &'static str {
        match self {
            Collection::Writing => "writing",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Collection::Writing => "Writing",
        }
    }

    fn intro(self) -> &'static str {
        match self {
            Collection::Writing => {
                "Longer essays and notes about software, preservation, faith, and life."
            }
        }
    }
}

#[derive(Debug)]
struct Site {
    home: Home,
    writing: Vec<Entry>,
    projects_html: String,
}

impl Site {
    fn load(root: &Path) -> Result<Self> {
        let content = root.join("content");
        Ok(Self {
            home: load_home(&content.join("home.md"))?,
            writing: load_collection(&content.join("writing"), Collection::Writing)?,
            projects_html: load_markdown_html(&content.join("projects.md"))?,
        })
    }

    fn build(&self, out_dir: &Path) -> Result<()> {
        if out_dir.exists() {
            fs::remove_dir_all(out_dir)
                .with_context(|| format!("cleaning {}", out_dir.display()))?;
        }
        fs::create_dir_all(out_dir).with_context(|| format!("creating {}", out_dir.display()))?;

        write_file(out_dir.join(".nojekyll"), "")?;
        write_file(out_dir.join("styles.css"), SITE_CSS)?;
        copy_public(Path::new("public"), out_dir)?;
        if Path::new("CNAME").exists() {
            fs::copy("CNAME", out_dir.join("CNAME")).context("copying CNAME")?;
        }

        write_file(out_dir.join("index.html"), self.render_home())?;
        self.write_collection(out_dir, Collection::Writing, &self.writing)?;
        write_file(out_dir.join("rss.xml"), self.render_rss())?;
        write_file(out_dir.join("sitemap.xml"), self.render_sitemap())?;

        Ok(())
    }

    fn write_collection(
        &self,
        out_dir: &Path,
        collection: Collection,
        entries: &[Entry],
    ) -> Result<()> {
        let visible: Vec<&Entry> = entries.iter().filter(|entry| !entry.draft).collect();
        if visible.is_empty() {
            return Ok(());
        }

        let index_path = out_dir.join(collection.as_str()).join("index.html");
        write_file(
            index_path,
            self.render_collection_index(collection, &visible),
        )?;

        for entry in visible {
            write_file(entry.output_path(out_dir), self.render_article(entry))?;
        }

        Ok(())
    }

    fn render_home(&self) -> String {
        let mut body = String::new();
        let mut section_number = 1;
        body.push_str("<header class=\"home-header\">\n");
        body.push_str(&format!("  <h1>{}</h1>\n", escape_html(&self.home.title)));
        body.push_str("</header>\n");

        if has_visible_content(&self.home.body_html) {
            body.push_str("<section class=\"abstract\" aria-labelledby=\"abstract-title\">\n");
            body.push_str("  <h2 id=\"abstract-title\">Abstract</h2>\n");
            body.push_str(&format!(
                "  <div class=\"abstract-body\">{}</div>\n",
                self.home.body_html
            ));
            body.push_str("</section>\n");
        }

        let mut writing = self.published_notes();
        writing.sort_by(|a, b| compare_entries_desc(a, b));
        if !writing.is_empty() {
            body.push_str("<section aria-labelledby=\"writing-title\">\n");
            push_section_title(&mut body, section_number, "writing-title", "Writing");
            section_number += 1;
            body.push_str("  <div class=\"entry-list\">\n");
            for entry in writing {
                body.push_str(&format!(
                    "    <div class=\"entry-row\"><a href=\"{}\">{}</a><span></span><time>{}</time></div>\n",
                    entry.url_path(),
                    escape_html(&entry.title),
                    entry.date.year()
                ));
            }
            body.push_str("  </div>\n");
            body.push_str("</section>\n");
        }

        if has_visible_content(&self.projects_html) {
            body.push_str("<section aria-labelledby=\"work-title\">\n");
            push_section_title(&mut body, section_number, "work-title", "Selected work");
            section_number += 1;
            body.push_str("  <div class=\"project-list\">\n");
            body.push_str(&self.projects_html);
            body.push_str("  </div>\n");
            body.push_str("</section>\n");
        }

        let elsewhere_html = render_elsewhere_html();
        if has_visible_content(&elsewhere_html) {
            body.push_str("<section aria-labelledby=\"elsewhere-title\">\n");
            push_section_title(&mut body, section_number, "elsewhere-title", "Elsewhere");
            body.push_str(&elsewhere_html);
            body.push_str("</section>\n");
        }
        body.push_str(&format!(
            "<footer class=\"site-footer\">&copy; {} {}</footer>\n",
            Local::now().year(),
            AUTHOR
        ));

        render_page(&self.home.title, &self.home.description, "/", "home", &body)
    }

    fn render_collection_index(&self, collection: Collection, entries: &[&Entry]) -> String {
        let mut sorted = entries.to_vec();
        sorted.sort_by(|a, b| compare_entries_desc(a, b));

        let mut body = String::new();
        body.push_str(&back_link("/"));
        body.push_str("<header class=\"page-header\">\n");
        body.push_str(&format!("  <h1>{}</h1>\n", collection.title()));
        body.push_str(&format!("  <p>{}</p>\n", collection.intro()));
        body.push_str("</header>\n");
        body.push_str("<div class=\"index-list\">\n");
        for entry in sorted {
            body.push_str(&format!(
                "  <article><p class=\"meta\">{} / {}</p><h2><a href=\"{}\">{}</a></h2><p>{}</p></article>\n",
                format_date(entry.date),
                escape_html(&entry.section),
                entry.url_path(),
                escape_html(&entry.title),
                escape_html(&entry.description)
            ));
        }
        body.push_str("</div>\n");

        render_page(
            collection.title(),
            collection.intro(),
            &format!("/{}/", collection.as_str()),
            "index",
            &body,
        )
    }

    fn render_article(&self, entry: &Entry) -> String {
        let mut body = String::new();
        body.push_str(&back_link("/"));
        body.push_str("<article class=\"article\">\n");
        body.push_str("  <header class=\"article-header\">\n");
        body.push_str(&format!("    <h1>{}</h1>\n", escape_html(&entry.title)));
        body.push_str(&format!("    <p>{}</p>\n", AUTHOR));
        body.push_str(&format!(
            "    <p class=\"dateline\">{}</p>\n",
            format_date(entry.date)
        ));
        body.push_str("  </header>\n");
        body.push_str("  <section class=\"article-abstract\" aria-label=\"Abstract\">\n");
        body.push_str("    <h2>Abstract</h2>\n");
        body.push_str(&format!("    <p>{}</p>\n", escape_html(&entry.description)));
        body.push_str("  </section>\n");
        body.push_str(&format!(
            "  <div class=\"article-body\">{}</div>\n",
            entry.body_html
        ));
        body.push_str("</article>\n");

        render_page(
            &entry.title,
            &entry.description,
            &entry.url_path(),
            "article",
            &body,
        )
    }

    fn render_rss(&self) -> String {
        let mut items = self.published_notes();
        items.sort_by(|a, b| compare_entries_desc(a, b));
        items.truncate(20);

        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<rss version=\"2.0\"><channel>\n");
        xml.push_str(&format!("  <title>{}</title>\n", escape_xml(AUTHOR)));
        xml.push_str(&format!("  <link>{}</link>\n", SITE_URL));
        xml.push_str(&format!(
            "  <description>{}</description>\n",
            escape_xml(&self.home.description)
        ));
        for entry in items {
            let url = format!("{}{}", SITE_URL, entry.url_path());
            xml.push_str("  <item>\n");
            xml.push_str(&format!(
                "    <title>{}</title>\n",
                escape_xml(&entry.title)
            ));
            xml.push_str(&format!("    <link>{}</link>\n", url));
            xml.push_str(&format!("    <guid>{}</guid>\n", url));
            xml.push_str(&format!(
                "    <pubDate>{}</pubDate>\n",
                entry
                    .date
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .to_rfc2822()
            ));
            xml.push_str(&format!(
                "    <description>{}</description>\n",
                escape_xml(&entry.description)
            ));
            xml.push_str("  </item>\n");
        }
        xml.push_str("</channel></rss>\n");
        xml
    }

    fn render_sitemap(&self) -> String {
        let has_writing = self.writing.iter().any(|entry| !entry.draft);
        let mut urls = vec![("/".to_string(), None)];
        if has_writing {
            urls.push(("/writing/".to_string(), None));
        }

        for entry in self.writing.iter().filter(|entry| !entry.draft) {
            urls.push((entry.url_path(), Some(entry.date)));
        }

        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
        for (path, lastmod) in urls {
            xml.push_str("  <url>\n");
            xml.push_str(&format!("    <loc>{}{}</loc>\n", SITE_URL, path));
            if let Some(date) = lastmod {
                xml.push_str(&format!("    <lastmod>{}</lastmod>\n", date));
            }
            xml.push_str("  </url>\n");
        }
        xml.push_str("</urlset>\n");
        xml
    }

    fn published_notes(&self) -> Vec<&Entry> {
        self.writing.iter().filter(|entry| !entry.draft).collect()
    }
}

#[derive(Debug)]
struct Home {
    title: String,
    description: String,
    body_html: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct HomeFrontMatter {
    title: String,
    description: String,
}

impl Default for HomeFrontMatter {
    fn default() -> Self {
        Self {
            title: AUTHOR.to_string(),
            description: "Personal site for Ben Letchford.".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct Entry {
    collection: Collection,
    slug: String,
    title: String,
    date: NaiveDate,
    section: String,
    description: String,
    draft: bool,
    body_html: String,
}

impl Entry {
    fn url_path(&self) -> String {
        format!("/{}/{}/", self.collection.as_str(), self.slug)
    }

    fn output_path(&self, out_dir: &Path) -> PathBuf {
        out_dir
            .join(self.collection.as_str())
            .join(&self.slug)
            .join("index.html")
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct EntryFrontMatter {
    title: String,
    date: String,
    section: String,
    description: String,
    draft: bool,
}

impl Default for EntryFrontMatter {
    fn default() -> Self {
        Self {
            title: String::new(),
            date: String::new(),
            section: "Notes".to_string(),
            description: String::new(),
            draft: false,
        }
    }
}

fn load_home(path: &Path) -> Result<Home> {
    if !path.exists() {
        let defaults = HomeFrontMatter::default();
        return Ok(Home {
            title: defaults.title,
            description: defaults.description,
            body_html: String::new(),
        });
    }

    let (front_matter, markdown) = read_markdown_with_front_matter::<HomeFrontMatter>(path)?;
    Ok(Home {
        title: front_matter.title,
        description: front_matter.description,
        body_html: render_markdown(&markdown),
    })
}

fn load_collection(dir: &Path, collection: Collection) -> Result<Vec<Entry>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for item in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if !item.file_type().is_file()
            || item.path().extension().and_then(|ext| ext.to_str()) != Some("md")
        {
            continue;
        }
        entries.push(load_entry(item.path(), dir, collection)?);
    }

    entries.sort_by(compare_entries_desc);
    Ok(entries)
}

fn load_entry(path: &Path, collection_root: &Path, collection: Collection) -> Result<Entry> {
    let (front_matter, markdown) = read_markdown_with_front_matter::<EntryFrontMatter>(path)?;
    if front_matter.title.trim().is_empty() {
        bail!("{} is missing a title", path.display());
    }
    if front_matter.description.trim().is_empty() {
        bail!("{} is missing a description", path.display());
    }
    let date = NaiveDate::parse_from_str(&front_matter.date, "%Y-%m-%d")
        .with_context(|| format!("parsing date for {}", path.display()))?;

    Ok(Entry {
        collection,
        slug: slug_from_path(path, collection_root)?,
        title: front_matter.title,
        date,
        section: front_matter.section,
        description: front_matter.description,
        draft: front_matter.draft,
        body_html: render_markdown(&markdown),
    })
}

fn load_markdown_html(path: &Path) -> Result<String> {
    if !path.exists() {
        return Ok(String::new());
    }

    let source = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let source = source.replace("\r\n", "\n");
    let markdown = if let Some(rest) = source.strip_prefix("---\n") {
        let end = rest.find("\n---\n").with_context(|| {
            format!(
                "{} is missing a closing front matter marker",
                path.display()
            )
        })?;
        &rest[end + 5..]
    } else {
        &source
    };

    Ok(render_markdown(markdown.trim_start_matches('\n')))
}

fn serve_dev(out_dir: &Path, host: &str, port: u16) -> Result<()> {
    build_site(out_dir)?;

    let address = format!("{host}:{port}");
    let listener =
        TcpListener::bind(&address).with_context(|| format!("binding dev server to {address}"))?;

    println!("Serving http://{address}/");
    println!("Refreshing a page rebuilds the site.");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) = handle_dev_request(stream, out_dir) {
                    eprintln!("dev server error: {error:#}");
                }
            }
            Err(error) => eprintln!("dev server connection error: {error}"),
        }
    }

    Ok(())
}

fn handle_dev_request(mut stream: TcpStream, out_dir: &Path) -> Result<()> {
    let mut buffer = [0; 8192];
    let bytes_read = stream.read(&mut buffer).context("reading HTTP request")?;
    if bytes_read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let Some(first_line) = request.lines().next() else {
        return Ok(());
    };
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or("/");

    if method != "GET" && method != "HEAD" {
        return send_response(
            &mut stream,
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            b"Method not allowed\n",
            method != "HEAD",
        );
    }

    if let Err(error) = build_site(out_dir) {
        let body = format!("Build failed:\n{error:#}\n");
        return send_response(
            &mut stream,
            "500 Internal Server Error",
            "text/plain; charset=utf-8",
            body.as_bytes(),
            method != "HEAD",
        );
    }

    let Some(path) = resolve_request_path(out_dir, target) else {
        return send_response(
            &mut stream,
            "400 Bad Request",
            "text/plain; charset=utf-8",
            b"Bad request\n",
            method != "HEAD",
        );
    };

    let body = match fs::read(&path) {
        Ok(body) => body,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return send_response(
                &mut stream,
                "404 Not Found",
                "text/plain; charset=utf-8",
                b"Not found\n",
                method != "HEAD",
            );
        }
        Err(error) => return Err(error).with_context(|| format!("reading {}", path.display())),
    };

    send_response(
        &mut stream,
        "200 OK",
        content_type(&path),
        &body,
        method != "HEAD",
    )
}

fn resolve_request_path(root: &Path, target: &str) -> Option<PathBuf> {
    let path = target.split('?').next().unwrap_or("/");
    let path = path.trim_start_matches('/');
    let path = percent_decode(path)?;

    let mut resolved = root.to_path_buf();
    if path.is_empty() {
        resolved.push("index.html");
        return Some(resolved);
    }

    for segment in path.split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." || segment.contains('\\') {
            return None;
        }
        resolved.push(segment);
    }

    if resolved.is_dir() {
        resolved.push("index.html");
    } else if resolved.extension().is_none() {
        let index = resolved.join("index.html");
        if index.exists() {
            resolved = index;
        }
    }

    Some(resolved)
}

fn percent_decode(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%' {
            let high = hex_value(*bytes.get(i + 1)?)?;
            let low = hex_value(*bytes.get(i + 2)?)?;
            decoded.push(high << 4 | low);
            i += 3;
        } else {
            decoded.push(bytes[i]);
            i += 1;
        }
    }

    String::from_utf8(decoded).ok()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn send_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
    include_body: bool,
) -> Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .context("writing HTTP response headers")?;

    if include_body {
        stream
            .write_all(body)
            .context("writing HTTP response body")?;
    }

    Ok(())
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("css") => "text/css; charset=utf-8",
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("ttf") => "font/ttf",
        Some("txt") => "text/plain; charset=utf-8",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("xml") => "application/xml; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn read_markdown_with_front_matter<T>(path: &Path) -> Result<(T, String)>
where
    T: for<'de> Deserialize<'de>,
{
    let source = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let source = source.replace("\r\n", "\n");
    if !source.starts_with("---\n") {
        bail!("{} must start with YAML front matter", path.display());
    }
    let rest = &source[4..];
    let end = rest.find("\n---\n").with_context(|| {
        format!(
            "{} is missing a closing front matter marker",
            path.display()
        )
    })?;
    let yaml = &rest[..end];
    let markdown = rest[end + 5..].trim_start_matches('\n').to_string();
    let front_matter = serde_yaml::from_str(yaml)
        .with_context(|| format!("parsing front matter in {}", path.display()))?;
    Ok((front_matter, markdown))
}

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

fn render_page(
    title: &str,
    description: &str,
    path: &str,
    body_class: &str,
    content: &str,
) -> String {
    let page_title = if title == AUTHOR {
        AUTHOR.to_string()
    } else {
        format!("{title} - {AUTHOR}")
    };
    BASE_TEMPLATE
        .replace("{{ page_title }}", &escape_html(&page_title))
        .replace("{{ description }}", &escape_attr(description))
        .replace(
            "{{ canonical_url }}",
            &escape_attr(&format!("{SITE_URL}{path}")),
        )
        .replace("{{ body_class }}", &escape_attr(body_class))
        .replace("{{ content }}", content)
}

fn push_section_title(body: &mut String, number: usize, id: &str, title: &str) {
    body.push_str(&format!(
        "  <h2 id=\"{}\" class=\"section-title\">{} {}</h2>\n",
        escape_attr(id),
        number,
        escape_html(title)
    ));
}

fn render_elsewhere_html() -> String {
    let mut html = String::new();
    html.push_str("  <div class=\"elsewhere\">\n");
    html.push_str("    <p><span>Email</span><a href=\"mailto:ben@letchford.cloud\">ben@letchford.cloud</a></p>\n");
    html.push_str("    <p><span>GitHub</span><a href=\"https://github.com/benletchford\" target=\"_blank\" rel=\"noopener\">github.com/benletchford</a></p>\n");
    html.push_str("    <p><span>LinkedIn</span><a href=\"https://www.linkedin.com/in/benletchford\" target=\"_blank\" rel=\"noopener\">in/benletchford</a></p>\n");
    html.push_str("  </div>\n");
    html
}

fn has_visible_content(html: &str) -> bool {
    let html = strip_html_comments(html);
    let lower = html.to_ascii_lowercase();
    if lower.contains("<img ") || lower.contains("<svg") || lower.contains("<video") {
        return true;
    }

    let mut text = String::new();
    let mut in_tag = false;
    for character in html.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(character),
            _ => {}
        }
    }

    !text.trim().is_empty()
}

fn strip_html_comments(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut rest = input;

    while let Some(start) = rest.find("<!--") {
        output.push_str(&rest[..start]);
        rest = &rest[start + 4..];
        if let Some(end) = rest.find("-->") {
            rest = &rest[end + 3..];
        } else {
            return output;
        }
    }

    output.push_str(rest);
    output
}

fn back_link(href: &str) -> String {
    format!(
        "<a class=\"back-link\" href=\"{}\">&larr;&ensp;Ben Letchford</a>\n",
        escape_attr(href)
    )
}

fn slug_from_path(path: &Path, collection_root: &Path) -> Result<String> {
    let relative = path
        .strip_prefix(collection_root)
        .with_context(|| format!("building slug for {}", path.display()))?;
    let mut without_extension = relative.to_path_buf();
    without_extension.set_extension("");
    Ok(without_extension
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/"))
}

fn compare_entries_desc(a: &Entry, b: &Entry) -> std::cmp::Ordering {
    b.date.cmp(&a.date).then_with(|| a.title.cmp(&b.title))
}

fn format_date(date: NaiveDate) -> String {
    format!(
        "{} {} {}",
        date.day(),
        month_name(date.month()),
        date.year()
    )
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

fn write_file(path: PathBuf, contents: impl AsRef<[u8]>) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    fs::write(&path, contents).with_context(|| format!("writing {}", path.display()))
}

fn copy_public(public_dir: &Path, out_dir: &Path) -> Result<()> {
    if !public_dir.exists() {
        return Ok(());
    }

    for item in WalkDir::new(public_dir).into_iter().filter_map(Result::ok) {
        if !item.file_type().is_file() {
            continue;
        }
        let relative = item.path().strip_prefix(public_dir)?;
        let target = out_dir.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
        fs::copy(item.path(), &target).with_context(|| {
            format!("copying {} to {}", item.path().display(), target.display())
        })?;
    }
    Ok(())
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(input: &str) -> String {
    escape_html(input).replace('"', "&quot;")
}

fn escape_xml(input: &str) -> String {
    escape_attr(input).replace('\'', "&apos;")
}
