import os
import json
import argparse
from pathlib import Path
from copy import copy

from .parse import *
from .generate import *

SVD_DIR = (Path.home() 
    / "Documents" 
    / "Research" 
    / "reference" 
    / "cmsis-packs"
    / "NordicSemiconductor.nRF_DeviceFamilyPack.8.59.0"
    / "SVD"
)

# use this script as a rust template generator for an svd file

parser = argparse.ArgumentParser()
parser.add_argument('svd_file', type=Path)
parser.add_argument('peripherals', nargs='*', type=str)
parser.add_argument('--all', action='store_true')
parser.add_argument('--abs', action='store_true')
parser.add_argument('--dst', type=Path, default=Path("."))
parser.add_argument('--print-peripherals', action='store_true')
parser.add_argument('--to-json', action='store_true')

args = parser.parse_args()

path = args.svd_file if args.abs else SVD_DIR / args.svd_file

args.peripherals = [p.lower() for p in args.peripherals]

assert path.exists(), \
    "file does not exist: {}".format(str(path))

device = load_svd(path)

if args.to_json:
    with open(Path('.') / f"{path.name}.json", 'w') as f:
        json.dump(device, f, indent=2)
    exit(0)

if args.print_peripherals:
    print(f"{device['name']} peripherals:")
    for peripheral in sorted(device['#peripheral_groups']):
        print(peripheral)
    exit(0)

if args.all:
    args.peripherals = [p for p in device['#peripheral_groups']]

args.dst.mkdir(exist_ok=True, parents=True)

peripheral_mods = [p.lower() for p in args.peripherals]
generate_device_mod(args.dst, device, peripheral_mods)
