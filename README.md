# cosmolkit-tools-web
Web-based COSMolKit tools for browser-native molecular parsing, visualization, conversion, and analysis via WASM.

## Serving Your App

```bash
dx serve --web
```

To test route-level WASM chunks locally, opt into the production splitting feature:

```bash
dx serve --web --features wasm-split --wasm-split
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

The script checks the package version declared in `Cargo.toml`, every Format Converter input/output combination, and the SMILES to SVG, molecular properties, and SMILES canonicalizer examples.

## SEO and static generation

Production uses Dioxus 0.7 static site generation so each public route contains its title, description, canonical URL, Open Graph metadata, H1, and page text before WebAssembly loads:

```bash
dx build --release --web --fullstack true --ssg --features "ssg wasm-split" --wasm-split --force-sequential --debug-symbols false
mkdir -p deploy/web/public
cp -R target/dx/cosmolkit-tools-web/release/web/public/. deploy/web/public/
cp robots.txt deploy/web/public/robots.txt
cp sitemap.xml deploy/web/public/sitemap.xml
cp _redirects deploy/web/public/_redirects
cp assets/benzene.svg deploy/web/public/assets/benzene.svg
cp assets/sdf.svg deploy/web/public/assets/sdf.svg
python scripts/check_ssg_output.py deploy/web/public
```

Route metadata is declared explicitly with the small `Seo` component at the top of each page. The `static_routes` server function supplies Dioxus with the routes to prerender; the generated `deploy/web/public` directory remains a static site and is deployed directly to Cloudflare Pages. Legacy tool URLs under `/tools/...` are permanently redirected to the current root-level tool routes through `_redirects`.

The project-level `ssg` feature is deliberately an empty compile-time marker. Dioxus CLI enables the existing `server` feature only for its native prerender target, while the WASM target remains an ordinary Web client with route-level splitting. Normal `dx serve --web` development therefore builds only the browser client and does not compile or start a native server.

Dioxus 0.7.10 cannot currently hydrate route-split output correctly ([DioxusLabs/dioxus#4631](https://github.com/DioxusLabs/dioxus/issues/4631)). The split client therefore replaces the prerendered mount after WASM starts instead of hydrating it. Search engines and no-JavaScript clients still receive the complete SSG document, and browser interactivity is mounted immediately afterward without the hydration crash. Revisit this workaround after the upstream issue is fixed.

`--fullstack true` is explicit so Dioxus CLI creates the temporary native renderer required by `--ssg`. The deployed result still contains only static files. The production command builds sequentially so the client bundle and chunks are complete before the native renderer writes the final prerendered HTML.

Dioxus CLI 0.7.10 performs prerendering through `dx build --ssg`; `dx bundle --ssg` currently does not invoke the SSG step. Keep the CLI and Dioxus dependency on the same version.

When a tool becomes public, add its route and page, provide unique `Seo` metadata, add its production URL to `sitemap.xml` and `tests/seo.rs`, then add the expected generated title to `scripts/check_ssg_output.py`. Keep unfinished routes such as Check PAINS out of the sitemap until their core capability is usable.
