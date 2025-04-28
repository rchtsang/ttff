import os
from copy import copy
from pathlib import Path
from collections import namedtuple

import jinja2
from jinja2 import Environment, FileSystemLoader

from .models import AccessType
from .utils import *

PARENT_DIR = Path(__file__).resolve().parent
TEMPLATES_DIR = PARENT_DIR / "templates"

RegType = namedtuple('RegType', [
    'name',
    'description',
    'struct',
    'offset',
    'perms',
    'reset',
    'dim',
    'dim_increment',
    'size',
    'register',
])

ClusterType = namedtuple('ClusterType', [
    'name',
    'description',
    'struct',
    'offset',
    'perms',
    'reset',
    'dim',
    'dim_increment',
    'size',
    'cluster',
])

FieldType = namedtuple('FieldType', [
    'name',
    'description',
    'width',
    'type',
    'field',
])


def _byte_size(peripheral_group: dict):
    return int(peripheral_group['addressBlock']['size'], 0)

def _backing_size(peripheral_group: dict):
    return int(peripheral_group['addressBlock']['size'], 0) // 4

def _reg_type(reg_holder: dict):
    if isinstance(reg_holder, ClusterType):
        reg_holder = reg_holder.cluster
    assert reg_holder['#tag'] in ["peripheral", "cluster"], \
        "must be a peripheral or cluster"
    return f"{reg_holder['name'].replace('[%s]', '')}RegType"

def _reg_types(reg_holder: dict | ClusterType):
    if isinstance(reg_holder, ClusterType):
        reg_holder = reg_holder.cluster
    assert reg_holder['#tag'] in ["peripheral", "cluster"], \
        "must be a peripheral or cluster"
    reg_types = []
    for reg in reg_holder['registers']:
        if reg['#tag'] == 'register':
            reg_types.append(RegType(
                name=reg['name'].replace('[%s]', ''),
                description=reg['description'],
                struct=reg['name'].replace('[%s]', ''),
                offset=int(reg['addressOffset'], 0),
                perms=f"0b{AccessType.as_bits(reg['access']):03b}",
                reset=f"Some({reg['resetValue']})" if 'resetValue' in reg else "None",
                dim=int(reg['dim'], 0) if 'dim' in reg else None,
                dim_increment=int(reg['dimIncrement'], 0) if 'dimIncrement' in reg else None,
                size=4,
                register=reg,
            ))
    return reg_types

def _cluster_types(reg_holder: dict | ClusterType):
    if isinstance(reg_holder, ClusterType):
        reg_holder = reg_holder.cluster
    assert reg_holder['#tag'] in ["peripheral", "cluster"], \
        "must be a peripheral or cluster"
    cluster_types = []
    for reg in reg_holder['registers']:
        if reg['#tag'] == 'cluster':
            cluster_types.append(ClusterType(
                name=reg['name'].replace('[%s]', ''),
                description=reg['description'],
                struct=reg['name'].replace('[%s]', ''),
                offset=int(reg['addressOffset'], 0),
                perms=f"0b{AccessType.as_bits(reg['access']):03b}",
                reset=f"Some({reg['resetValue']})" if 'resetValue' in reg else "None",
                dim=int(reg['dim'], 0) if 'dim' in reg else None,
                dim_increment=int(reg['dimIncrement'], 0) if 'dimIncrement' in reg else None,
                size=_cluster_size(reg),
                cluster=reg,
            ))
    return cluster_types

def _cluster_mod(cluster_type: ClusterType | dict):
    if isinstance(cluster_type, ClusterType):
        return cluster_type.name.lower()
    elif isinstance(cluster_type, dict):
        assert cluster_type['#tag'] == 'cluster', "must be cluster"
        return cluster_type['name'].replace('[%s]', '').lower()
    assert False, "must be ClusterType or dict"

def _cluster_size(cluster: dict):
    assert cluster['#tag'] == 'cluster', "must be a cluster"
    cluster_offset = int(cluster['addressOffset'], 0)
    end_offsets = []
    for reg in cluster['registers']:
        # address offset is relative to the start of the cluster
        address_offset = int(reg['addressOffset'], 0)
        end_offset = address_offset + 4
        
        if "dim" in reg:
            dim = int(reg['dim'], 0)
            incr = int(reg["dimIncrement"], 0)
            end_offset += dim * incr

        end_offsets.append(end_offset)

    cluster_size = max(end_offsets)
    return cluster_size

