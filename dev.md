# Development

COSMolKit Tools uses Dioxus 0.7.10 and targets Rust/WebAssembly for the browser. Keep the Dioxus CLI and crate versions aligned.

## Local development

Install the WebAssembly target and Dioxus CLI, then start the browser development server:

```bash
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.10 --locked
dx serve --web
```

Normal `dx serve --web` development builds only the browser client. To test route-level WASM chunks locally, enable the production splitting feature:

```bash
dx serve --web --features wasm-split --wasm-split
```

For desktop debugging without rebuilding the WebAssembly target:

```bash
dx serve --desktop --no-default-features --always-on-top false
```

## Checks

Run the Rust checks before submitting a change:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

The Python example regression script checks the COSMolKit package version, every Format Converter input/output combination, and the examples for SMILES to SVG, molecular properties, and SMILES canonicalization. Run it with the repository's `COS` Conda environment:

```powershell
conda activate COS
python scripts/test_python_examples.py
```

## Production static build

Production uses Dioxus static site generation so every public route contains its metadata, heading, and useful page text before WebAssembly loads. The deployed output remains a static Cloudflare Pages site.

```bash
dx build --release --web --fullstack true --ssg --features "ssg wasm-split" --wasm-split --force-sequential --debug-symbols false
```

The generated static site is written to `target/dx/cosmolkit-tools-web/release/web/public`. The deployment workflow in `.github/workflows/depoly.yml` is the source of truth for assembling root static files, validating the generated pages, and publishing to Cloudflare Pages.

## Dioxus SSG notes

Route metadata is declared explicitly with the small `Seo` component at the top of each page. The `static_routes` server function supplies the routes to prerender. Legacy tool URLs under `/tools/...` are permanently redirected to the current root-level routes through `_redirects`.

The project-level `ssg` feature is an empty compile-time marker. Dioxus CLI enables the existing `server` feature only for its temporary native prerender target, while the WASM target remains an ordinary Web client with route-level splitting. `--fullstack true` is required for that native renderer; it does not add a server to the deployed site.

Dioxus 0.7.10 cannot currently hydrate route-split output correctly ([DioxusLabs/dioxus#4631](https://github.com/DioxusLabs/dioxus/issues/4631)). The split client replaces the prerendered mount after WebAssembly starts instead of hydrating it. Revisit this workaround after the upstream issue is fixed.

Use `dx build --ssg` for prerendering. In Dioxus CLI 0.7.10, `dx bundle --ssg` does not invoke the SSG step. The production build runs sequentially so the client bundle and route chunks are complete before the native renderer writes the final HTML.

## Adding a public route

When a tool becomes usable and public:

1. Add its route and page.
2. Add unique title, description, canonical URL, and Open Graph metadata with `Seo`.
3. Add the canonical production URL to `sitemap.xml` and the route expectations to `tests/seo.rs`.
4. Add the generated title expectation to `scripts/check_ssg_output.py`.
5. Keep unfinished routes out of the sitemap until their core capability is available.
