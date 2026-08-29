"""Flatten Dioxus SSG route directories for Cloudflare Pages clean URLs."""

from pathlib import Path
import shutil
import sys


def flatten_routes(public_dir: Path) -> int:
    flattened = 0
    for route_dir in sorted(public_dir.iterdir()):
        if not route_dir.is_dir():
            continue

        index = route_dir / "index.html"
        if not index.exists():
            continue

        children = list(route_dir.iterdir())
        if children != [index]:
            names = ", ".join(child.name for child in children)
            raise ValueError(
                f"route directory contains unexpected files: {route_dir} ({names})"
            )

        destination = public_dir / f"{route_dir.name}.html"
        if destination.exists():
            raise FileExistsError(f"cannot flatten {route_dir}: {destination} exists")

        shutil.move(str(index), str(destination))
        route_dir.rmdir()
        flattened += 1

    return flattened


def main() -> None:
    public_dir = Path(sys.argv[1] if len(sys.argv) > 1 else "deploy/web/public")
    if not public_dir.is_dir():
        raise SystemExit(f"public directory does not exist: {public_dir}")
    print(f"Flattened {flatten_routes(public_dir)} SSG route directories in {public_dir}")


if __name__ == "__main__":
    main()
