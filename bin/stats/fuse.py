#!/usr/bin/env python3

import os
import pathlib


pwd = pathlib.Path(__file__).parent.resolve()
proj_root = f"{pwd}/../.."


def check_dir(path, code):

    files = os.listdir(path)
    for item in sorted(files):

        if item in ['out', 'target', '.git', 'pkg', 'Cargo.lock', 'fuse.txt', '.DS_Store']:
            continue
        if item.endswith('.png') or item.endswith('.jpg') or item.endswith('.ico'):
            continue
        print(item)

        new_path = os.path.join(path, item)
        if os.path.isdir(new_path):
            code = check_dir(new_path, code)

        else:
            with open(new_path, 'r') as fp:
                code += ''.join(fp.readlines())

    return code


code = check_dir(proj_root, "")
# print(f"\nnr. of lines: {code}")
path_to_fuse = os.path.join(pwd, 'fuse.txt')
with open(path_to_fuse, 'w') as fp:
    fp.write(code)

