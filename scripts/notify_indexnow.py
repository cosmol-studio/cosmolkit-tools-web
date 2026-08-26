import argparse
import json
import os
import re
import time
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET
from urllib.parse import urlparse


SITE_ORIGIN = "https://tools.cosmol.org"
INDEXNOW_ENDPOINT = "https://api.indexnow.org/indexnow"
KEY_PATTERN = re.compile(r"^[A-Za-z0-9-]{8,128}$")

USER_AGENT = "COSMolKit-IndexNow/1.0 (+https://tools.cosmol.org/)"


def sitemap_urls(path):
    root = ET.parse(path).getroot()
    urls = [
        node.text.strip()
        for node in root.findall("{*}url/{*}loc")
        if node.text
    ]

    if not urls:
        raise ValueError("sitemap must contain at least one URL")

    if len(urls) != len(set(urls)):
        raise ValueError("sitemap URLs must be unique")

    expected = urlparse(SITE_ORIGIN)

    for url in urls:
        parsed = urlparse(url)

        if (
            parsed.scheme != expected.scheme
            or parsed.netloc != expected.netloc
        ):
            raise ValueError(
                f"all sitemap URLs must use origin {SITE_ORIGIN}: {url}"
            )

    return urls


def response_debug(headers):
    return (
        f"server={headers.get('server')!r}, "
        f"cf-ray={headers.get('cf-ray')!r}, "
        f"cf-mitigated={headers.get('cf-mitigated')!r}, "
        f"cf-cache-status={headers.get('cf-cache-status')!r}, "
        f"content-type={headers.get('content-type')!r}"
    )


def wait_for_key(key, key_location, attempts, delay):
    for attempt in range(1, attempts + 1):
        request = urllib.request.Request(
            key_location,
            headers={
                "User-Agent": USER_AGENT,
                "Accept": "text/plain, */*;q=0.1",
                "Cache-Control": "no-cache",
            },
            method="GET",
        )

        try:
            with urllib.request.urlopen(request, timeout=15) as response:
                body = response.read().decode(
                    "utf-8",
                    errors="replace",
                ).strip()

                if body == key:
                    print(
                        f"IndexNow key is available at {key_location} "
                        f"(HTTP {response.status}; "
                        f"{response_debug(response.headers)})"
                    )
                    return

                print(
                    f"IndexNow key check {attempt}/{attempts} failed: "
                    f"HTTP {response.status}; "
                    f"{response_debug(response.headers)}; "
                    f"unexpected body={body[:500]!r}"
                )

        except urllib.error.HTTPError as error:
            body = error.read().decode(
                "utf-8",
                errors="replace",
            )

            print(
                f"IndexNow key check {attempt}/{attempts} failed: "
                f"HTTP {error.code} {error.reason}; "
                f"{response_debug(error.headers)}; "
                f"body={body[:1000]!r}"
            )

        except urllib.error.URLError as error:
            print(
                f"IndexNow key check {attempt}/{attempts} failed: "
                f"URL error: {error.reason!r}"
            )

        except (OSError, UnicodeError) as error:
            print(
                f"IndexNow key check {attempt}/{attempts} failed: "
                f"{type(error).__name__}: {error}"
            )

        if attempt < attempts:
            time.sleep(delay)

    raise RuntimeError(
        f"IndexNow key is not available at {key_location}"
    )


def submit_indexnow(payload):
    data = json.dumps(payload).encode("utf-8")

    request = urllib.request.Request(
        INDEXNOW_ENDPOINT,
        data=data,
        headers={
            "User-Agent": USER_AGENT,
            "Content-Type": "application/json; charset=utf-8",
            "Accept": "application/json, text/plain, */*",
        },
        method="POST",
    )

    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            status = response.status
            body = response.read().decode(
                "utf-8",
                errors="replace",
            )

            if status not in (200, 202):
                raise RuntimeError(
                    f"IndexNow returned unexpected HTTP {status}; "
                    f"{response_debug(response.headers)}; "
                    f"body={body[:1000]!r}"
                )

            return status, body

    except urllib.error.HTTPError as error:
        body = error.read().decode(
            "utf-8",
            errors="replace",
        )

        raise RuntimeError(
            f"IndexNow returned HTTP {error.code} {error.reason}; "
            f"{response_debug(error.headers)}; "
            f"body={body[:1000]!r}"
        ) from error

    except urllib.error.URLError as error:
        raise RuntimeError(
            f"IndexNow request failed: {error.reason!r}"
        ) from error


def main():
    parser = argparse.ArgumentParser(
        description="Notify IndexNow from the production sitemap"
    )
    parser.add_argument(
        "--sitemap",
        default="deployment/public/sitemap.xml",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
    )
    parser.add_argument(
        "--attempts",
        type=int,
        default=12,
    )
    parser.add_argument(
        "--delay",
        type=int,
        default=5,
    )
    args = parser.parse_args()

    if args.attempts < 1:
        raise ValueError("--attempts must be at least 1")

    if args.delay < 0:
        raise ValueError("--delay must not be negative")

    key = os.environ.get("INDEXNOW_KEY", "")

    if not KEY_PATTERN.fullmatch(key):
        raise ValueError(
            "INDEXNOW_KEY must be 8-128 letters, digits, or hyphens"
        )

    key_location = f"{SITE_ORIGIN}/{key}.txt"
    urls = sitemap_urls(args.sitemap)

    payload = {
        "host": urlparse(SITE_ORIGIN).netloc,
        "key": key,
        "keyLocation": key_location,
        "urlList": urls,
    }

    if args.dry_run:
        print(json.dumps(payload, indent=2))
        return

    wait_for_key(
        key=key,
        key_location=key_location,
        attempts=args.attempts,
        delay=args.delay,
    )

    status, body = submit_indexnow(payload)

    print(
        f"IndexNow accepted {len(urls)} URLs "
        f"with HTTP {status}"
    )

    if body.strip():
        print(f"IndexNow response: {body.strip()[:1000]}")


if __name__ == "__main__":
    main()
