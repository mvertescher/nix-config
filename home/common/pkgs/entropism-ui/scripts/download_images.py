#!/usr/bin/env python3
import urllib.request
import os
import argparse

ENTROPISM_IMAGES = [
    ("img-00-login.png", "f5dd1e118663901.60e5fa6f085fd.png"),
    ("img-01-dashboard.png", "274a8b118663901.60e5fa6097ef3.png"),
    ("img-02-emails.png", "264e0d118663901.60901b1b6088f.png"),
    ("img-03-matrix.png", "a42bf7118663901.60901b1b6176a.png"),
    ("img-04-store.png", "30ba36118663901.60e5fa609903c.png"),
    ("img-05-chat.png", "3c1773118663901.60e5fa60984dc.png"),
    ("img-06-private.png", "9c9903118663901.60901b1b61cb4.png"),
    ("img-07-devices.png", "a1de39118663901.60e5fa609775f.png"),
    ("img-08-network.png", "48e1d6118663901.60e5fa6098aa5.png"),
    ("img-09-terminal.png", "360994118663901.60901b1b6119b.png"),
]

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    default_dest = os.path.abspath(os.path.join(script_dir, "..", "images"))

    parser = argparse.ArgumentParser(description="Download Entropism UI design images from Behance.")
    parser.add_argument("-o", "--output", default=default_dest, help=f"Destination folder (defaults to '{default_dest}')")
    args = parser.parse_args()

    dest_dir = os.path.abspath(args.output)
    os.makedirs(dest_dir, exist_ok=True)

    headers = {'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36'}

    for name, img_id in ENTROPISM_IMAGES:
        downloaded = False
        for module_type in ["source", "fs", "max_1200"]:
            url = f"https://mir-s3-cdn-cf.behance.net/project_modules/{module_type}/{img_id}"
            req = urllib.request.Request(url, headers=headers)
            try:
                with urllib.request.urlopen(req) as response:
                    filepath = os.path.join(dest_dir, name)
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
