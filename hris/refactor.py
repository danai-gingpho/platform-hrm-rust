import os
import re

def camel_case(snake_str):
    components = snake_str.split('_')
    return "".join(x.title() for x in components)

domain_dir = 'src/domain'
dirs = [d for d in os.listdir(domain_dir) if os.path.isdir(os.path.join(domain_dir, d)) and d not in ['shared', 'errors', 'entity']]

for d in dirs:
    repo_path = os.path.join(domain_dir, d, 'repository.rs')
    if os.path.exists(repo_path):
        with open(repo_path, 'r') as f:
            content = f.read()
        
        entity_name = camel_case(d)
        # Special cases if any
        if d == 'employment_contract':
            entity_name = 'EmploymentContract'
        elif d == 'allowance_type':
            entity_name = 'AllowanceType'
        
        # We need to be careful with entity_name being a substring of others.
        # But in repository.rs it should be mostly the main entity.
        
        # Replace the import
        old_import = f"::entity::{entity_name}"
        new_import = "::entity::Model"
        content = content.replace(old_import, new_import)
        
        # Replace usages
        # Use regex to match whole word
        content = re.sub(rf'\b{entity_name}\b', 'Model', content)
        
        with open(repo_path, 'w') as f:
            f.write(content)
        print(f"Updated {repo_path}")

# Task 2: entity.rs files
for d in dirs:
    entity_path = os.path.join(domain_dir, d, 'entity.rs')
    if os.path.exists(entity_path):
        with open(entity_path, 'r') as f:
            content = f.read()
        
        # Replace super::<dir>::Entity with crate::domain::<dir>::Entity
        # Replace super::<dir>::Column with crate::domain::<dir>::Column
        
        content = re.sub(r'super::([a-z_]+)::Entity', r'crate::domain::\1::Entity', content)
        content = re.sub(r'super::([a-z_]+)::Column', r'crate::domain::\1::Column', content)
        
        # Task 3 & 4: decimal issues
        content = content.replace('use decimal::Decimal;', 'use rust_decimal::Decimal;')
        
        with open(entity_path, 'w') as f:
            f.write(content)
        print(f"Updated {entity_path}")

