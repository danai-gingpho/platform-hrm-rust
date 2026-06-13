import os

def fix_mod_rs(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    if 'pub mod model;' not in content:
        new_content = 'pub mod model;\n' + content
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Fixed {file_path}")

def main():
    base_dir = 'platform/src/infrastructure/owner/db'
    for item in os.listdir(base_dir):
        item_path = os.path.join(base_dir, item)
        if os.path.isdir(item_path):
            mod_rs = os.path.join(item_path, 'mod.rs')
            if os.path.exists(mod_rs):
                fix_mod_rs(mod_rs)

if __name__ == "__main__":
    main()
