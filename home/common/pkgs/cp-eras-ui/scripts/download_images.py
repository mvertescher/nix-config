#!/usr/bin/env python3
"""Fetch the full-res Behance sources for `images/` (gitignored).

Ids are `<6 hex>118663901.<13 hex>` from the Part-1 gallery; the
position numbers and the reasoning behind each assignment live in
`docs/sources.md`. The two entropism names are kept the wrong way
round on purpose — `docs/sources.md` and the SVG headers already
refer to them by these names.
"""
import urllib.request
import os
import argparse

IMAGES = [
    # neomil (#53-62)
    # ("img-00-login.png", "5707b7118663901.60e5fa6f09768.png"),
    # ("img-01-dashboard.png", "065b4f118663901.60e5fa6a80a6b.png"),
    # ("img-02-emails.png", "e0d82b118663901.60901b222676f.png"),
    # ("img-03-matrix.png", "c0c628118663901.60e5fa6a810c2.png"),
    # ("img-04-store.png", "cfff3f118663901.60901b22240e8.png"),
    # ("img-05-chat.png", "ea286f118663901.60e5fa6a7fab1.png"),
    ("img-06-private.png", "6cfb20118663901.60901b2225a24.png"),   # #59 login (three user cards)
    ("img-07-dashboard.png", "3fc4ef118663901.60e5fa6a7f2f7.png"), # #60 hub
    ("img-08-main.png", "c2e462118663901.60e5fa6a80470.png"),      # #61 mail
    ("img-09-store.png", "2ff48a118663901.60901b22249d2.png"),     # #62 store
    # entropism (#34-42)
    ("entropism-login.png", "9c9903118663901.60901b1b61cb4.png"),     # #39
    ("entropism-store.png", "a1de39118663901.60e5fa609775f.png"),     # #40 — the HUB (name swapped)
    ("entropism-mail.png", "48e1d6118663901.60e5fa6098aa5.png"),      # #41
    ("entropism-dashboard.png", "360994118663901.60901b1b6119b.png"), # #42 — the STORE (name swapped)
    # kitsch (#44-52)
    ("kitsch-dashboard.png", "e6ea35118663901.60e5fa669c12d.png"),    # #49 hub
    ("kitsch-login.png", "0bf802118663901.60e5fa669a019.png"),        # #50
    ("kitsch-mail.png", "fd108d118663901.60e5fa669a7cd.png"),         # #51
    ("kitsch-store.png", "75b8de118663901.60e5fa669b49a.png"),        # #52
    # neokitsch (#64-72)
    ("neokitsch-dashboard.png", "17a5c4118663901.60e5fa6e30417.png"), # #69 hub
    ("neokitsch-login.png", "a43e76118663901.60901b230a734.png"),     # #70
    ("neokitsch-mail.png", "f1104d118663901.60e5fa6e2fce8.png"),      # #71
    ("neokitsch-store.png", "ca9fd2118663901.60901b230b10d.png"),     # #72
]

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    default_dest = os.path.abspath(os.path.join(script_dir, "..", "images"))

    parser = argparse.ArgumentParser(description="Download the cp-eras UI design sources from Behance.")
    parser.add_argument("-o", "--output", default=default_dest, help=f"Destination folder (defaults to '{default_dest}')")
    parser.add_argument("-f", "--force", action="store_true", help="Re-download files that already exist")
    args = parser.parse_args()

    dest_dir = os.path.abspath(args.output)
    os.makedirs(dest_dir, exist_ok=True)

    headers = {'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36'}

    for name, img_id in IMAGES:
        filepath = os.path.join(dest_dir, name)
        if os.path.exists(filepath) and not args.force:
            print(f"Skipping {name} (exists)")
            continue
        downloaded = False
        for module_type in ["source", "fs", "max_1200"]:
            url = f"https://mir-s3-cdn-cf.behance.net/project_modules/{module_type}/{img_id}"
            req = urllib.request.Request(url, headers=headers)
            try:
                with urllib.request.urlopen(req) as response:
                    with open(filepath, 'wb') as out_file:
                        out_file.write(response.read())
                print(f"Downloaded {name} ({module_type})")
                downloaded = True
                break
            except Exception:
                pass
        if not downloaded:
            print(f"Error: Failed to download {name}")

if __name__ == "__main__":
    main()
