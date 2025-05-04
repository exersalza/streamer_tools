import sys
import os
import re
import operator
import random

FILTER = (".rs", ".ts", ".js", ".tsx", ".jsx", ".py", ".html", ".css")
BLACKLIST = ["venv", "target", "dist", "node_modules", ".git"]

COMMENT_STRING_TRANS = {
    "tsx": "rs",
    "ts": "rs",
    "js": "rs",
    "jsx": "rs",
    "c": "rs",
    "cpp": "rs",
}

COMMENT_STRINGS = {
    "py": [
        "#",
    ],
    "rs": [
        "//",
        # /// will also get trigered when we test for //
        "/*", # same thing here, this will also trigger for /**
        "*",
    ],
    "html": ["<!--", "-->"],
    "css": [
        "#"
    ]
}

loc = {}
found_comments = []

def form_blacklist() -> str:
    ret = ".*("

    for i in BLACKLIST:
        ret += f"({i})|"

    return ret[: len(ret) - 1] + ").*"


def count_comments(file_type: str, lines: list[str]) -> int:
    found = 0
    comments = {}
    if file_type in COMMENT_STRINGS.keys():
        comments = COMMENT_STRINGS[file_type]
    else:
        comments = COMMENT_STRINGS[COMMENT_STRING_TRANS[file_type]]

    for i in lines:
        for j in comments:
            found += i.strip().startswith(j)

    return found

def count_types(file_type: str, lines: list[str]):
    pass


def count_lines(path: str) -> int:
    lines = 0

    with open(path, "r") as f:
        file_type = path.split(".")[-1]
        f_lines = f.readlines()

        found_comments.append(count_comments(file_type, f_lines))
        count_types(file_type, f_lines)

        # filter out empty lines
        for i in f_lines: 
            if len(i.strip()) != 0:
                lines += 1

    loc[path] = lines
    return lines


def find_biggest() -> list[tuple[ str, int ]]:
    sorted_dict = dict(sorted(loc.items(), key=operator.itemgetter(1)))
    ret: list[ tuple[ str, int ] ] = [(i[0], i[1]) for i in sorted_dict.items()]

    return [ret[-1], ret[-2], ret[-3]]


def main() -> int:
    bl_re = form_blacklist()

    for i in os.walk("."):
        path = i[0]

        if re.match(bl_re, path):
            continue

        files = i[2]
        for j in files:
            if not j.endswith(FILTER) or "config" in j:
                continue
            count_lines(f"{path}\\{j}")

    count = 0

    for _, value in loc.items():
        count += value

    print(f"{count} Lines of code")
    print(f"{sum(found_comments)} Comments\n")
    print("Top 3 Storage hogs: ")

    for i in find_biggest():
        print(f"{i[0]} with {i[1]} lines and a size of {os.path.getsize(i[0])/1000:.2f}KB")

    

    if random.randint(0,10) == 9:
        # NOTE: very important
        print(chr(sum(range(ord(min(str(not())))))))

    return 0


if __name__ == "__main__":
    sys.exit(main())
