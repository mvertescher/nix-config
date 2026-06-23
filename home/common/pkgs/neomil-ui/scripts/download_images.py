#!/usr/bin/env python3
import urllib.request
import os
import argparse

NEOMIL_IMAGES = [
    ("img-00-login.png", "5707b7118663901.60e5fa6f09768.png"),
    ("img-01-dashboard.png", "065b4f118663901.60e5fa6a80a6b.png"),
    ("img-02-emails.png", "e0d82b118663901.60901b222676f.png"),
    ("img-03-matrix.png", "c0c628118663901.60e5fa6a810c2.png"),
    ("img-04-store.png", "cfff3f118663901.60901b22240e8.png"),
    ("img-05-chat.png", "ea286f118663901.60e5fa6a7fab1.png"),
    ("img-06-private.png", "6cfb20118663901.60901b2225a24.png"),
    ("img-07-devices.png", "3fc4ef118663901.60e5fa6a7f2f7.png"),
    ("img-08-network.png", "c2e462118663901.60e5fa6a80470.png"),
    ("img-09-terminal.png", "2ff48a118663901.60901b22249d2.png"),
]

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    default_dest = os.path.abspath(os.path.join(script_dir, "..", "images"))

    parser = argparse.ArgumentParser(description="Download Neomil UI design images from Behance.")
    parser.add_argument("-o", "--output", default=default_dest, help=f"Destination folder (defaults to '{default_dest}')")
    args = parser.parse_args()

    dest_dir = os.path.abspath(args.output)
    os.makedirs(dest_dir, exist_ok=True)

    headers = {'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36'}

    for name, img_id in NEOMIL_IMAGES:
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
