from html.parser import HTMLParser
from pathlib import Path
import sys


SITE_ORIGIN = "https://tools.cosmol.org"
PAGES = {
    "/": "COSMolKit Tools — Browser-Native Cheminformatics",
    "/tools": "Free Molecular & Cheminformatics Tools — COSMolKit",
    "/tools/smiles-to-svg": "SMILES to SVG — Molecular Structure Renderer | COSMolKit",
    "/tools/format-converter": "Molecular Format Converter — SMILES, SDF, MOL2, PDB, mmCIF | COSMolKit",
    "/tools/conformer-generator": "SMILES to 3D Conformer — Browser ETKDG Generator | COSMolKit",
    "/tools/inchi": "InChI Converter — InChI, InChIKey & Molecular Structure | COSMolKit",
    "/ecosystem": "COSMol Ecosystem — COSMolKit, Viewer & Browser Tools",
}


class SeoParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.in_title = False
        self.in_h1 = False
        self.title = ""
        self.h1 = ""
        self.description = None
        self.canonical = None

    def handle_starttag(self, tag, attrs):
        attributes = dict(attrs)
        if tag == "title":
            self.in_title = True
        elif tag == "h1":
            self.in_h1 = True
        elif tag == "meta" and attributes.get("name") == "description":
            self.description = attributes.get("content")
        elif tag == "link" and attributes.get("rel") == "canonical":
            self.canonical = attributes.get("href")

    def handle_endtag(self, tag):
        if tag == "title":
            self.in_title = False
        elif tag == "h1":
            self.in_h1 = False

    def handle_data(self, data):
        if self.in_title:
            self.title += data
        if self.in_h1:
            self.h1 += data


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
        if parser.canonical != expected_canonical:
            failures.append(f"{route}: incorrect canonical {parser.canonical!r}")
        if not parser.h1.strip():
            failures.append(f"{route}: missing prerendered H1")

    for static_name in ("robots.txt", "sitemap.xml"):
        if not (public_dir / static_name).exists():
            failures.append(f"missing deployed static file: {static_name}")

    if failures:
        raise SystemExit("SSG validation failed:\n- " + "\n- ".join(failures))
    print(f"Validated {len(PAGES)} prerendered routes in {public_dir}")


if __name__ == "__main__":
    main()
