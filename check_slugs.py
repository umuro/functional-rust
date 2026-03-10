import sys, os, re

def load_slugs():
    with open('/tmp/slugs.txt', 'r') as f:
        return [line.strip() for line in f if line.strip()]

def load_queue_titles():
    titles = []
    with open('QUEUE.md', 'r') as f:
        for line in f:
            m = re.match(r'^### \d+: (.+)$', line.strip())
            if m:
                titles.append(m.group(1).lower())
    return titles

def main():
    slugs = load_slugs()
    titles = load_queue_titles()
    print(f"Total slugs: {len(slugs)}")
    print(f"Total existing titles: {len(titles)}")
    # For each slug, see if any title contains slug-like words
    added = []
    for slug in slugs:
        # Convert slug to title-like words: hello-world -> Hello World
        title_guess = ' '.join(word.capitalize() for word in slug.split('-'))
        # Check if title_guess appears in titles (or partial)
        found = any(title_guess.lower() in title for title in titles)
        if not found:
            added.append(slug)
            print(f"New: {slug} -> {title_guess}")
    print(f"\nCandidates: {len(added)}")
    # Select top 15
    for i, slug in enumerate(added[:15]):
        print(f"{i+1}: {slug}")

if __name__ == '__main__':
    main()