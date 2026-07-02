use anyhow::{bail, Context, Result};
use chrono::{Datelike, Local, NaiveDate};
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

const SITE_URL: &str = "https://benletchford.com";
const AUTHOR: &str = "Ben Letchford";
const BASE_TEMPLATE: &str = include_str!("../templates/base.html");

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let Command::Render { out_dir } = parse_args(&args)?;
    render_site(&out_dir)?;
    println!("Rendered {}", out_dir.display());

    Ok(())
}

fn render_site(out_dir: &Path) -> Result<()> {
    let site = Site::load(Path::new("."))?;
    site.render(out_dir)?;
    Ok(())
}

enum Command {
    Render { out_dir: PathBuf },
}

fn parse_args(args: &[String]) -> Result<Command> {
    let mut out_dir = PathBuf::from("dist");
    let mut i = 1;

    if args.first().map(String::as_str) != Some("render") {
        bail!(
            "usage: cargo run -- render [--out dist]\n       use `trunk build` or `trunk serve` for the site workflow"
        );
    }

    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let value = args
                    .get(i + 1)
                    .context("--out requires a directory argument")?;
                out_dir = PathBuf::from(value);
                i += 2;
            }
            other => bail!("unknown argument: {other}"),
        }
    }

    Ok(Command::Render { out_dir })
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

    fn render(&self, out_dir: &Path) -> Result<()> {
        fs::create_dir_all(out_dir).with_context(|| format!("creating {}", out_dir.display()))?;

        write_file(out_dir.join(".nojekyll"), "")?;
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
                    format_month_year(entry.date)
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
            "<footer class=\"site-footer\"><a href=\"https://github.com/benletchford/benletchford.com\" target=\"_blank\" rel=\"noopener\">&copy; {} {}</a></footer>\n",
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
        body.push_str("  <div class=\"article-body\">");
        body.push_str(&entry.body_html);
        body.push_str(&render_references(&entry.references));
        body.push_str("</div>\n");
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
    references: Vec<Reference>,
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
    references: Vec<Reference>,
}

impl Default for EntryFrontMatter {
    fn default() -> Self {
        Self {
            title: String::new(),
            date: String::new(),
            section: "Notes".to_string(),
            description: String::new(),
            draft: false,
            references: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Reference {
    key: String,
    citation: String,
    entry: String,
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
    validate_references(path, &front_matter.references)?;
    let date = NaiveDate::parse_from_str(&front_matter.date, "%Y-%m-%d")
        .with_context(|| format!("parsing date for {}", path.display()))?;
    let markdown = resolve_citations(&markdown, &front_matter.references, path)?;

    Ok(Entry {
        collection,
        slug: slug_from_path(path, collection_root)?,
        title: front_matter.title,
        date,
        section: front_matter.section,
        description: front_matter.description,
        draft: front_matter.draft,
        body_html: render_markdown(&markdown),
        references: front_matter.references,
    })
}

fn validate_references(path: &Path, references: &[Reference]) -> Result<()> {
    let mut keys = HashSet::new();
    for reference in references {
        if reference.key.trim().is_empty() {
            bail!("{} has a reference with an empty key", path.display());
        }
        if !is_reference_key(&reference.key) {
            bail!(
                "{} has an invalid reference key: {}",
                path.display(),
                reference.key
            );
        }
        if reference.citation.trim().is_empty() {
            bail!(
                "{} has reference {} without citation text",
                path.display(),
                reference.key
            );
        }
        if reference.entry.trim().is_empty() {
            bail!(
                "{} has reference {} without an entry",
                path.display(),
                reference.key
            );
        }
        if !keys.insert(reference.key.as_str()) {
            bail!(
                "{} has a duplicate reference key: {}",
                path.display(),
                reference.key
            );
        }
    }

    Ok(())
}

fn resolve_citations(markdown: &str, references: &[Reference], path: &Path) -> Result<String> {
    if references.is_empty() || !markdown.contains("[@") {
        return Ok(markdown.to_string());
    }

    let references_by_key: HashMap<&str, &Reference> = references
        .iter()
        .map(|reference| (reference.key.as_str(), reference))
        .collect();
    let mut output = String::with_capacity(markdown.len());
    let mut offset = 0;

    while let Some(start_relative) = markdown[offset..].find("[@") {
        let start = offset + start_relative;
        output.push_str(&markdown[offset..start]);

        let content_start = start + 1;
        let Some(end_relative) = markdown[content_start..].find(']') else {
            output.push_str(&markdown[start..]);
            return Ok(output);
        };
        let end = content_start + end_relative;
        let content = &markdown[content_start..end];

        output.push_str(&render_citation_group(content, &references_by_key, path)?);
        offset = end + 1;
    }

    output.push_str(&markdown[offset..]);
    Ok(output)
}

fn render_citation_group(
    content: &str,
    references_by_key: &HashMap<&str, &Reference>,
    path: &Path,
) -> Result<String> {
    let mut html = String::new();
    html.push('(');
    for (index, part) in content.split(';').enumerate() {
        let key = part.trim().strip_prefix('@').with_context(|| {
            format!(
                "{} has an invalid citation group: [{}]",
                path.display(),
                content
            )
        })?;
        if !is_reference_key(key) {
            bail!("{} has an invalid citation key: {}", path.display(), key);
        }
        let reference = references_by_key
            .get(key)
            .with_context(|| format!("{} cites unknown reference: {}", path.display(), key))?;

        if index > 0 {
            html.push_str("; ");
        }
        html.push_str(&format!(
            "<a class=\"citation-link\" href=\"#{}\">{}</a>",
            escape_attr(&reference_id(key)),
            escape_html(&reference.citation)
        ));
    }
    html.push(')');
    Ok(html)
}

fn render_references(references: &[Reference]) -> String {
    if references.is_empty() {
        return String::new();
    }

    let mut html = String::new();
    html.push_str("<h2 id=\"references\" class=\"references-title\">References</h2>\n");
    html.push_str("<div class=\"references\">\n");
    for reference in references {
        html.push_str(&format!(
            "  <p id=\"{}\">{}</p>\n",
            escape_attr(&reference_id(&reference.key)),
            render_reference_entry(&reference.entry)
        ));
    }
    html.push_str("</div>\n");
    html
}

fn render_reference_entry(entry: &str) -> String {
    let rendered = render_markdown(entry.trim());
    rendered
        .trim()
        .strip_prefix("<p>")
        .and_then(|inner| inner.strip_suffix("</p>"))
        .unwrap_or(rendered.trim())
        .to_string()
}

fn reference_id(key: &str) -> String {
    format!("ref-{key}")
}

fn is_reference_key(key: &str) -> bool {
    !key.is_empty()
        && key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
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
        .replace("{{ math_head }}", render_math_head(content))
        .replace("{{ body_class }}", &escape_attr(body_class))
        .replace("{{ content }}", content)
}

fn render_math_head(content: &str) -> &'static str {
    if !contains_latex_math(content) {
        return "";
    }

    r#"<script>
      window.MathJax = {
        tex: {
          inlineMath: [["$", "$"]],
          displayMath: [["$$", "$$"]],
          processEscapes: true
        },
        options: {
          skipHtmlTags: ["script", "noscript", "style", "textarea", "pre", "code"]
        }
      };
    </script>
    <script defer src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-chtml.js"></script>"#
}

fn contains_latex_math(content: &str) -> bool {
    content.contains("$$")
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

fn format_month_year(date: NaiveDate) -> String {
    format!("{}, {}", month_name(date.month()), date.year())
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
