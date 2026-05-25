import os
import re

categories = ['architecture', 'services', 'systems', 'applications', 'governance', 'infrastructure', 'company', 'reference', 'help']

def fix_links():
    total_fixed = 0
    for root_dir in ['content-wiki-documentation']:
        for dirpath, _, filenames in os.walk(root_dir):
            if '.git' in dirpath: continue
            for f in filenames:
                if f.endswith('.md'):
                    filepath = os.path.join(dirpath, f)
                    with open(filepath, 'r', encoding='utf-8') as file:
                        content = file.read()
                    
                    new_content = content
                    for cat in categories:
                        new_content = new_content.replace(f'](/wiki/{cat}/)', f'](/category/{cat})')
                        
                    if content != new_content:
                        with open(filepath, 'w', encoding='utf-8') as file:
                            file.write(new_content)
                        total_fixed += 1
                        print(f"Fixed category links in {f}")
                        
    print(f"Summary: Modified {total_fixed} files.")

if __name__ == '__main__':
    fix_links()
