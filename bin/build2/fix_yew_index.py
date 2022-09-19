#!/usr/bin/env python3

template_file = "../../src/mxyz-server/static/dist/index.html.tera"
with open(template_file) as fp:
    lines = fp.readlines()

content = ''.join(lines)
content = content.replace('mxyz-client', 'static/dist/mxyz-client')

with open(template_file, 'w') as fp:
    fp.write(content)
