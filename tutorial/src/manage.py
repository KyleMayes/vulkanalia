#!/usr/bin/env python3

import argparse
import os
import sys
import subprocess

sources_dir = os.path.dirname(os.path.realpath(__file__))
sources = [p for p in os.listdir(sources_dir) if p.endswith(".rs")]
sources_by_prefix = dict([(int(s.split("_")[0]), s) for s in sources])

#=================================================
# Compare
#=================================================

def compare(args):
    for prefix in range(0, max(sources_by_prefix.keys())):
        this = sources_by_prefix[prefix]
        that = sources_by_prefix[prefix + 1]
        output = f"diff_{prefix}_{prefix + 1}.diff"
        command = ["git", "diff", "--no-index", f"--output={output}", this, that]
        try: subprocess.check_output(command, cwd=sources_dir)
        except: pass

#=================================================
# Patch
#=================================================

def patch(args):
    start = sources_by_prefix[args.start]
    patch = subprocess.check_output(["git", "diff", start], cwd=sources_dir)
    for prefix in range(args.start + 1, args.end + 1):
        target = sources_by_prefix[prefix]
        apply_patch(patch, target)

def apply_patch(patch, target):
    patcher = subprocess.Popen(["patch", "-f", target], cwd=sources_dir, stdin=subprocess.PIPE)
    patcher.communicate(patch)
    code = patcher.wait()

    try: os.remove(f"{target}.orig")
    except: pass
    try: os.remove(f"{target}.rej")
    except: pass
    
    if code != 0:
        sys.exit(code)

#=================================================
# Arguments
#=================================================

parser = argparse.ArgumentParser(
    prog="manage",
    description="Manages tutorial sources.",
)

subparsers = parser.add_subparsers(
    help="command",
)

#- Compare -----------------------------------------

compare_parser = subparsers.add_parser(
    "compare",
    help="Generates consecutive diffs for tutorial sources.",
)

compare_parser.set_defaults(command=compare)

#- Patch -----------------------------------------

patch_parser = subparsers.add_parser(
    "patch",
    help="Applies a change to a sequence of tutorial sources.",
    epilog="""
USAGE: The Git patch for the unstaged changes in the starting tutorial source
will be applied to the other tutorial sources in the specified sequence.
    """,
)

patch_parser.add_argument(
    "start",
    help="The number of the starting tutorial source.",
    type=int,
)

patch_parser.add_argument(
    "end",
    help="The number of the ending tutorial source (inclusive).",
    type=int,
)

patch_parser.set_defaults(command=patch)

#- Command ---------------------------------------

args = parser.parse_args()
args.command(args)
