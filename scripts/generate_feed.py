from argparse import ArgumentParser
from datetime import datetime, timezone
from email.utils import format_datetime
from html import unescape
from html.parser import HTMLParser
from pathlib import Path
import json
import re
from urllib.parse import urlparse
from xml.dom import minidom


ROOT = Path(__file__).resolve().parents[1]
METADATA_PATH = ROOT / "content" / "articles.json"
CONTENT_NAMESPACE = "http://purl.org/rss/1.0/modules/content/"
ATOM_NAMESPACE = "http://www.w3.org/2005/Atom"
VOID_ELEMENTS = {
    "area",
    "base",
    "br",
    "col",
    "embed",
    "hr",
    "img",
    "input",
    "link",
    "meta",
    "param",
    "source",
    "track",
    "wbr",
}


class ArticleExtractor(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=False)
        self.article_count = 0
        self.article_depth = 0
        self.skipped_h1 = False
        self.skip_depth = 0
        self.body = []
        self.h1 = []

    @property
    def capturing(self):
        return self.article_depth > 0

    def handle_starttag(self, tag, attrs):
        tag = tag.lower()
        if not self.capturing:
            if tag == "article":
                self.article_count += 1
                self.article_depth = 1
            return

        if tag not in VOID_ELEMENTS:
            self.article_depth += 1

        if self.skip_depth:
            if tag not in VOID_ELEMENTS:
                self.skip_depth += 1
            return

        if tag == "h1" and not self.skipped_h1:
            self.skipped_h1 = True
            self.skip_depth = 1
            return

        self.body.append(self.get_starttag_text())

    def handle_startendtag(self, tag, attrs):
        if self.capturing and not self.skip_depth:
            self.body.append(self.get_starttag_text())

    def handle_endtag(self, tag):
        tag = tag.lower()
        if not self.capturing:
            return

        if self.skip_depth:
            self.skip_depth -= 1
            self.article_depth -= 1
            return

        if tag == "article" and self.article_depth == 1:
            self.article_depth = 0
            return

        self.body.append(f"</{tag}>")
        self.article_depth -= 1

    def handle_data(self, data):
        if not self.capturing:
            return
        if self.skip_depth:
            self.h1.append(data)
        else:
            self.body.append(data)

    def handle_entityref(self, name):
        value = f"&{name};"
        if self.capturing:
            (self.h1 if self.skip_depth else self.body).append(value)

    def handle_charref(self, name):
        value = f"&#{name};"
        if self.capturing:
            (self.h1 if self.skip_depth else self.body).append(value)

    def handle_comment(self, data):
        if self.capturing and not self.skip_depth:
            self.body.append(f"<!--{data}-->")

    def result(self):
        if self.article_count != 1:
            raise ValueError(f"expected one article element, found {self.article_count}")
        if self.article_depth != 0:
            raise ValueError("article element was not closed")
        if not self.skipped_h1:
            raise ValueError("article is missing its primary h1")
        return unescape("".join(self.h1).strip()), "".join(self.body).strip()


class EmbeddedUrlValidator(HTMLParser):
    def __init__(self):
        super().__init__()
        self.invalid = []

    def handle_starttag(self, tag, attrs):
        attributes = dict(attrs)
        for attribute in ("href", "src"):
            value = attributes.get(attribute)
            if value and not self._is_allowed(value):
                self.invalid.append((tag, attribute, value))

        srcset = attributes.get("srcset")
        if srcset:
            for candidate in srcset.split(","):
                value = candidate.strip().split(" ", 1)[0]
                if value and not self._is_allowed(value):
                    self.invalid.append((tag, "srcset", value))

    def handle_startendtag(self, tag, attrs):
        self.handle_starttag(tag, attrs)

    @staticmethod
    def _is_allowed(value):
        if value.startswith("#"):
            return True
        parsed = urlparse(value)
        return parsed.scheme in {"https", "mailto"}


def load_metadata():
    with METADATA_PATH.open(encoding="utf-8") as handle:
        metadata = json.load(handle)

    feed = metadata.get("feed")
    articles = metadata.get("articles")
    if not isinstance(feed, dict) or not isinstance(articles, list):
        raise ValueError("articles.json must contain feed and articles entries")
    if not articles:
        raise ValueError("articles.json must contain at least one article")
    return feed, articles


def validate_metadata(feed, articles):
    for key in ("title", "link", "self_url", "description", "language"):
        if not feed.get(key):
            raise ValueError(f"feed metadata is missing {key}")

    paths = set()
    titles = set()
    for article in articles:
        for key in ("source", "path", "canonical_url", "title", "description", "tags"):
            if not article.get(key):
                raise ValueError(f"article metadata is missing {key}: {article!r}")

        path = article["path"]
        if not path.startswith("/") or path == "/":
            raise ValueError(f"article path must be an absolute site path: {path}")
        if path in paths:
            raise ValueError(f"duplicate article path: {path}")
        if article["canonical_url"] != f"https://tools.cosmol.org{path}":
            raise ValueError(f"canonical URL does not match article path: {path}")
        if article["title"] in titles:
            raise ValueError(f"duplicate article title: {article['title']}")
        paths.add(path)
        titles.add(article["title"])

        tags = article["tags"]
        if len(tags) > 4:
            raise ValueError(f"DEV supports at most four tags: {path}")
        for tag in tags:
            if len(tag) > 20 or not re.fullmatch(r"[a-z0-9]+", tag):
                raise ValueError(f"invalid DEV tag {tag!r} for {path}")

        author = article.get("author")
        if author and not re.fullmatch(r"\S+@\S+ \(.+\)", author):
            raise ValueError(
                f"RSS author must use 'email@example.com (Author Name)' for {path}"
            )

        source = ROOT / article["source"]
        markdown_title = source.read_text(encoding="utf-8").splitlines()[0]
        if markdown_title != f"# {article['title']}":
            raise ValueError(f"metadata title does not match Markdown h1: {source}")


