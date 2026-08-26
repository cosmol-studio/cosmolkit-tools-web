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
    "/blog": "COSMolKit Blog — Rust Cheminformatics Notes",
    "/rust-cheminformatics": "Rust Cheminformatics Beyond RDKit Bindings | COSMolKit",
    "/rdkit-alternative-rust": "Rust Cheminformatics State Management and Molecular Mutation | COSMolKit",
    "/rust-cheminformatics-libraries": "Rust Cheminformatics: Porting RDKit Source Semantics to Rust | COSMolKit",
    "/validation": "Rust Cheminformatics Validation on ChEMBL 37 | COSMolKit",
}
PLACEHOLDER_PAGES = {}
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
        self.title_count = 0
        self.title = ""
        self.h1 = ""
        self.body_text = ""
        self.description = None
        self.descriptions = []
        self.canonical = None
        self.canonicals = []
        self.keywords = []
        self.robots = None
        self.robots_values = []
        self.open_graph = {}
        self.empty_resource_links = []
        self.image_sources = []
        self.card_icons = []
        self.current_select = None
        self.selected_options = {}

    def handle_starttag(self, tag, attrs):
        attributes = dict(attrs)
        if tag == "title":
            self.in_title = True
            self.title_count += 1
        elif tag == "body":
            self.in_body = True
        elif tag == "h1":
            self.in_h1 = True
        elif tag == "meta" and attributes.get("name") == "description":
            self.description = attributes.get("content")
            self.descriptions.append(self.description)
        elif tag == "meta" and attributes.get("name") == "keywords":
            self.keywords.append(attributes.get("content"))
        elif tag == "meta" and attributes.get("name") == "robots":
            self.robots = attributes.get("content")
            self.robots_values.append(self.robots)
        elif tag == "meta" and attributes.get("property"):
            prop = attributes["property"]
            self.open_graph.setdefault(prop, []).append(attributes.get("content"))
        elif tag == "link":
            rel = attributes.get("rel")
            href = attributes.get("href")
            if rel == "canonical":
                self.canonical = href
                self.canonicals.append(href)
            if rel in {"icon", "stylesheet", "preload"} and not href:
                self.empty_resource_links.append(rel)
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


GLOBAL_SEARCH_TERMS = ("rust", "cheminformatics", "cosmolkit")
MIN_META_DESCRIPTION_LENGTH = 120
MAX_META_DESCRIPTION_LENGTH = 160
OPEN_GRAPH_PROPERTIES = ("og:title", "og:description", "og:url", "og:type")


def validate_shared_metadata(route, parser, failures):
    if parser.title_count != 1:
        failures.append(f"{route}: expected one title, found {parser.title_count}")

    if len(parser.descriptions) != 1:
        failures.append(
            f"{route}: expected one meta description, found {len(parser.descriptions)}"
        )
    elif not (
        MIN_META_DESCRIPTION_LENGTH
        <= len(parser.description.strip())
        <= MAX_META_DESCRIPTION_LENGTH
    ):
        failures.append(
            f"{route}: meta description length {len(parser.description.strip())} is outside "
            f"{MIN_META_DESCRIPTION_LENGTH}-{MAX_META_DESCRIPTION_LENGTH} characters"
        )

    if len(parser.canonicals) != 1:
        failures.append(
            f"{route}: expected one canonical link, found {len(parser.canonicals)}"
        )
    elif parser.canonical != canonical_for(route):
        failures.append(f"{route}: incorrect canonical {parser.canonical!r}")

    if len(parser.keywords) != 1:
        failures.append(f"{route}: expected one keywords tag, found {len(parser.keywords)}")

    for prop in OPEN_GRAPH_PROPERTIES:
        count = len(parser.open_graph.get(prop, []))
        if count != 1:
            failures.append(f"{route}: expected one {prop} tag, found {count}")

    if parser.empty_resource_links:
        failures.append(
            f"{route}: empty resource links for {', '.join(parser.empty_resource_links)}"
        )

    normalized_description = (parser.description or "").lower()
    normalized_body = parser.body_text.lower()
    normalized_keywords = " ".join(parser.keywords).lower()
    for term in GLOBAL_SEARCH_TERMS:
        if term not in normalized_description:
            failures.append(f"{route}: description is missing global term {term!r}")
        if term not in normalized_body:
            failures.append(f"{route}: prerendered body is missing global term {term!r}")
        if term not in normalized_keywords:
            failures.append(f"{route}: keywords are missing global term {term!r}")


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
        validate_shared_metadata(route, parser, failures)
        if parser.robots and "noindex" in parser.robots.lower():
            failures.append(f"{route}: published page must not be noindex")
        if parser.title.strip() != expected_title:
            failures.append(f"{route}: incorrect title {parser.title.strip()!r}")
        for phrase in SEARCH_PHRASES.get(route, ()):
            if phrase not in parser.title.lower():
                failures.append(f"{route}: title is missing {phrase!r}")
            if phrase not in parser.description.lower():
                failures.append(f"{route}: description is missing {phrase!r}")
            if phrase not in parser.body_text.lower():
                failures.append(f"{route}: prerendered body is missing {phrase!r}")
        if route == "/rust-cheminformatics":
            for phrase in (
                "value semantics",
                "copy-on-write",
                "opparts",
                "repository references",
            ):
                if phrase not in parser.body_text.lower():
                    failures.append(
                        f"{route}: prerendered article is missing {phrase!r}"
                    )
        if route == "/rdkit-alternative-rust":
            for phrase in (
                "operation contracts",
                "opparts",
                "removehs(sanitize=false)",
                "repository references",
            ):
                if phrase not in parser.body_text.lower():
                    failures.append(
                        f"{route}: prerendered article is missing {phrase!r}"
                    )
        if route == "/rust-cheminformatics-libraries":
            for phrase in (
                "corpus acts as auditor",
                "source-reproduction protocol",
                "first divergent state boundary",
                "repository references",
            ):
                if phrase not in parser.body_text.lower():
                    failures.append(
                        f"{route}: prerendered article is missing {phrase!r}"
                    )
        if route == "/validation":
            for phrase in (
                "three corpus layers",
                "the most useful result was a failure",
                "2026-08-20",
                "binary roundtrips",
                "validation is useful precisely because it is allowed to fail the project",
            ):
                if phrase not in parser.body_text.lower():
                    failures.append(
                        f"{route}: prerendered article is missing {phrase!r}"
                    )
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
        if route == "/smiles-to-svg" and any(
            source.startswith("data:image/svg+xml") for source in parser.image_sources
        ):
            failures.append(
                "/smiles-to-svg: prerendered SVG data URL is unsafe for hydration"
            )
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
        validate_shared_metadata(route, parser, failures)
        if parser.title.strip() != expected_title:
            failures.append(f"{route}: incorrect title {parser.title.strip()!r}")
        if len(parser.robots_values) != 1:
            failures.append(
                f"{route}: expected one robots tag, found {len(parser.robots_values)}"
            )
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
