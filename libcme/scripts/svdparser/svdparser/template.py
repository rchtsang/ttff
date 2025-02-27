import os
import re
from copy import copy
from pathlib import Path

from models import AccessType
from utils import *

PARENT_DIR = Path(__file__).resolve().parent
TEMPLATES_DIR = PARENT_DIR / "templates"

TEMPLATE_FIELD_PTRN = re.compile(r"\/\*\% (?P<template_field>.*?) \%\*\/")
SUBTEMPLATE_PTRN = re.compile(r"\/\*! (?P<template_field>.*?) --->\n(?P<subtemplate>(?:\s|.)*?)^ *!\*\/", flags=re.M)
SUBTEMPLATE_FIELD_PTRN = re.compile(r"%(?P<subtemplate_field>.*?)%")
SUBTEMPLATE_LINK_PTRN = re.compile(r"#(?P<subtemplate_link>.*?)#")


def gen_field_ptrn(field_name):
    return re.compile(r"\/\*\% {} \%\*\/".format(field_name))

def gen_subtemplate_ptrn(subtemplate_name):
    return re.compile(r"^ *?\/\*! {} --->\n.*?!\*\/\n?".format(subtemplate_name), flags=re.S | re.M)

def gen_subfield_ptrn(subfield_name):
    return re.compile(r"%{}%".format(subfield_name))

def gen_subfield(subfield_name, value):
    if ':' in subfield_name:
        _, fmt = subfield_name.split(':', 1)
        try:
            if any([c in value for c in "dbox"]):
                value = int(value, 0)
            return (f"{{:{fmt}}}").format(value)
        except ValueError as e:
            print(f"warning: {subfield_name} -> {e}")
            pass
    return (f"{value}")

def get_field_name(field):
    if ':' in field:
        name, _ = field.split(':', 1)
        return name
    return field

def get_cluster_size(cluster: dict):
    cluster_size = 4
    for reg in cluster['registers']:
        address_offset = int(reg['addressOffset'], 0)
        end_offset = address_offset + 4
        
        if "dim" in reg:
            dim = int(reg['dim'], 0)
            incr = int(reg["dimIncrement"], 0)
            end_offset += dim * incr
        if end_offset > cluster_size:
            cluster_size = end_offset

    return cluster_size

def gen_from_template(src: Path, fields: dict, subtemplate_fields: dict):
    """
    fields are expected to be a shallow dictionary.
    subtemplate fields is a dictionary of lists of dictionaries 
    containing the subfield variables to be replaced
    """
    assert src.exists(), "file not found: {}".format(str(src))
    with open(src, 'r') as f:
        template = f.read()

    template_fields = list(sorted(set(TEMPLATE_FIELD_PTRN.findall(template))))
    field_ptrns = [gen_field_ptrn(field) for field in template_fields]

    missing_fields = [field for field in template_fields if field not in fields]
    assert not missing_fields, \
        "missing fields:\n\t{}".format('\n\t'.join(missing_fields))

    for field, ptrn in zip(template_fields, field_ptrns):
        # if not isinstance(fields[field], str):
        #     print(field, fields[field])
        template = ptrn.sub(fields[field], template)

    subtemplates = list(sorted(set(SUBTEMPLATE_PTRN.findall(template))))
    for name, subtemplate in subtemplates:
        subtemplate_ptrn = gen_subtemplate_ptrn(name)
        needed_subfields = list(sorted(set(SUBTEMPLATE_FIELD_PTRN.findall(subtemplate))))
        links = list(sorted(set(SUBTEMPLATE_LINK_PTRN.findall(subtemplate))))

        ptrns = [gen_subfield_ptrn(field) for field in needed_subfields]
        contents = []
        for subfields in subtemplate_fields[name]:
            content = copy(subtemplate)
            for field, ptrn in zip(needed_subfields, ptrns):
                field_name = get_field_name(field)
                if field_name in subfields:
                    value = subfields[field_name]
                elif field_name in fields:
                    value = fields[field_name]
                else:
                    continue
                # print(field, value)
                value = gen_subfield(field, value)
                content = ptrn.sub(value.lstrip(), content)

            for link in links:
                assert (src.parent / link).exists(), \
                    "file not found: {}".format(str(src.parent / link))
                value = gen_from_template(
                    src.parent / link,
                    **subfields)
                content = re.sub(r"#{}#".format(link), value.lstrip(), content)

            contents.append(content)
        template = subtemplate_ptrn.sub(''.join(contents), template)

    return template

