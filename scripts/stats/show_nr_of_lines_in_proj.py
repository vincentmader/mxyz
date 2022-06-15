#!/usr/bin/env python3

import os
import pathlib


# pwd = pathlib.Path(__file__).parent.resolve()
# proj_root = f"{pwd}/../.."
PATH_TO_PROJECT = pathlib.Path(__file__).parent.resolve()
proj_root = f"{PATH_TO_PROJECT}/../.."


def check_dir(path, length):

    files = os.listdir(path)
    for item in sorted(files):

        if item in ['out', 'target', '.git', 'pkg', 'Cargo.lock']:
            continue
        if item.endswith('.png') or item.endswith('.jpg') or item.endswith('.ico'):
            continue
        print(item)

        new_path = os.path.join(path, item)
        if os.path.isdir(new_path):
            length = check_dir(new_path, length)

        else:
            with open(new_path, 'rb') as fp:
                content = fp.readlines()
                length += len(content)

    return length


length = check_dir(proj_root, 0)
print(f"\nnr. of lines: {length}")

