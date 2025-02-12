import re
from copy import copy
from pathlib import Path
from collections import defaultdict

from lxml import etree

BITRANGE_PTRN = re.compile(r"\[(?P<msb>\d+):(?P<lsb>\d+)\]")

def load_etree(path):
    """load xml file from path as lxml etree"""
    assert isinstance(path, Path)
    assert path.exists(), "file does not exist: {}".format(str(path))
    with open(path, 'rb') as f:
        bytedata = f.read()
    return etree.fromstring(bytedata)

_to_list = ['fields', 'enumeratedValues', 'dimArrayIndex', 'registers', 'cluster', 'register']

def format_data(tag, data):
    match tag:
        case "field":
            if "bitOffset" in data and "bitWidth" in data:
                return data
            if "msb" in data or "lsb" in data:
                assert "msb" in data and "lsb" in data
                lsb = int(data["lsb"], 0)
                msb = int(data["msb"], 0)
                del data["lsb"]
                del data["msb"]
            elif "bitRange" in data:
                assert (match := BITRANGE_PTRN.search(data["bitRange"]))
                lsb = int(match.group('lsb'), 0)
                msb = int(match.group('msb'), 0)
                del data["bitRange"]
            data["bitOffset"] = lsb
            data["bitWidth"] = msb - lsb + 1
            return data
        
        case "fields" | "enumeratedValues":
            return data[tag[:-1]]
        
        case "dimArrayIndex":
            return data['enumeratedValue']
        
        case "registers":
            values = []
            if 'register' in data:
                assert isinstance(data['register'], list)
                values.extend(data['register'])
            if 'cluster' in data:
                assert isinstance(data['cluster'], list)
                values.extend(data['cluster'])
            return values
        
        case "cluster":
            regs = []
            if 'register' in data:
                assert isinstance(data['register'], list)
                regs.extend(data['register'])
                del data['register']
            if 'cluster' in data:
                assert isinstance(data['cluster'], list)
                regs.extend(data['cluster'])
                del data['cluster']
            data['registers'] = regs
            return data

        case "peripherals":
            peripherals = {}
            for peripheral in data['peripheral']:
                peripherals[peripheral['name'].lower()] = peripheral
            return peripherals

        case _:
            return data

def element_to_dict(elem):
    """recursively transform element to dictionary"""
    children = elem.getchildren()
    if not children:
        return elem.tag, elem.text
    data = {}
    data["#tag"] = elem.tag
    data["#text"] = elem.text.strip()
    for key, value in elem.items():
        data[f"@{key}"] = value
    for child in children:
        tag, value = element_to_dict(child)
        if tag in data:
            if isinstance(data[tag], list):
                data[tag].append(value)
            else:
                data[tag] = [data[tag], value]
        else:
            data[tag] = value
    for key in _to_list:
        if key in data and not isinstance(data[key], list):
            data[key] = [data[key]]
    data = format_data(elem.tag, data)
    return elem.tag, data


def load_svd(path):
    """load svd file as dict of dicts"""
    tree = load_etree(path)
    _, device = element_to_dict(tree)

    # convert peripherals to peripheral groups
    derived = defaultdict(dict)
    peripheral_groups = {}
    for name, peripheral in device["peripherals"].items():
        if "@derivedFrom" in peripheral:
            src = peripheral['@derivedFrom']
            src_peripheral = device['peripherals'][src.lower()]
            group = src_peripheral['groupName'].lower()
            derived[group][name] = peripheral
            continue
        if "groupName" not in peripheral:
            continue

        group = peripheral['groupName']
        base_peripheral = copy(peripheral)
        base_peripheral['name'] = group
        derived[group.lower()][name] = peripheral
        peripheral_groups[group.lower()] = base_peripheral

    for group, peripheral_group in peripheral_groups.items():
        peripheral_group['#derives'] = derived[group]

    device['#peripheral_groups'] = peripheral_groups

    return device