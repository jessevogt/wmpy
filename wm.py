#!/usr/bin/env python3

from typing import NamedTuple
import subprocess
import sys


def except_hook(exctype, value, traceback):
    with open("/tmp/wmpy.log", "a") as f:
        f.write(str(exctype) + "\n")
        f.write(str(value) + "\n")
        f.write(str(traceback) + "\n")
        sys.__excepthook__(exctype, value, traceback)


sys.excepthook = except_hook


class Screen(NamedTuple):
    height: int
    width: int
    name: str


screens = (
    Screen(
        height=1680,
        width=1050,
        name="left",
    ),
    Screen(
        height=1413,  # screen has menubar
        width=2560,
        name="middle",
    ),
    Screen(
        height=1920,
        width=1200,
    ),
)


def main():
    res = subprocess.check_call(["xdotool", "getactivewindow", "getwindowgeometry", "--shell"])
    with open()


if __name__ == "__main__":
    main()
