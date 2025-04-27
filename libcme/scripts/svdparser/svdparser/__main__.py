import os
import argparse
from pathlib import Path
from copy import copy

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

args = parser.parse_args()

path = args.svd_file if args.abs else SVD_DIR / args.svd_file

args.peripherals = [p.lower() for p in args.peripherals]

assert path.exists(), \
    "file does not exist: {}".format(str(path))

device = load_svd(path)

if args.print_peripherals:
    print(f"{device['name']} peripherals:")
    for peripheral in sorted(device['#peripheral_groups']):
        print(peripheral)
    exit(0)

if args.all:
    args.peripherals = [p for p in device['#peripheral_groups']]

args.dst.mkdir(exist_ok=True, parents=True)

peripheral_mods = []
for p in args.peripherals:
    if p.lower() not in device['#peripheral_groups']:
        print(f"peripheral not found: {p.lower()}")
        continue
    print(f"generating {p.lower()} module...")
    gen_peripheral_mod(args.dst, p.lower(), device)
    peripheral_mods.append({ "peripheral_mod": p.lower() })

generated_peripheral_mods = gen_from_template(
    TEMPLATES_DIR / "device.rs",
    fields={
        "device_name_lower": device['name'].lower(),
        "device_name": device['name'],
    },
    subtemplate_fields={
        "peripheral_mods": peripheral_mods,
    })

with open(args.dst / "mod.rs", 'w') as f:
    f.write(generated_peripheral_mods)