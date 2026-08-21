# cosmolkit-tools-web
Web-based COSMolKit tools for browser-native molecular parsing, visualization, conversion, and analysis via WASM.

## Serving Your App

```bash
dx serve --web
```

For desktop debugging, run the frontend without rebuilding wasm:

```bash
dx serve --desktop --no-default-features --always-on-top false
```

## Python Example Regression Test

Run the Python examples against the COSMolKit package installed in the `COS` environment:

```powershell
conda activate COS
python scripts/test_python_examples.py
```

The script checks the package version declared in `Cargo.toml`, every Format Converter input/output combination, and the SMILES to SVG file example.

## SEO and static generation

Production uses Dioxus 0.7 static site generation so each public route contains its title, description, canonical URL, Open Graph metadata, H1, and page text before WebAssembly loads:

```bash
dx build --release --web --ssg --features ssg --force-sequential --debug-symbols false
mkdir -p deploy/web/public
cp -R target/dx/cosmolkit-tools-web/release/web/public/. deploy/web/public/
cp robots.txt deploy/web/public/robots.txt
cp sitemap.xml deploy/web/public/sitemap.xml
cp _redirects deploy/web/public/_redirects
python scripts/check_ssg_output.py deploy/web/public
```

Route metadata is declared explicitly with the small `Seo` component at the top of each page. The `static_routes` server function supplies Dioxus with the routes to prerender; the generated `deploy/web/public` directory remains a static site and is deployed directly to Cloudflare Pages. Legacy tool URLs under `/tools/...` are permanently redirected to the current root-level tool routes through `_redirects`.

The `ssg` feature enables Dioxus fullstack support only for the production prerender build. Normal `dx serve --web` development builds only the browser client and does not compile or start a native server. The production command builds sequentially so the client bundle is complete before the native renderer writes the final prerendered HTML.

Dioxus CLI 0.7.10 performs prerendering through `dx build --ssg`; `dx bundle --ssg` currently does not invoke the SSG step. Keep the CLI and Dioxus dependency on the same version.

When a tool becomes public, add its route and page, provide unique `Seo` metadata, add its production URL to `sitemap.xml` and `tests/seo.rs`, then add the expected generated title to `scripts/check_ssg_output.py`. Keep unfinished routes such as Check PAINS out of the sitemap until their core capability is usable.
