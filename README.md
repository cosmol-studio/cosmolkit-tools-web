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
