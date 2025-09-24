#!/usr/bin/env python3
import os
import sys
import re
from pathlib import Path

def extract_frame_number(filename):
    """Extract frame number from filename like 'frame-5.bmp' -> 5"""
    match = re.search(r'frame-(\d+)\.bmp$', filename, re.IGNORECASE)
    return int(match.group(1)) if match else 0

def generate_animation_rust(target_path, output_path):
    target_dir = Path(target_path)
    output_file = Path(output_path)
    
    if not target_dir.exists():
        print(f"Error: Target directory '{target_path}' does not exist")
        return
    
    # Find all BMP files and sort by frame number
    bmp_files = [f for f in target_dir.glob("*.bmp")]
    bmp_files.sort(key=lambda x: extract_frame_number(x.name))
    
    if not bmp_files:
        print(f"No BMP files found in '{target_path}'")
        return
    
    # Get animation name from directory
    anim_name = target_dir.name
    
    # Generate Rust code
    rust_code = f"// Auto-generated animation: {anim_name}\n"
    rust_code += f"// Generated from: {target_path}\n"
    rust_code += f"// Frame count: {len(bmp_files)}\n\n"
    
    # Generate individual frame constants
    for i, bmp_file in enumerate(bmp_files):
        relative_path = os.path.relpath(bmp_file, output_file.parent)
        relative_path = relative_path.replace("\\", "/")  # Normalize path separators
        rust_code += f'const FRAME_{i}: &[u8] = include_bytes!("{relative_path}");\n'
    
    rust_code += "\n// Public array of all frames - count is automatically derived\n"
    rust_code += "pub const FRAMES: &[&[u8]] = &[\n"
    
    # Add frames to array (5 per line for readability)
    for i in range(0, len(bmp_files), 5):
        line_frames = []
        for j in range(i, min(i + 5, len(bmp_files))):
            line_frames.append(f"FRAME_{j}")
        
        if i + 5 >= len(bmp_files):
            rust_code += f"    {', '.join(line_frames)}\n"
        else:
            rust_code += f"    {', '.join(line_frames)},\n"
    
    rust_code += "];\n\n"
    rust_code += "// Helper function to get frame count\n"
    rust_code += "pub const fn frame_count() -> usize {\n"
    rust_code += "    FRAMES.len()\n"
    rust_code += "}\n"
    
    # Write to output file
    output_file.parent.mkdir(parents=True, exist_ok=True)
    with open(output_file, 'w') as f:
        f.write(rust_code)
    
    print(f"Generated {output_file} with {len(bmp_files)} frames")

def main():
    if len(sys.argv) != 3:
        print("Usage: python generate_bmp_src.py <target_path> <output_path>")
        print("Example: python generate_bmp_src.py include/nooo src/animations/nooo.rs")
        sys.exit(1)
    
    target_path = sys.argv[1]
    output_path = sys.argv[2]
    
    generate_animation_rust(target_path, output_path)

if __name__ == "__main__":
    main()