def _fields(reg: dict | RegType):
    if isinstance(reg, RegType):
        reg = register.register
    assert reg['#tag'] == 'register', "must be a register"
    fields = []
    width = 0

    if 'fields' not in reg:
        reg['fields'] = []

    for field in reg['fields']:
        bitoffset = field['bitOffset']
        bitwidth = field['bitWidth']

        if bitoffset > width:
            fields.append(FieldType(
                name="__",
                description="",
                width=bitoffset - width,
                type="u32",
                field={},
            ))
            width = bitoffset

        width += bitwidth

        fields.append(FieldType(
            name=sanitize_name(field['name'].lower()),
            description=field['description'],
            width=bitwidth,
            type=size_to_type(bitwidth),
            field=field,
        ))

    if width < 32:
        fields.append(FieldType(
            name="__",
            description="",
            width=32 - width,
            type="u32",
            field={},
        ))

    return fields

helpers = {
    "_byte_size": _byte_size,
    "_backing_size": _backing_size,
    "_reg_type": _reg_type,
    "_reg_types": _reg_types,
    "_cluster_types": _cluster_types,
    "_cluster_mod": _cluster_mod,
    "_cluster_size": _cluster_size,
    "_fields": _fields,
    "int": int,
    "str": str,
    "range": range,
    "hex": hex,
}


def generate_device_mod(
    dst: Path,
    device: dict,
    peripheral_mods: list[str],
):
    env = Environment(
        block_start_string='/*%',
        block_end_string='%*/',
        variable_start_string='/*{',
        variable_end_string='}*/',
        comment_start_string='/*#',
        comment_end_string='#*/',
        loader=FileSystemLoader(str(TEMPLATES_DIR),
            encoding='utf-8',
            followlinks=False)
    )
    env.globals.update(helpers)

    device_template = env.get_template("device.rs")

    for i in range(len(peripheral_mods)):
        peripheral_mods[i] = peripheral_mods[i].lower()

    # generate device module
    with open(dst / 'mod.rs', 'w') as f:
        f.write(device_template.render(
            device_name=device['name'],
            peripheral_mods=peripheral_mods,
        ))

    # generate peripheral modules
    for p in peripheral_mods:
        if p not in device['#peripheral_groups']:
            print(f"peripheral not found: {p.lower()}")
            continue
        print(f"generating {p.lower()} module...")
        generate_peripheral_mod(env, dst, device['#peripheral_groups'][p])
    
    return

def generate_peripheral_mod(
    env: Environment,
    dst: Path,
    peripheral_group: dict,
):
    peripheral_template = env.get_template(str("peripheral/mod.rs"))
    registers_template = env.get_template(str("peripheral/registers.rs"))
    cluster_template = env.get_template(str("peripheral/cluster.rs"))

    
    # generate peripheral mod template
    file_dst = (dst / peripheral_group['name'].lower() / 'mod.rs')
    file_dst.parent.mkdir(exist_ok=True, parents=True)
    with open(file_dst, 'w') as f:
        f.write(peripheral_template.render(
            peripheral_group=peripheral_group))

    # generate registers
    file_dst = (dst
        / peripheral_group['name'].lower()
        / 'registers'
        / 'mod.rs')
    file_dst.parent.mkdir(exist_ok=True, parents=True)
    with open(file_dst, 'w') as f:
        f.write(registers_template.render(
            peripheral_group=peripheral_group))

    # find all clusters and subclusters
    maybe_regs = copy(peripheral_group['registers'])
    clusters = []
    while maybe_regs:
        maybe_reg = maybe_regs.pop(0)
        if maybe_reg['#tag'] != 'cluster':
            continue

        # clusters won't have circular references so this is ok
        clusters.append(maybe_reg)
        maybe_regs.extend(maybe_reg['registers'])

    # generate all cluster modules
    for cluster in clusters:
        file_dst = (dst 
            / peripheral_group['name'].lower()
            / 'registers'
            / f"{_cluster_mod(cluster)}.rs")
        file_dst.parent.mkdir(exist_ok=True, parents=True)
        with open(file_dst, 'w') as f:
            f.write(cluster_template.render(cluster=cluster))

    return
