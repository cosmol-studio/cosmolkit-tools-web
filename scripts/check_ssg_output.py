from html.parser import HTMLParser
from pathlib import Path
import sys


SITE_ORIGIN = "https://tools.cosmol.org"
INDEXNOW_KEY_FILE = "b2d03e0f-cc00-4050-8e29-3316108be26b.txt"
PAGES = {
    "/": "COSMolKit — Browser-Native Cheminformatics Powered by Rust",
    "/tools": "Browser-Based Cheminformatics Tools Powered by Rust — COSMolKit",
    "/smiles-to-svg": "SMILES to SVG — Molecular Structure Renderer | COSMolKit",
    "/format-converter": "Molecular Format Converter — SDF, SMILES, MOL2, PDB | COSMolKit",
    "/smiles-converter": "SMILES Converter Online — Powered by Rust | COSMolKit",
    "/sdf-to-smiles": "SDF to SMILES Converter Online — Powered by Rust | COSMolKit",
    "/mol-to-smiles": "MOL to SMILES Converter Online — Powered by Rust | COSMolKit",
    "/mol2-to-smiles": "MOL2 to SMILES Converter Online — Powered by Rust | COSMolKit",
    "/pdb-to-smiles": "PDB to SMILES Converter Online — Powered by Rust | COSMolKit",
    "/mmcif-to-smiles": "mmCIF to SMILES Converter Online — Powered by Rust | COSMolKit",
    "/xyz-to-smiles": "XYZ to SMILES Converter Online — Powered by Rust | COSMolKit",
    "/smiles-to-sdf": "SMILES to SDF Converter Online — Powered by Rust | COSMolKit",
    "/smiles-to-sdf-v3000": "SMILES to SDF V3000 Converter Online — Powered by Rust | COSMolKit",
    "/smiles-to-mol": "SMILES to MOL Converter Online — Powered by Rust | COSMolKit",
    "/smiles-to-mol-v3000": "SMILES to MOL V3000 Converter Online — Powered by Rust | COSMolKit",
    "/smiles-to-pdb": "SMILES to PDB Converter Online — Powered by Rust | COSMolKit",
    "/mol-to-sdf": "MOL to SDF Converter Online — Powered by Rust | COSMolKit",
    "/mol-to-sdf-v3000": "MOL to SDF V3000 Converter Online — Powered by Rust | COSMolKit",
    "/mol-to-pdb": "MOL to PDB Converter Online — Powered by Rust | COSMolKit",
    "/mol-to-svg": "MOL to SVG Converter Online — Powered by Rust | COSMolKit",
    "/mol2-to-mol": "MOL2 to MOL Converter Online — Powered by Rust | COSMolKit",
    "/mol2-to-mol-v3000": "MOL2 to MOL V3000 Converter Online — Powered by Rust | COSMolKit",
    "/mol2-to-sdf": "MOL2 to SDF Converter Online — Powered by Rust | COSMolKit",
    "/mol2-to-sdf-v3000": "MOL2 to SDF V3000 Converter Online — Powered by Rust | COSMolKit",
    "/mol2-to-pdb": "MOL2 to PDB Converter Online — Powered by Rust | COSMolKit",
    "/sdf-to-mol": "SDF to MOL Converter Online — Powered by Rust | COSMolKit",
    "/sdf-to-mol-v3000": "SDF to MOL V3000 Converter Online — Powered by Rust | COSMolKit",
    "/sdf-to-pdb": "SDF to PDB Converter Online — Powered by Rust | COSMolKit",
    "/sdf-to-svg": "SDF to SVG Converter Online — Powered by Rust | COSMolKit",
    "/pdb-to-mol": "PDB to MOL Converter Online — Powered by Rust | COSMolKit",
    "/pdb-to-mol-v3000": "PDB to MOL V3000 Converter Online — Powered by Rust | COSMolKit",
    "/pdb-to-sdf": "PDB to SDF Converter Online — Powered by Rust | COSMolKit",
    "/pdb-to-sdf-v3000": "PDB to SDF V3000 Converter Online — Powered by Rust | COSMolKit",
    "/mmcif-to-mol": "mmCIF to MOL Converter Online — Powered by Rust | COSMolKit",
    "/mmcif-to-mol-v3000": "mmCIF to MOL V3000 Converter Online — Powered by Rust | COSMolKit",
    "/mmcif-to-sdf": "mmCIF to SDF Converter Online — Powered by Rust | COSMolKit",
    "/mmcif-to-sdf-v3000": "mmCIF to SDF V3000 Converter Online — Powered by Rust | COSMolKit",
    "/mmcif-to-pdb": "mmCIF to PDB Converter Online — Powered by Rust | COSMolKit",
    "/xyz-to-sdf": "XYZ to SDF Converter Online — Powered by Rust | COSMolKit",
    "/xyz-to-mol": "XYZ to MOL Converter Online — Powered by Rust | COSMolKit",
    "/xyz-to-mol-v3000": "XYZ to MOL V3000 Converter Online — Powered by Rust | COSMolKit",
    "/xyz-to-sdf-v3000": "XYZ to SDF V3000 Converter Online — Powered by Rust | COSMolKit",
    "/xyz-to-pdb": "XYZ to PDB Converter Online — Powered by Rust | COSMolKit",
    "/conformer-generator": "SMILES to 3D Conformer — Browser ETKDG Generator | COSMolKit",
    "/inchi": "InChI Converter — InChI, InChIKey & Molecular Structure | COSMolKit",
    "/molecular-properties": "Molecular Properties Calculator — MW, TPSA, logP | COSMolKit",
    "/smiles-canonicalizer": "SMILES Canonicalizer — Canonical & Isomeric SMILES | COSMolKit",
    "/ecosystem": "COSMol Ecosystem — Rust-Powered Cheminformatics & Browser-Native Tools",
}
PLACEHOLDER_PAGES = {
    "/blog": "COSMolKit Blog — Rust Cheminformatics Notes",
    "/rust-cheminformatics": "Rust Cheminformatics — COSMolKit Blog",
    "/rdkit-alternative-rust": "RDKit Alternative in Rust — COSMolKit Blog",
    "/rust-cheminformatics-libraries": "Rust Cheminformatics Libraries — COSMolKit Blog",
    "/validation": "Cheminformatics Software Validation — COSMolKit Blog",
}
CONVERSION_PRESETS = {
    "/smiles-converter": ("smiles", "sdf-v2000"),
    "/sdf-to-smiles": ("sdf", "smiles"),
    "/mol-to-smiles": ("mol", "smiles"),
    "/mol2-to-smiles": ("mol2", "smiles"),
    "/pdb-to-smiles": ("pdb", "smiles"),
    "/mmcif-to-smiles": ("mmcif", "smiles"),
    "/xyz-to-smiles": ("xyz", "smiles"),
    "/smiles-to-sdf": ("smiles", "sdf-v2000"),
    "/smiles-to-sdf-v3000": ("smiles", "sdf-v3000"),
    "/smiles-to-mol": ("smiles", "mol-v2000"),
    "/smiles-to-mol-v3000": ("smiles", "mol-v3000"),
    "/smiles-to-pdb": ("smiles", "pdb"),
    "/mol-to-sdf": ("mol", "sdf-v2000"),
    "/mol-to-sdf-v3000": ("mol", "sdf-v3000"),
    "/mol-to-pdb": ("mol", "pdb"),
    "/mol-to-svg": ("mol", "svg"),
    "/mol2-to-mol": ("mol2", "mol-v2000"),
    "/mol2-to-mol-v3000": ("mol2", "mol-v3000"),
    "/mol2-to-sdf": ("mol2", "sdf-v2000"),
    "/mol2-to-sdf-v3000": ("mol2", "sdf-v3000"),
    "/mol2-to-pdb": ("mol2", "pdb"),
    "/sdf-to-mol": ("sdf", "mol-v2000"),
    "/sdf-to-mol-v3000": ("sdf", "mol-v3000"),
    "/sdf-to-pdb": ("sdf", "pdb"),
    "/sdf-to-svg": ("sdf", "svg"),
    "/pdb-to-mol": ("pdb", "mol-v2000"),
    "/pdb-to-mol-v3000": ("pdb", "mol-v3000"),
    "/pdb-to-sdf": ("pdb", "sdf-v2000"),
    "/pdb-to-sdf-v3000": ("pdb", "sdf-v3000"),
    "/mmcif-to-mol": ("mmcif", "mol-v2000"),
    "/mmcif-to-mol-v3000": ("mmcif", "mol-v3000"),
    "/mmcif-to-sdf": ("mmcif", "sdf-v2000"),
    "/mmcif-to-sdf-v3000": ("mmcif", "sdf-v3000"),
    "/mmcif-to-pdb": ("mmcif", "pdb"),
    "/xyz-to-sdf": ("xyz", "sdf-v2000"),
    "/xyz-to-mol": ("xyz", "mol-v2000"),
    "/xyz-to-mol-v3000": ("xyz", "mol-v3000"),
    "/xyz-to-sdf-v3000": ("xyz", "sdf-v3000"),
    "/xyz-to-pdb": ("xyz", "pdb"),
}
SEARCH_PHRASES = {
    "/": ("rust", "cheminformatics", "browser-native"),
    "/tools": ("rust", "cheminformatics", "browser-based"),
    "/ecosystem": ("rust", "cheminformatics", "browser-native"),
}
CARD_ICONS = {
    "/": ("depiction", "format", "conformer", "identifier"),
    "/tools": (
        "depiction",
        "format",
        "conformer",
        "identifier",
        "properties",
        "canonical",
        "filter-alert",
    ),
}


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
        self.robots = None
        self.image_sources = []
        self.card_icons = []
        self.current_select = None
        self.selected_options = {}

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
        elif tag == "meta" and attributes.get("name") == "robots":
            self.robots = attributes.get("content")
        elif tag == "link" and attributes.get("rel") == "canonical":
            self.canonical = attributes.get("href")
        elif tag == "img":
            self.image_sources.append(attributes.get("src", ""))
        elif tag == "svg" and attributes.get("data-card-icon"):
            self.card_icons.append(attributes["data-card-icon"])
        elif tag == "select":
            self.current_select = attributes.get("id")
        elif tag == "option" and self.current_select and "selected" in attributes:
            self.selected_options[self.current_select] = attributes.get("value")

    def handle_endtag(self, tag):
        if tag == "title":
            self.in_title = False
        elif tag == "body":
            self.in_body = False
        elif tag == "h1":
            self.in_h1 = False
        elif tag == "select":
            self.current_select = None

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
        if route in CONVERSION_PRESETS:
            expected_input, expected_output = CONVERSION_PRESETS[route]
            if parser.selected_options.get("input-format") != expected_input:
                failures.append(f"{route}: incorrect selected input format")
            if parser.selected_options.get("output-format") != expected_output:
                failures.append(f"{route}: incorrect selected output format")
        if route in ("/", "/tools") and "" in parser.image_sources:
            failures.append(f"{route}: contains an empty image source")
        if route in CARD_ICONS:
            for icon in CARD_ICONS[route]:
                if icon not in parser.card_icons:
                    failures.append(f"{route}: missing embedded card icon {icon}")

    for route, expected_title in PLACEHOLDER_PAGES.items():
        html_path = public_dir / route.lstrip("/") / "index.html"
        if not html_path.exists():
            failures.append(f"missing generated placeholder page: {html_path}")
            continue

        parser = SeoParser()
        parser.feed(html_path.read_text(encoding="utf-8"))
        if parser.title.strip() != expected_title:
            failures.append(f"{route}: incorrect title {parser.title.strip()!r}")
        if parser.canonical != canonical_for(route):
            failures.append(f"{route}: incorrect canonical {parser.canonical!r}")
        if parser.robots != "noindex, follow":
            failures.append(f"{route}: placeholder must be noindex, follow")
        if not parser.h1.strip():
            failures.append(f"{route}: missing prerendered H1")

    known_routes = set(PAGES) | set(PLACEHOLDER_PAGES)
    for html_path in public_dir.glob("*/index.html"):
        route = f"/{html_path.parent.name}"
        if "-to-" in route and route not in known_routes:
            failures.append(f"unsupported conversion page was prerendered: {route}")

    for static_name in ("robots.txt", "sitemap.xml", "_redirects", INDEXNOW_KEY_FILE):
        if not (public_dir / static_name).exists():
            failures.append(f"missing deployed static file: {static_name}")

    if failures:
        raise SystemExit("SSG validation failed:\n- " + "\n- ".join(failures))
    page_count = len(PAGES) + len(PLACEHOLDER_PAGES)
    print(f"Validated {page_count} prerendered routes in {public_dir}")


if __name__ == "__main__":
    main()