def gen_cluster_content(cluster: dict):
    stripped_name = cluster['name'].replace('[%s]', '')
    cluster_filename = f"{stripped_name.lower()}.rs"
    cluster_name = stripped_name
    cluster_type_name = f"{stripped_name}RegType"

    # separate into registers and clusters
    elements = copy(cluster['registers'])
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
    
    registers.sort(key=lambda r: int(r['addressOffset'], 0))
    clusters.sort(key=lambda r: int(r['addressOffset'], 0))

    assert len(clusters) == 0, "there are subclusters!"

    cluster_size = 4

    data_params = "&self"
    data_call = ""

    if 'dim' in cluster:
        assert '[%s]' in cluster['name']
        cluster_size = int(cluster['dimIncrement'], 0)
        data_params = "&self, n: u8"
        data_call = "n"

    cluster_type_variants = []
    reg_type_offset_match_arms = []
    reg_type_info_match_arms = [] 
    register_struct_defs = []
    cluster_array_info_match_arms = []
    reg_type_enumeration = []
    cluster_type_enumeration = []

    for reg in registers:
        assert reg['#tag'] != 'cluster', \
            "cluster in a cluster! {}".format(reg['name'])
        address_offset = int(reg['addressOffset'], 0)
        end_offset = address_offset + 4
        
        if "dim" in reg:
            dim = int(reg['dim'], 0)
            incr = int(reg["dimIncrement"], 0)
            end_offset += dim * incr

            cluster_type_variants.append({
                "cluster_type_variant": reg['name'].upper().replace('[%S]', '(u8)'),
                "cluster_type_variant_desc": reg['description'],
            })

            reg_type_offset_match_arms.append({
                "cluster_type_variant": reg['name'].upper().replace(
                    '[%S]', f'(((offset - {address_offset:#05x}) / {incr}) as u8)'),
                "match_ptrn": f"{address_offset:#05x}..={address_offset+dim*incr-1:#05x}",
            })

            if 'dim' in cluster:
                cluster_dim = int(cluster['dim'], 0)
                cluster_incr = int(cluster['dimIncrement'], 0)

                for i in range(cluster_dim):
                    cluster_offset = address_offset + (i * cluster_incr)

                    for j in range(dim):
                        byte_offset = cluster_offset + j * incr

                        cluster_array_info_match_arms.append({
                            "cluster_type_variant": reg['name'].upper().replace('[%S]', f'({j})'),
                            "match_cond": f"n == {i}",
                            "byte_offset": f"{byte_offset:#05x}",
                            "perms": f"0b{AccessType.as_bits(reg['access']):03b}",
                            "reset_value": f"Some({reg['resetValue']})" if 'resetValue' in reg else "None",
                        })

            else:

                for j in range(dim):
                    byte_offset = address_offset + j * incr
                    reg_type_info_match_arms.append({
                        "byte_offset": f"{byte_offset:#05x}",
                        "cluster_type_variant": reg['name'].upper().replace('[%S]', f'({j})'),
                        "perms": f"0b{AccessType.as_bits(reg['access']):03b}",
                        "reset_value": f"Some({reg['resetValue']})" if 'resetValue' in reg else "None",
                    })
                    reg_type_enumeration.append({
                        "reg_type_variant": reg['name'].upper().replace('[%S]', f"({j})"),
                    })

            register_struct_defs.append(gen_register_content(reg))
        else:
            cluster_type_variant = reg['name'].upper()

            cluster_type_variants.append({
                "cluster_type_variant": cluster_type_variant,
                "cluster_type_variant_desc": reg['description'],
            })

            reg_type_offset_match_arms.append({
                "cluster_type_variant": cluster_type_variant,
                "match_ptrn": reg['addressOffset'],
            })

            if 'dim' in cluster:
                cluster_dim = int(cluster['dim'], 0)
                cluster_incr = int(cluster['dimIncrement'], 0)

                for i in range(cluster_dim):
                    cluster_offset = address_offset + (i * cluster_incr)

                    cluster_array_info_match_arms.append({
                        "cluster_type_variant": cluster_type_variant,
                        "match_cond": f"n == {i}",
                        "byte_offset": f"{cluster_offset:#05x}",
                        "perms": f"0b{AccessType.as_bits(reg['access']):03b}",
                        "reset_value": f"Some({reg['resetValue']})" if 'resetValue' in reg else "None",
                    })

            else:
                reg_type_info_match_arms.append({
                    "byte_offset": reg['addressOffset'],
                    "cluster_type_variant": cluster_type_variant,
                    "perms": f"0b{AccessType.as_bits(reg['access']):03b}",
                    "reset_value": f"Some({reg['resetValue']})" if 'resetValue' in reg else "None",
                })
                reg_type_enumeration.append({
                    "reg_type_variant": cluster_type_variant
                })

            register_struct_defs.append(gen_register_content(reg))

        if end_offset > cluster_size:
            cluster_size = end_offset

    cluster_array_info_match_arms.sort(key=lambda d: d['match_cond'])

    return {
        "fields": {
            "cluster_filename": cluster_filename,
            "cluster_name": cluster_name,
            "cluster_size": f"{cluster_size:#06x}",
            "cluster_type_name": cluster_type_name,
            "data_params": data_params,
            "data_call": data_call,
        },
        "subtemplate_fields": {
            "cluster_type_variants": cluster_type_variants,
            "reg_type_offset_match_arms": reg_type_offset_match_arms,
            "reg_type_info_match_arms": reg_type_info_match_arms,
            "cluster_array_info_match_arms": cluster_array_info_match_arms,
            "register_struct_defs": register_struct_defs,
            "reg_type_enumeration": reg_type_enumeration,
            "cluster_type_enumeration": cluster_type_enumeration,
        },
    }

