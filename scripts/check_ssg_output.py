from html.parser import HTMLParser
from pathlib import Path
import sys


SITE_ORIGIN = "https://tools.cosmol.org"
PAGES = {
    "/": "COSMolKit — Browser-Native Cheminformatics Powered by Rust",
    "/tools": "Browser-Based Cheminformatics Tools Powered by Rust — COSMolKit",
    "/smiles-to-svg": "SMILES to SVG — Molecular Structure Renderer | COSMolKit",
    "/format-converter": "Molecular Format Converter — SDF, SMILES, MOL2, PDB | COSMolKit",
    "/conformer-generator": "SMILES to 3D Conformer — Browser ETKDG Generator | COSMolKit",
    "/inchi": "InChI Converter — InChI, InChIKey & Molecular Structure | COSMolKit",
    "/molecular-properties": "Molecular Properties Calculator — MW, TPSA, logP | COSMolKit",
    "/smiles-canonicalizer": "SMILES Canonicalizer — Canonical & Isomeric SMILES | COSMolKit",
    "/ecosystem": "COSMol Ecosystem — Rust-Powered Cheminformatics & Browser-Native Tools",
}
SEARCH_PHRASES = {
    "/": ("rust", "cheminformatics", "browser-native"),
    "/tools": ("rust", "cheminformatics", "browser-based"),
    "/ecosystem": ("rust", "cheminformatics", "browser-native"),
}
CARD_IMAGE_PREFIXES = ("/assets/benzene-", "/assets/sdf-")


class SeoParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.in_title = False
        self.in_h1 = False
        self.in_body = False
        self.title = ""
        self.h1 = ""
        self.body_text = ""
        self.description = None
        self.canonical = None
        self.image_sources = []

    def handle_starttag(self, tag, attrs):
        attributes = dict(attrs)
        if tag == "title":
            self.in_title = True
        elif tag == "body":
            self.in_body = True
        elif tag == "h1":
            self.in_h1 = True
        elif tag == "meta" and attributes.get("name") == "description":
            self.description = attributes.get("content")
        elif tag == "link" and attributes.get("rel") == "canonical":
            self.canonical = attributes.get("href")
        elif tag == "img":
            self.image_sources.append(attributes.get("src", ""))

    def handle_endtag(self, tag):
        if tag == "title":
            self.in_title = False
        elif tag == "body":
            self.in_body = False
        elif tag == "h1":
            self.in_h1 = False

    def handle_data(self, data):
        if self.in_title:
            self.title += data
        if self.in_h1:
            self.h1 += data
        if self.in_body:
            self.body_text += data


def canonical_for(route):
    return f"{SITE_ORIGIN}/" if route == "/" else f"{SITE_ORIGIN}{route}"


def main():
    public_dir = Path(sys.argv[1] if len(sys.argv) > 1 else "deploy/web/public")
    failures = []

    for route, expected_title in PAGES.items():
        html_path = public_dir / route.lstrip("/") / "index.html"
        if route == "/":
            html_path = public_dir / "index.html"
        if not html_path.exists():
            failures.append(f"missing generated page: {html_path}")
            continue

        parser = SeoParser()
        parser.feed(html_path.read_text(encoding="utf-8"))
        expected_canonical = canonical_for(route)
        if parser.title.strip() != expected_title:
            failures.append(f"{route}: incorrect title {parser.title.strip()!r}")
        if not parser.description or len(parser.description.strip()) < 40:
            failures.append(f"{route}: missing useful meta description")
        for phrase in SEARCH_PHRASES.get(route, ()):
            if phrase not in parser.title.lower():
                failures.append(f"{route}: title is missing {phrase!r}")
            if phrase not in parser.description.lower():
                failures.append(f"{route}: description is missing {phrase!r}")
            if phrase not in parser.body_text.lower():
                failures.append(f"{route}: prerendered body is missing {phrase!r}")
        if parser.canonical != expected_canonical:
            failures.append(f"{route}: incorrect canonical {parser.canonical!r}")
        if not parser.h1.strip():
            failures.append(f"{route}: missing prerendered H1")
        if route in ("/", "/tools") and "" in parser.image_sources:
            failures.append(f"{route}: contains an empty image source")
        if route in ("/", "/tools"):
            for prefix in CARD_IMAGE_PREFIXES:
                matching_sources = [
                    source
                    for source in parser.image_sources
                    if source.startswith(prefix) and source.endswith(".svg")
                ]
                if not matching_sources:
                    failures.append(f"{route}: missing bundled card image {prefix}*.svg")
                    continue
                for source in matching_sources:
                    if not (public_dir / source.lstrip("/")).exists():
                        failures.append(f"{route}: missing referenced card image {source}")

    for static_name in ("robots.txt", "sitemap.xml", "_redirects"):
        if not (public_dir / static_name).exists():
            failures.append(f"missing deployed static file: {static_name}")

    if failures:
        raise SystemExit("SSG validation failed:\n- " + "\n- ".join(failures))
    print(f"Validated {len(PAGES)} prerendered routes in {public_dir}")


if __name__ == "__main__":
    main()
