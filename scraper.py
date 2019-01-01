from lxml import html
import json

emoji_map = {}
all_emojis = list()
with open ('people-and-smileys.html', 'r') as emoji_page:
    page = emoji_page.read().replace('\n', '')
    tree = html.fromstring(page)
    emojis = tree.xpath('//ul[@class="emoji-list"]//a[//span[@class="emoji"]]')

    for element in emojis:
        emoji = element[0].text
        text = element[0].tail[1:]
        emoji_map[text] = emoji;
        all_emojis.append(emoji);
tags_json = json.dumps(emoji_map, ensure_ascii=False)
category_json = json.dumps(all_emojis, ensure_ascii=False)

with open('tags.json', 'w') as tags_file:
    tags_file.write(tags_json)

with open('category.json', 'w') as category_file:
    category_file.write(category_json)