def gen_register_content(reg: dict):
    register_fields = []
    fields = reg['fields'] if 'fields' in reg else []
    fields.sort(key=lambda f: f['bitOffset'])
    width = 0    
    for field in fields:
        bitoffset = field['bitOffset']
        bitwidth = field['bitWidth']

        if bitoffset > width:
            register_fields.append({
                "field_description": "",
                "field_width": f"{bitoffset - width:d}",
                "field_name": "__",
                "field_type": "u32",
            })
            width = bitoffset

        width += bitwidth

        register_fields.append({
            "field_description": field['description'],
            "field_width": f"{bitwidth}",
            "field_name": sanitize_name(field['name'].lower()),
            "field_type": size_to_type(bitwidth),
        })

    if width < 32:
        register_fields.append({
            "field_description": "",
            "field_width": f"{32 - width}",
            "field_name": "__",
            "field_type": "u32",
        })

    return {
        'fields': {
            "register_description": reg['description'],
            "register_name": reg['name'].replace('[%s]', ''),
            "register_name_upper": reg['name'].replace('[%s]', '').upper(),
            "register_size": "u32", # this may be a field, but i'm lazy rn
        },
        'subtemplate_fields': {
            "register_fields": register_fields,
        },
    }

def gen_registers_content(peripheral: dict):
    peripheral_name = peripheral['name']
    peripheral_size = peripheral['addressBlock']['size']
    reg_type_name = f"{peripheral['name']}RegType"

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

    registers.sort(key=lambda r: int(r['addressOffset'], 0))
    clusters.sort(key=lambda r: int(r['addressOffset'], 0))

    cluster_mods = []
    reg_type_variants = []
    reg_type_info_match_arms = []
    reg_type_offset_match_arms = []
    register_struct_defs = []
    reg_type_enumeration = []

    for reg in registers:
        address_offset = int(reg['addressOffset'], 0)
        if "dim" in reg:
            dim = int(reg['dim'], 0)
            incr = int(reg["dimIncrement"], 0)

            reg_type_variants.append({
                "reg_type_variant": reg['name'].upper().replace('[%S]', '(u8)'),
                "reg_type_variant_desc": reg['description'],
            })

            reg_type_offset_match_arms.append({
                "reg_type_variant": reg['name'].upper().replace(
                    '[%S]', f'(((offset - {address_offset:#05x}) / {incr}) as u8)'),
                "match_ptrn": f"{address_offset:#05x}..={address_offset+dim*incr-1:#05x}",
            })

            for i in range(dim):
                byte_offset = address_offset + i * incr
                reg_type_info_match_arms.append({
                    "byte_offset": f"{byte_offset:#05x}",
                    "reg_type_variant": reg['name'].upper().replace('[%S]', f'({i})'),
                    "perms": f"0b{AccessType.as_bits(reg['access']):03b}",
                    "reset_value": f"Some({reg['resetValue']})" if 'resetValue' in reg else "None",
                })
                reg_type_enumeration.append({
                    "reg_type_variant": reg['name'].upper().replace('[%S]', f'({i})'),
                })

            register_struct_defs.append(gen_register_content(reg))

        else:
            reg_type_variants.append({
                "reg_type_variant": reg['name'].upper(),
                "reg_type_variant_desc": reg['description'],
            })

            reg_type_offset_match_arms.append({
                "reg_type_variant": reg['name'].upper(),
                "match_ptrn": reg['addressOffset'],
            })

            reg_type_info_match_arms.append({
                "byte_offset": reg['addressOffset'],
                "reg_type_variant": reg['name'].upper(),
                "perms": f"0b{AccessType.as_bits(reg['access']):03b}",
                "reset_value": f"Some({reg['resetValue']})" if 'resetValue' in reg else "None",
            })
            reg_type_enumeration.append({
                "reg_type_variant": reg['name'].upper(),
            })

            register_struct_defs.append(gen_register_content(reg))

    cluster_type_offset_match_arms = []
    cluster_type_info_match_arms = []
    cluster_type_enumeration = []

    for cluster in clusters:
        address_offset = int(cluster['addressOffset'], 0)
        cluster_size = get_cluster_size(cluster)

        if 'dim' in cluster:
            dim = int(cluster['dim'], 0)
            incr = int(cluster['dimIncrement'], 0)

            # TODO: finish accounting for cluster array

            stripped_name = cluster['name'].replace('[%s]', '')

            cluster_mods.append({
                "cluster_name": stripped_name.lower(),
            })

            reg_type_variants.append({
                "reg_type_variant": f"{stripped_name}(u8, {stripped_name}RegType)",
                "reg_type_variant_desc": cluster['description'],
            })

            for i in range(dim):
                offset = address_offset + (i * incr)

                cluster_type_offset_match_arms.append({
                    "cluster_type_offset": (
                        f"{stripped_name}RegType::lookup_offset(offset - {offset:#05x})"
                        f".map(|reg| {reg_type_name}::{stripped_name}(((offset - {offset:#05x}) / {incr:#04x}) as u8, reg))"
                    ),
                    "match_ptrn": f"{offset:#05x}..={offset+incr-1:#05x}",
                })
                cluster_type_enumeration.append({
                    "cluster_reg_type": f"{stripped_name}RegType",
                    "mapped_reg_type": f"{reg_type_name}::{stripped_name}({i}, reg_type)"
                })

            cluster_type_info_match_arms.append({
                "cluster_type_variant": f"{stripped_name}(n, reg)",
                "cluster_info": f"reg._data(n)",
            })

        else:
            cluster_mods.append({
                "cluster_name": cluster['name'].lower(),
            })

            reg_type_variants.append({
                "reg_type_variant": f"{cluster['name']}({cluster['name']}RegType)",
                "reg_type_variant_desc": cluster['description'],
            })

            cluster_type_offset_match_arms.append({
                "cluster_type_offset": (
                    f"{cluster['name']}RegType::lookup_offset(offset - {address_offset:#05x})"
                    f".map(|reg| {reg_type_name}::{cluster['name']}(reg))"
                ),
                "match_ptrn": f"{address_offset:#05x}..={address_offset+cluster_size-1:#05x}",
            })
            cluster_type_enumeration.append({
                "cluster_reg_type": f"{cluster['name']}RegType",
                "mapped_reg_type": f"{reg_type_name}::{cluster['name']}(reg_type)"
            })

            cluster_type_info_match_arms.append({
                "cluster_type_variant": f"{cluster['name']}(reg)",
                "cluster_info": f"reg._data()",
            })

    return {
        'fields': {
            "peripheral_name": peripheral_name,
            "peripheral_size": peripheral_size,
            "reg_type_name": reg_type_name,
        },
        'subtemplate_fields': {
            "cluster_mods": cluster_mods,
            "reg_type_variants": reg_type_variants,
            "reg_type_info_match_arms": reg_type_info_match_arms,
            "reg_type_offset_match_arms": reg_type_offset_match_arms,
            "register_struct_defs": register_struct_defs,
            "cluster_type_offset_match_arms": cluster_type_offset_match_arms,
            "cluster_type_info_match_arms": cluster_type_info_match_arms,
            "reg_type_enumeration": reg_type_enumeration,
            "cluster_type_enumeration": cluster_type_enumeration,
        }
    }

