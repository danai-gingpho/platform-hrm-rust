import os

def replace_in_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    new_content = content.replace('Tenant', 'Company')
    new_content = new_content.replace('tenant', 'company')
    new_content = new_content.replace('x-tenant-id', 'x-company-id')
    # tenant_ is already covered by tenant -> company, 
    # but let's be explicit if there are any specific ones we want to be sure about.
    # Actually, the user mentioned tenant_ prefix specifically.
    
    if content != new_content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Updated {file_path}")

def main():
    for root, dirs, files in os.walk('platform'):
        for file in files:
            if file.endswith(('.rs', '.proto', '.md', '.toml')):
                file_path = os.path.join(root, file)
                replace_in_file(file_path)

if __name__ == "__main__":
    main()
