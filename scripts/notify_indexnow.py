import argparse
import json
import os
import re
import time
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET
from pathlib import Path
from urllib.parse import urlparse


SITE_ORIGIN = "https://tools.cosmol.org"
INDEXNOW_ENDPOINT = "https://api.indexnow.org/indexnow"
KEY_PATTERN = re.compile(r"^[A-Za-z0-9-]{8,128}$")


def sitemap_urls(path):
    root = ET.parse(path).getroot()
    urls = [node.text.strip() for node in root.findall("{*}url/{*}loc") if node.text]
    if not urls or len(urls) != len(set(urls)):
        raise ValueError("sitemap URLs must be present and unique")
    if any(not url.startswith(f"{SITE_ORIGIN}/") for url in urls):
        raise ValueError(f"all sitemap URLs must use {SITE_ORIGIN}")
    return urls


def wait_for_key(key, key_location, attempts, delay):
    for attempt in range(1, attempts + 1):
        try:
            with urllib.request.urlopen(key_location, timeout=15) as response:
                if response.read().decode("utf-8").strip() == key:
                    print(f"IndexNow key is available at {key_location}")
                    return
        except (OSError, UnicodeError, urllib.error.HTTPError) as error:
            print(f"IndexNow key check {attempt}/{attempts} failed: {error}")
        if attempt < attempts:
            time.sleep(delay)
    raise RuntimeError(f"IndexNow key is not available at {key_location}")


def main():
    parser = argparse.ArgumentParser(description="Notify IndexNow from the production sitemap")
    parser.add_argument("--sitemap", default="sitemap.xml")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--attempts", type=int, default=12)
    parser.add_argument("--delay", type=int, default=5)
    args = parser.parse_args()

    key = os.environ.get("INDEXNOW_KEY", "")
    if not KEY_PATTERN.fullmatch(key):
        raise ValueError("INDEXNOW_KEY must be 8-128 letters, digits, or hyphens")

    key_file = Path(f"{key}.txt")
    if not key_file.is_file() or key_file.read_text(encoding="utf-8").strip() != key:
        raise ValueError(f"{key_file} must exist and contain the IndexNow key")

    key_location = f"{SITE_ORIGIN}/{key_file.name}"
    payload = {
        "host": urlparse(SITE_ORIGIN).netloc,
        "key": key,
        "keyLocation": key_location,
        "urlList": sitemap_urls(args.sitemap),
    }

    if args.dry_run:
        print(json.dumps(payload, indent=2))
        return

    wait_for_key(key, key_location, args.attempts, args.delay)
    request = urllib.request.Request(
        INDEXNOW_ENDPOINT,
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json; charset=utf-8"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            status = response.status
            body = response.read().decode("utf-8", errors="replace")
    except urllib.error.HTTPError as error:
        body = error.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"IndexNow returned HTTP {error.code}: {body}") from error

    if status not in (200, 202):
        raise RuntimeError(f"IndexNow returned unexpected HTTP {status}: {body}")
    print(f"IndexNow accepted {len(payload['urlList'])} URLs with HTTP {status}")


if __name__ == "__main__":
    main()
