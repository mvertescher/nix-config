import sys
import os
from PIL import Image, ImageChops, ImageStat

def shift_image(img, dx, dy, bg_color):
    """Shift image by (dx, dy) and fill the void with bg_color (no wrap-around)."""
    shifted = Image.new(img.mode, img.size, bg_color)
    # Calculate the region to paste
    # If dx is positive, we paste at x=dx. If negative, we crop the left and paste at x=0.
    # PIL's paste handles negative coordinates and out-of-bounds automatically if we pass the box!
    # Actually, paste with a simple tuple (x, y) works with negative values in newer PIL,
    # but to be safe and compatible, we can just use:
    shifted.paste(img, (dx, dy))
    return shifted

def compute_similarity(img1, img2):
    """Compute similarity percentage between two images."""
    diff = ImageChops.difference(img1, img2)
    stat = ImageStat.Stat(diff)
    mean_diff = sum(stat.mean) / len(stat.mean)
    similarity = (1.0 - (mean_diff / 255.0)) * 100.0
    return similarity, diff, mean_diff

def main():
    if len(sys.argv) < 4:
        print("Usage: visual_diff.py <reference.png> <render.png> <output_diff.png> [enhance_factor]")
        sys.exit(1)

    ref_path = sys.argv[1]
    render_path = sys.argv[2]
    out_path = sys.argv[3]
    enhance_factor = float(sys.argv[4]) if len(sys.argv) > 4 else 1.0

    if not os.path.exists(ref_path):
        print(f"Error: Reference image not found: {ref_path}")
        sys.exit(1)
    if not os.path.exists(render_path):
        print(f"Error: Render image not found: {render_path}")
        sys.exit(1)

    # Load and convert to RGB
    img_ref = Image.open(ref_path).convert("RGB")
    img_render = Image.open(render_path).convert("RGB")

    # Resize render to match reference size for comparison
    if img_ref.size != img_render.size:
        print(f"Warning: Images have different sizes. Resizing render {img_render.size} to match reference {img_ref.size}")
        img_render = img_render.resize(img_ref.size, Image.Resampling.LANCZOS)

    # Detect background color from the top-left corner of the reference
    bg_color = img_ref.getpixel((0, 0))

    print("Finding best alignment...")
    best_similarity = -1.0
    best_offset = (0, 0)
    best_diff = None
    best_mean_diff = 0.0

    # Search range for translation alignment (dx, dy)
    search_range = 60
    
    # We brute-force search the best offset
    for dx in range(-search_range, search_range + 1):
        for dy in range(-search_range, search_range + 1):
            shifted_render = shift_image(img_render, dx, dy, bg_color)
            similarity, diff, mean_diff = compute_similarity(img_ref, shifted_render)
            if similarity > best_similarity:
                best_similarity = similarity
                best_offset = (dx, dy)
                best_diff = diff
                best_mean_diff = mean_diff

    print(f"Best Alignment Offset: dx={best_offset[0]}, dy={best_offset[1]}")
    print(f"Aligned Similarity Score: {best_similarity:.2f}% (Average pixel diff: {best_mean_diff:.2f}/255)")

    # Save the diff at the best alignment
    if enhance_factor != 1.0:
        enhanced_diff = best_diff.point(lambda p: min(int(p * enhance_factor), 255))
        enhanced_diff.save(out_path)
        print(f"Saved ENHANCED (x{enhance_factor}) aligned diff image to: {out_path}")
    else:
        best_diff.save(out_path)
        print(f"Saved raw aligned diff image to: {out_path}")

    # Also save the aligned render for reference
    aligned_render = shift_image(img_render, best_offset[0], best_offset[1], bg_color)
    aligned_render_path = os.path.splitext(out_path)[0] + "_aligned_render.png"
    aligned_render.save(aligned_render_path)
    print(f"Saved aligned render to: {aligned_render_path}")

if __name__ == "__main__":
    main()