def gen_peripheral_mod_content(peripheral_group: dict):
    peripheral_filename = peripheral_group['name'].lower()
    peripheral_name = peripheral_group['name']
    byte_size = int(peripheral_group['addressBlock']['size'], 0)
    base_address = int(peripheral_group['baseAddress'], 0)
    backing_size = byte_size // 4
    reg_type = f"{peripheral_name}RegType"


    fields = {
        "peripheral_name": peripheral_name,
        "peripheral_filename": peripheral_filename,
        "peripheral_description": peripheral_group['description'],
        "default_base_address": hex(base_address),
        "backing_size": hex(backing_size),
        "byte_size": hex(byte_size),
        "reg_type": reg_type,
    }

    elements = copy(peripheral_group["registers"])
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

    modules = []
    cluster_type_variants_match_arms = []
    reg_type_variants_match_arms = []
    register_getters = []
    cluster_reg_getters = []

    for cluster in clusters:
        stripped_name = cluster['name'].replace('[%s]', '')
        cluster_reg_type = f"{stripped_name}RegType"

        if "dim" in cluster:
            cluster_type_variants_match_arms.append({
                "cluster_type_variant": cluster['name'].replace('[%s]', '(u8)'),
            })

            for reg in cluster['registers']:
                ref_params = "&self, n: u8"
                mut_params = "&mut self, n: u8"
                offset_call_params = "n"
                cluster_reg_type_variant = reg['name'].upper().replace('[%S]', '(i)')
                cluster_reg_type_variant_lower = reg['name'].replace('[%s]', '').lower()
                cluster_reg_type_variant_struct = reg['name'].upper().replace('[%S]', '')

                if 'dim' in reg:
                    ref_params = "&self, n: u8, i: u8"
                    mut_params = "&mut self, n: u8, i: u8"

                cluster_reg_getters.append({
                    "cluster_name_lower": stripped_name.lower(),
                    "cluster_reg_type_variant_lower": cluster_reg_type_variant_lower,
                    "offset_call_params": offset_call_params,
                    "ref_params": ref_params,
                    "mut_params": mut_params,
                    "cluster_mod": stripped_name.lower(),
                    "cluster_reg_type_variant_struct": cluster_reg_type_variant_struct,
                    "cluster_reg_type": cluster_reg_type,
                    "cluster_reg_type_variant": cluster_reg_type_variant,
                })
        else:
            cluster_type_variants_match_arms.append({
                "cluster_type_variant": cluster['name'],
            })

            for reg in cluster['registers']:
                ref_params = "&self"
                mut_params = "&mut self"
                offset_call_params = ""
                cluster_reg_type_variant = reg['name'].upper().replace('[%S]', '(i)')
                cluster_reg_type_variant_lower = reg['name'].replace('[%s]', '').lower()
                cluster_reg_type_variant_struct = reg['name'].upper().replace('[%S]', '')

                if 'dim' in reg:
                    ref_params = "&self, i: u8"
                    mut_params = "&mut self, i: u8"

                cluster_reg_getters.append({
                    "cluster_name_lower": stripped_name.lower(),
                    "cluster_reg_type_variant_lower": cluster_reg_type_variant_lower,
                    "offset_call_params": offset_call_params,
                    "ref_params": ref_params,
                    "mut_params": mut_params,
                    "cluster_mod": stripped_name.lower(),
                    "cluster_reg_type_variant_struct": cluster_reg_type_variant_struct,
                    "cluster_reg_type": cluster_reg_type,
                    "cluster_reg_type_variant": cluster_reg_type_variant,
                })

    for register in registers:
        if "dim" in register:
            stripped_name = register['name'].replace('[%s]', '')
            reg_type_variants_match_arms.append({
                "reg_type_variant": register['name'].upper().replace('[%S]', '(n)'),
            })
            register_getters.append({
                "ref_params": "&self, n: u8",
                "mut_params": "&mut self, n: u8",
                "reg_type_variant": register['name'].upper().replace('[%S]', '(n)'),
                "reg_type_variant_lower": stripped_name.lower(),
                "reg_type_variant_struct": stripped_name.upper(),
            })
        else:
            reg_type_variants_match_arms.append({
                "reg_type_variant": register['name'].upper(),
            })
            register_getters.append({
                "ref_params": "&self",
                "mut_params": "&mut self",
                "reg_type_variant": register['name'].upper(),
                "reg_type_variant_lower": register['name'].lower(),
                "reg_type_variant_struct": register['name'].upper(),
            })

    base_addresses = []
    for name, peripheral in sorted(peripheral_group['#derives'].items()):
        base_addresses.append({
            "peripheral_name": name.upper(),
            "peripheral_base_address": peripheral['baseAddress'],
        })

    subtemplate_fields = {
        "base_addresses": base_addresses,
        "modules": modules,
        "cluster_type_variants_match_arms": cluster_type_variants_match_arms,
        "reg_type_variants_match_arms": reg_type_variants_match_arms,
        "register_ref_getters": register_getters,
        "register_mut_getters": register_getters,
        "cluster_reg_ref_getters": cluster_reg_getters,
        "cluster_reg_mut_getters": cluster_reg_getters,
    }

    return {
        'fields': fields,
        'subtemplate_fields': subtemplate_fields,
    }


if __name__ == "__main__":
    # use this script as a rust template reader
    # prints out the fields in the template
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument('template_file_path', type=Path)

    args = parser.parse_args()

    assert args.template_file_path.exists(), \
        "file does not exist: {}".format(str(args.template_file_path))

    with open(args.template_file_path, 'r') as f:
        template = f.read()

    single_fields = list(sorted(set(TEMPLATE_FIELD_PTRN.findall(template))))
    print("fields:")
    for field in single_fields:
        print(f"- {field}")
    print()

    print("subtemplates:")
    subtemplates = list(sorted(set(SUBTEMPLATE_PTRN.findall(template))))
    for name, subtemplate in subtemplates:
        print(name)
        print(subtemplate)
        subfields = list(sorted(set(SUBTEMPLATE_FIELD_PTRN.findall(subtemplate))))
        links = list(sorted(set(SUBTEMPLATE_LINK_PTRN.findall(subtemplate))))
        for subfield in subfields:
            print(f"- {subfield}")
        for link in links:
            print(f"+ {link}")
        print()
