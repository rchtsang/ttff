import os
from pathlib import Path
from copy import copy

from parse import *
from template import *

def gen_peripheral_mod(dst: Path, peripheral: str, device: dict):
    peripheral = device['#peripheral_groups'][peripheral]

    if "@derivedFrom" in peripheral:
        src = peripheral['@derivedFrom'].lower()
        p = copy(device['peripherals'][src])
        p.update(peripheral)
        peripheral = p

    peripheral_content = gen_peripheral_mod_content(peripheral)
    registers_content = gen_registers_content(peripheral)

    peripheral_filename = peripheral_content['fields']['peripheral_filename']
    dst = dst / peripheral_filename
    (dst / "registers").mkdir(exist_ok=True, parents=True)

    elements = copy(peripheral["registers"])
    registers = []
    clusters = []
    while elements:
        element = elements.pop()
        match element['#tag']:
            case 'register':
                registers.append(element)
            case 'cluster':
                clusters.append(element)
            case _:
                raise ValueError(f"invalid register tag: {element['#tag']}")

    cluster_content = []

    for cluster in clusters:
        cluster_content.append(gen_cluster_content(cluster))

    peripheral_mod_template_path = TEMPLATES_DIR / "peripheral" / "mod.rs"
    registers_template_path = TEMPLATES_DIR / "peripheral" / "registers.rs"
    cluster_template_path = TEMPLATES_DIR / "peripheral" / "cluster.rs"

    generated_peripheral_mod_content = gen_from_template(
        peripheral_mod_template_path, **peripheral_content)
    with open(dst / "mod.rs", 'w') as f:
        f.write(generated_peripheral_mod_content)

    generated_registers_content = gen_from_template(
        registers_template_path, **registers_content)
    with open(dst / "registers" / "mod.rs", 'w') as f:
        f.write(generated_registers_content)

    for kwargs in cluster_content:
        cluster_filename = kwargs['fields']['cluster_filename']
        generated_content = gen_from_template(cluster_template_path, **kwargs)
        with open(dst / "registers" / f"{cluster_filename}", 'w') as f:
            f.write(generated_content)


if __name__ == "__main__":
    SVD_DIR = (Path.home() 
        / "Documents" 
        / "Research" 
        / "reference" 
        / "cmsis-packs"
        / "NordicSemiconductor.nRF_DeviceFamilyPack.8.59.0"
        / "SVD"
    )

    # use this script as a rust template generator for an svd file
    import argparse

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