def extract_article(public_dir, article):
    html_path = public_dir / article["path"].lstrip("/") / "index.html"
    extractor = ArticleExtractor()
    extractor.feed(html_path.read_text(encoding="utf-8"))
    title, body = extractor.result()
    if title != article["title"]:
        raise ValueError(f"metadata title does not match prerendered h1: {html_path}")
    if "]]>" in body:
        raise ValueError(f"article body contains a CDATA terminator: {html_path}")

    validator = EmbeddedUrlValidator()
    validator.feed(body)
    if validator.invalid:
        details = ", ".join(f"{tag}[{attr}]={value!r}" for tag, attr, value in validator.invalid)
        raise ValueError(f"article body contains non-absolute URLs: {details}")
    return body


def append_text(document, parent, tag, value):
    element = document.createElement(tag)
    element.appendChild(document.createTextNode(value))
    parent.appendChild(element)
    return element


def parse_published_at(value, path):
    try:
        published_at = datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError as error:
        raise ValueError(f"invalid published_at for {path}: {value}") from error
    if published_at.tzinfo is None:
        raise ValueError(f"published_at must include a timezone for {path}")
    return published_at.astimezone(timezone.utc)


def newest_articles_first(articles):
    earliest = datetime.min.replace(tzinfo=timezone.utc)
    return sorted(
        articles,
        key=lambda article: (
            parse_published_at(article["published_at"], article["path"])
            if article.get("published_at")
            else earliest
        ),
        reverse=True,
    )


def build_feed(public_dir, feed, articles):
    document = minidom.Document()
    rss = document.createElement("rss")
    rss.setAttribute("version", "2.0")
    rss.setAttribute("xmlns:content", CONTENT_NAMESPACE)
    rss.setAttribute("xmlns:atom", ATOM_NAMESPACE)
    document.appendChild(rss)

    channel = document.createElement("channel")
    rss.appendChild(channel)
    append_text(document, channel, "title", feed["title"])
    append_text(document, channel, "link", feed["link"])
    append_text(document, channel, "description", feed["description"])
    append_text(document, channel, "language", feed["language"])

    self_link = document.createElement("atom:link")
    self_link.setAttribute("href", feed["self_url"])
    self_link.setAttribute("rel", "self")
    self_link.setAttribute("type", "application/rss+xml")
    channel.appendChild(self_link)

    published_dates = []
    for article in newest_articles_first(articles):
        body = extract_article(public_dir, article)
        canonical = article["canonical_url"]

        item = document.createElement("item")
        channel.appendChild(item)
        append_text(document, item, "title", article["title"])
        append_text(document, item, "link", canonical)
        guid = append_text(document, item, "guid", canonical)
        guid.setAttribute("isPermaLink", "true")
        append_text(document, item, "description", article["description"])

        if article.get("published_at"):
            published_at = parse_published_at(article["published_at"], article["path"])
            published_dates.append(published_at)
            append_text(document, item, "pubDate", format_datetime(published_at, usegmt=True))
        if article.get("author"):
            append_text(document, item, "author", article["author"])

        for tag in article["tags"]:
            append_text(document, item, "category", tag)

        content = document.createElement("content:encoded")
        content.appendChild(document.createCDATASection(body))
        item.appendChild(content)

    if published_dates:
        last_build_date = append_text(
            document,
            channel,
            "lastBuildDate",
            format_datetime(max(published_dates), usegmt=True),
        )
        channel.insertBefore(last_build_date, channel.getElementsByTagName("item")[0])

    output = document.toprettyxml(indent="  ", encoding="UTF-8")
    return output.replace(b"\r\n", b"\n")


def main():
    parser = ArgumentParser(description="Generate the COSMolKit RSS feed from SSG output")
    parser.add_argument("public_dir", type=Path, help="Dioxus prerendered public directory")
    parser.add_argument("output", type=Path, help="feed.xml output path")
    parser.add_argument("--check", action="store_true", help="fail if output is not current")
    args = parser.parse_args()

    feed, articles = load_metadata()
    validate_metadata(feed, articles)
    output = build_feed(args.public_dir, feed, articles)

    if args.check:
        if not args.output.exists() or args.output.read_bytes() != output:
            raise SystemExit(
                f"{args.output} is stale; regenerate it with scripts/generate_feed.py"
            )
        print(f"Validated RSS feed with {len(articles)} articles: {args.output}")
        return

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(output)
    print(f"Generated RSS feed with {len(articles)} articles: {args.output}")


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError) as error:
        raise SystemExit(f"RSS generation failed: {error}") from error
