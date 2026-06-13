import os

def replace_in_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Domain
    new_content = content.replace('crate::domain::', 'crate::domain::owner::')
    # Application
    new_content = new_content.replace('crate::application::', 'crate::application::owner::')
    # Infrastructure
    new_content = new_content.replace('crate::infrastructure::db::', 'crate::infrastructure::owner::db::')
    # Interface
    new_content = new_content.replace('crate::interface::', 'crate::interface::owner::')
    # Proto
    new_content = new_content.replace('crate::owner::', 'crate::proto::owner::')
    
    # Special cases for infrastructure/owner/db/mod.rs
    if 'infrastructure/owner/db/mod.rs' in file_path:
        new_content = new_content.replace('pub mod staff;', 'pub mod staff;\npub mod role;\npub mod permission;\npub mod user_role;\npub mod role_permission;\npub mod seeder;')

    if content != new_content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Updated {file_path}")

def main():
    dirs_to_process = [
        'platform/src/application/owner',
        'platform/src/domain/owner',
        'platform/src/infrastructure/owner',
        'platform/src/interface/owner'
    ]
    for start_dir in dirs_to_process:
        for root, dirs, files in os.walk(start_dir):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    replace_in_file(file_path)

if __name__ == "__main__":
    main()
