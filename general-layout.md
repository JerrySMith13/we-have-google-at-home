okay soooo basically


- Step 1: Users enter a list of URIs and the program begins crawling them
- Step 2: Once pages are indexed, begin parsing into search terms. Each page gets locally stored in a separate db
- Step 3: Parameters fed into meilesearch database with path to content
- Step 4: Meilesearch handles the rest


# Step 1 walkthrough
- URIs are parsed from a toml file to a data structure, includes a deny list for certain paths/keywords
- Once they're parsed, it's time to begin a-scraping.
    - Use an HTML parser to PURELY find links in every page, MUST lead to another page from the same URI. Ignores links with a path on the blacklist
    - Use this to build a list of links to parse/index

# Step 2 walkthrough
- Use some form of parsing to turn a raw html webpage into search terms
    - Title, date, keywords, etc.
- Once we have a defineable set of keys that can be turned into a JSON schema, we insert it into the Meillesearch document
