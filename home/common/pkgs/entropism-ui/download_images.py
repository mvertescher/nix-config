#!/usr/bin/env python3
import urllib.request
import os
import argparse

ENTROPISM_IMAGES = [
    ("img_00_f5dd1e118663901.60e5fa6f085fd.png", "f5dd1e118663901.60e5fa6f085fd.png"),
    ("img_01_274a8b118663901.60e5fa6097ef3.png", "274a8b118663901.60e5fa6097ef3.png"),
    ("img_02_264e0d118663901.60901b1b6088f.png", "264e0d118663901.60901b1b6088f.png"),
    ("img_03_a42bf7118663901.60901b1b6176a.png", "a42bf7118663901.60901b1b6176a.png"),
    ("img_04_30ba36118663901.60e5fa609903c.png", "30ba36118663901.60e5fa609903c.png"),
    ("img_05_3c1773118663901.60e5fa60984dc.png", "3c1773118663901.60e5fa60984dc.png"),
    ("img_06_9c9903118663901.60901b1b61cb4.png", "9c9903118663901.60901b1b61cb4.png"),
    ("img_07_a1de39118663901.60e5fa609775f.png", "a1de39118663901.60e5fa609775f.png"),
    ("img_08_48e1d6118663901.60e5fa6098aa5.png", "48e1d6118663901.60e5fa6098aa5.png"),
    ("img_09_360994118663901.60901b1b6119b.png", "360994118663901.60901b1b6119b.png"),
]

def main():
    parser = argparse.ArgumentParser(description="Download Entropism UI design images from Behance.")
    parser.add_argument("-o", "--output", default="entropism-ui", help="Destination folder (defaults to 'entropism-ui')")
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
