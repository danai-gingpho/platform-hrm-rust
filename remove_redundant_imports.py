import os
import re

def remove_redundant_imports(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    new_lines = []
    # Match something like: use crate::infrastructure::owner::db::staff::model;
    pattern = re.compile(r'^use crate::infrastructure::owner::db::\w+::model;$')
    
    for line in lines:
        if not pattern.match(line.strip()):
            new_lines.append(line)

    if len(lines) != len(new_lines):
        with open(file_path, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)
        print(f"Removed redundant imports from {file_path}")

def main():
    base_dir = 'platform/src/infrastructure/owner/db'
    for root, dirs, files in os.walk(base_dir):
        for file in files:
            if file == 'mod.rs':
                remove_redundant_imports(os.path.join(root, file))

if __name__ == "__main__":
    main()
