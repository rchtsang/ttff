import os
from pathlib import Path
from copy import copy

from .parse import *
from .template import *

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
