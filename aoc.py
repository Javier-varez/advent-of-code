#!/usr/bin/env python

import argparse
import sys
import os
import pathlib
import requests

from datetime import date

parser = argparse.ArgumentParser('aoc', description='helper for advent-of-code')

templates = {
    'nix': ('default.nix', '''#!/usr/bin/env nix-instantiate --strict --eval
let
  pkgs = import <nixpkgs> { };
  inputTxt = lib.readFile ./input.txt;
in
{
  inherit inputTxt;
}
'''),
    'python': ('solution.py', '''#!/usr/bin/env python

import sys

if len(sys.argv) <= 1:
    print('Using default input file')
    fileName = 'input.txt'
else:
    fileName = sys.argv[1]

with open(fileName) as f:
    print(f.readlines())
'''),
    'zig': ('main.zig', '''
const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const err = gpa.deinit();
        if (err == .leak) {
            std.log.err("Memory leak!", .{});
        }
    }
    const allocator = gpa.allocator();
    const data = try std.fs.cwd().readFileAlloc(allocator, "input.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    std.debug.print("data: {s}", .{data});
}
'''),
}

def cur_date():
    cur_date = date.today()
    day = cur_date.day
    month = cur_date.month
    year = cur_date.year
    return (str(year), str(day))

def init_project_folder(args):
    folderPath = pathlib.Path(args.year).joinpath('day'+args.day)
    if folderPath.exists():
        print(f'Solution for {folderPath} was already initialized')
        return

    if not args.language in templates:
        print(f'Unknown language {args.language}')
        return

    if not 'AOC_SESSION' in os.environ:
        print(f'Please setup your environment with the AOC_SESSION var pointing to your session cookie')
        return

    os.makedirs(folderPath)

    session = os.environ['AOC_SESSION']
    input = requests.get(f'https://adventofcode.com/{args.year}/day/{args.day}/input', cookies = {"session": session})
    with open(folderPath.joinpath('input.txt'), 'wb') as f:
        f.write(input.content)

    fileName, content = templates[args.language]
    with open(folderPath.joinpath(fileName), 'w') as f:
        f.write(content)

subparsers = parser.add_subparsers()

new_command = subparsers.add_parser('new')

default_year, default_day = cur_date()
new_command.add_argument('--day', required=False, default=default_day)
new_command.add_argument('--year', required=False, default=default_year)
new_command.add_argument('-l', '--language', required=False, default='zig')
new_command.set_defaults(handler = init_project_folder)

args = parser.parse_args()

if not 'handler' in args:
    parser.print_usage()
    sys.exit(1)

args.handler(args)
