#!/usr/bin/env python3
import os
import json
import argparse
from pathlib import Path
from glob import glob
from collections import namedtuple
from copy import copy

import networkx as nx
import pygraphviz as pgv

from pypcode import Context, OpFormat

def op_fmt(op):
    inputs = ", ".join([OpFormat.fmt_vn(inp) for inp in op.inputs])
    if op.output:
        return "{}(out={}, in=[{}])".format(
            op.opcode.name, OpFormat.fmt_vn(op.output), inputs)

    return "{}(in=[{}])".format(
        op.opcode.name, inputs)

disasm_fmt = lambda ins: f"{ins.addr.offset:#x}: {ins.mnem:<6} {ins.body}"

RENDER_PARENT_DIR = Path(__file__).resolve().parent

LISTING_TEMPLATE = (
"""<!DOCTYPE html>
<html>
<head>
<title>{title}</title>
<style>{style}</style>
</head>
<body>
<div id="options" class="overlay">
<button onclick="togglePcode()">Toggle Pcode</button>
</div>
<div class="listing">
{listing}
</div>
<script>
{script}
</script>
</body>
</html>
"""
)
LISTING_SCRIPT = """
function togglePcode() {
    toggleClass([`.insn-listing .insn-pcode`]);
}

function toggleClass(class_selectors) {
    // Get all the style sheets in the document
    const styleSheets = document.styleSheets;

    // Iterate over all style sheets
    for (let i = 0; i < styleSheets.length; i++) {
        const styleSheet = styleSheets[i];
        // Get all rules in the current style sheet
        const rules = styleSheet.cssRules || styleSheet.rules;

        // Iterate over all the rules in the current style sheet
        for (let j = 0; j < rules.length; j++) {
            const rule = rules[j];

            // Check if the rule applies to the specific class
            if (class_selectors.includes(rule.selectorText)) {
                // Modify the desired CSS property
                if (rule.style.display === "none") {
                    rule.style.display = "";
                } else {
                    rule.style.display = "none";
                }
            }
        }
    }
}
"""
LISTING_STYLE = """
.overlay {
    width: 150px;
    height: auto;
    position: fixed;
    top: 0;
    right: 0;
    z-index: 1;
    overflow-x: hidden;
}
.overlay button {
    display: block;
    width: 100%;
}
.insn-listing {
    font-family: monospace;
    padding: 0px;
    display: grid;
    grid-template-areas:
    "labels addr  bytes mnemonic operands edges"
    "labels blank pcode pcode    pcode    edges";
    grid-template-columns: 150px 70px 90px 70px 1fr 1fr;
    row-gap: 0px;
    column-gap: 10px;
}
.insn-listing:nth-child(even) {
    background-color: #F8F8FF;
}
.insn-listing:nth-child(odd) {
    background-color: #E5E5EE;
}
.insn-listing .insn-labels {
    font-style: italic;
    text-align: right;
    grid-area: labels;
}
.insn-listing .insn-address {
    font-weight: bold;
    text-align: right;
    grid-area: addr;
}
.insn-listing .insn-bytes {
    grid-area: bytes;
}
.insn-listing .insn-mnemonic {
    grid-area: mnemonic;
}
.insn-listing .insn-operands {
    grid-area: operands;
}
.insn-listing .insn-pcode {
    grid-area: pcode;
    display: none;
    font-style: italic;
    font-size: smaller;
    padding-top: 5px;
    padding-bottom: 5px;

    .pcode-offset {
        width: 5%;
        float: left;
        text-align: right;
        padding-right: 10px;
    }
    .insn-pcodeop {
        width: auto;
    }
}
.insn-listing .insn-edges {
    text-align: left;
    grid-area: edges;
}
"""
INSN_LISTING_TEMPLATE = (
    "<div class=\"insn-listing\" id=\"[{address:#x}]\">\n"
    "<div class=\"insn-labels\">{labels}</div>\n"
    "<div class=\"insn-address\">{address:#06x}</div>\n"
    "<div class=\"insn-bytes\">{bytes}</div>\n"
    "<div class=\"insn-mnemonic\">{mnemonic}</div>"
    "<div class=\"insn-operands\">{operands}</div>\n"
    "<div class=\"insn-pcode\">\n{pcode}\n</div>\n"
    "<div class=\"insn-edges\">\n{edges}\n</div>\n"
    "</div>"
)
fmt_pcode_html = lambda pcode: "\n".join([(
        f"<div class=\"pcode-offset\">{i}</div>"
        f"<div class=\"insn-pcodeop\">{op}</div>\n"
    ) for op in pcode 
])
def fmt_edges_html(block, successors, predecessors):
    edges = [(
            "<span class=\"insn-edge\">"
            f"<a href=\"#[{block.address:#x}]\">{block.address:#x}</a>&lt;-"
            f"<a href=\"#[{pred:#x}]\">{pred:#x}</a>"
            "</span>"
        ) for pred in predecessors
    ]
    last_address = block.insn_addrs[-1]
    edges.extend([(
            "<span class=\"insn-edge\">"
            f"<a href=\"#[{last_address:#x}]\">{last_address:#x}</a>-&gt;"
            f"<a href=\"#[{succ:#x}]\">{succ:#x}</a>"
            "</span>"
        ) for succ in successors
    ])

    return "<br/>\n".join(edges)

def to_insn_html(block, lift_data):
    insns = []
    for i, insn in enumerate(lift_data):
        successors = [] if i != (len(block.insn_addrs) - 1) else block.successors
        predecessors = [] if i != 0 else block.predecessors
        labels = "" if i != 0 else (
            f"<span class=\"insn-label\">block_{block.address:#06x}</span>")
        edges = fmt_edges_html(block, successors, predecessors)
        insn = copy(insn)
        insn['pcode'] = "<br/>\n".join(insn['pcode'])

        insns.append(INSN_LISTING_TEMPLATE.format(
            labels=labels,
            edges=edges,
            **insn))
    return insns

BLOCK_DOT_TEMPLATE = """
{pad}{address} [
{pad}{pad}label=<<font face="monospace" point-size="6">
{pad}{pad}<table align="left" cellborder="0" cellpadding="0" cellspacing="0">
{pad}{pad}<tr><td><b>{address:#x}</b></td></tr>
{pad}{pad}{pad}{content}
{pad}{pad}</table>
{pad}{pad}</font>>,
{pad}{pad}shape=none,
{pad}{pad}address={address},
{pad}{pad}block_size={size},
{pad}{pad}insn_addrs="{insn_addrs}"
{pad}];
{successors}
"""

SimpleBBlock = namedtuple("SimpleBBlock", [
    "address",
    "size",
    "insn_addrs",
    "predecessors",
    "successors",
])

def get_block_data(path, cfg):
    # read block data from path
    data = {}
    with open(path, 'rb') as f:
        binary = f.read()
    for node in sorted(cfg['nodes'], key=lambda blk: blk.address):
        data[node.address] = { "raw": binary[node.address:node.address + node.size] }
    return data

def to_hex(b):
    from itertools import islice
    def chunks(n, b):
        it = iter(b)
        while True:
            chunk = bytes(islice(it, n))
            if not chunk:
                return
            yield chunk
    return " ".join([f"{int.from_bytes(c, 'little'):04x}" for c in chunks(2, b)])


def disasm(data):
    disasm_data = {}
    ctx = Context("ARM:LE:32:Cortex")
    for address, info in data.items():
        dx = ctx.disassemble(info["raw"], base_address=address)
        insns = []
        for ins in dx.instructions:
            offset = ins.addr.offset - address
            tx = ctx.translate(info["raw"],
                base_address=ins.addr.offset,
                offset=offset,
                max_bytes=ins.length)
            pcode = [op_fmt(op) for op in tx.ops]
            insns.append({
                "address": ins.addr.offset,
                "mnemonic": ins.mnem,
                "operands": ins.body,
                "bytes": to_hex(info["raw"][offset:offset+ins.length]),
                "disasm": disasm_fmt(ins),
                "pcode": pcode,
            })
        disasm_data[address] = insns
    return disasm_data

DISM_PCODE_FMT_STR = (
    "<tr><td align=\"left\" balign=\"left\">\n{pad}{disasm}\n{pad}</td></tr>"
    "\n{pad}"
    "<tr><td align=\"left\" balign=\"left\">{pcode}\n{pad}</td></tr>"
)
DISM_FMT_STR = (
    "<tr><td align=\"left\" balign=\"left\">{disasm}</td></tr>")

def fmt_insns(insns, pad="\t", pcode=True, tainted=set()):
    # expect list of dicts { "address": int, "disasm": str, "pcode": list[str] }

    if pcode:
        fmt_insn = lambda data, tainted: DISM_PCODE_FMT_STR.format(
            pad=pad,
            disasm=f"<font color=\"tomato\">{data['disasm']}</font>" \
                if tainted else data['disasm'],
            pcode="<br/>".join([f"\n{pad}&nbsp;&nbsp;{p}" for p in data['pcode']]))
    else:
        fmt_insn = lambda data, tainted: DISM_FMT_STR.format(
            pad=pad,
            disasm=f"<font color=\"tomato\">{data['disasm']}</font>" \
                if tainted else data['disasm'])

    formatted = []
    for insn in insns:
        if insn['address'] in tainted:
            formatted.append(fmt_insn(insn, True))
        else:
            formatted.append(fmt_insn(insn, False))

    return f"\n{pad}".join(formatted)

def to_dot(bblock, disasm_data, pad="\t", pcode=True, tainted=set()):
    assert isinstance(bblock, SimpleBBlock), \
        "expected a SimpleBBlock: {}".format(bblock)

    insns = disasm_data[bblock.address]
    successors = ""
    if bblock.successors:
        successors = "{pad}{address} -> {{ {successors} }};".format(
            pad=pad,
            address=bblock.address,
            successors=", ".join([str(v) for v in bblock.successors]))
    return BLOCK_DOT_TEMPLATE.format(
        pad=pad,
        address=bblock.address,
        size=bblock.size,
        insn_addrs=bblock.insn_addrs,
        content=fmt_insns(insns, pad=pad*3, pcode=pcode, tainted=tainted),
        successors=successors)

def build_graph(cfg):
    graph = nx.DiGraph()
    for node in cfg['nodes']:
        graph.add_node(node.address, 
            address=node.address,
            size=node.size,
            insn_addrs=node.insn_addrs)

    for node in cfg['nodes']:
        graph.add_edges_from([(node.address, child) for child in node.successors])

    return graph

def build_dot(cfg, bin_path):
    block_data = get_block_data(bin_path, cfg)
    disasm_data = disasm(block_data)

    dot = ["""digraph "" {"""]
    for block in cfg['nodes']:
        dot_block = to_dot(block, disasm_data)
        dot.append(dot_block)
    dot.append("}\n")

    dot_graph = "\n".join(dot)
    return dot_graph

def build_listing(cfg, bin_path):
    block_data = get_block_data(bin_path, cfg)
    disasm_data = disasm(block_data)

    listing = []
    last_address = 0
    for block in sorted(cfg['nodes'], key=lambda blk: blk.address):
        if block.address != last_address:
            listing += ["<br/>"]
        lift_data = disasm_data[block.address]

        insns_html = to_insn_html(block, lift_data)
        listing.extend(insns_html)
        last_address = block.address + block.size

    html = LISTING_TEMPLATE.format(
        title=f"{bin_path.name}",
        style=LISTING_STYLE,
        script=LISTING_SCRIPT,
        listing="\n".join(listing))
    return html


def draw_dot(dot_graph, svg_path):
    viz_graph = pgv.AGraph(dot_graph)
    viz_graph.layout(prog="dot")
    viz_graph.draw(str(svg_path))


def get_tainted_locs(path):
    # read tainted addresses from file
    # ignore pcode offsets for now because imark changes offset,
    # which is annoying
    locs = set()
    with open(path, 'r') as f:
        for line in f.readlines():
            address, position = line.strip().split('\t')
            address = int(address, 0)
            position = int(position, 0)
            locs.add(address)
    return locs

def find_bin(target_path: Path, search_locs: list[Path]=[]):
    bin_path = None
    for loc in search_locs:
        name = target_path.stem.replace(".simple-cfg", "")
        if (res := list(loc.glob(f"**/{name}.bin"))):
            bin_path = res[0]
            break
    else:
        print(f"binary not found: {str(target_path)}")
    return bin_path

def load_cfg(target_path: Path):
    with open(target_path, 'r') as cfgfile:
        cfg = json.load(cfgfile)

    cfg['nodes'] = [ SimpleBBlock(**block) for block in cfg['nodes'] ]
    return cfg

def load_tainted(target_path: Path, tainted_locs_dir: list[Path]=[]):
    tainted = set()
    for loc in tainted_locs_dir:
        loc = Path(loc)
        name = target_path.stem.replace(".simple-cfg", ".tainted-locs")
        if (res := list(loc.glob(f"**/{name}.tsv"))):
            path = res[0]
            tainted = get_tainted_locs(path)

    return tainted

def build_dot(cfg: dict, bin_path: Path, tainted: set, pcode=False):
    graph = nx.DiGraph()
    for node in cfg['nodes']:
        graph.add_node(node.address,
            address=node.address,
            size=node.size,
            insn_addrs=node.insn_addrs)

    for node in cfg['nodes']:
        graph.add_edges_from([(node.address, child) for child in node.successors])

    block_data = get_block_data(bin_path, cfg)
    disasm_data = disasm(block_data)

    dot = ["""digraph "" {"""]
    for block in cfg['nodes']:
        dot_block = to_dot(block, disasm_data, pcode=pcode, tainted=tainted)
        dot.append(dot_block)
    dot.append("}\n")

    dot_graph = "\n".join(dot)
    return dot_graph

def draw_dot(dot_graph: str, dst: Path):
    viz_graph = pgv.AGraph(dot_graph)
    viz_graph.layout(prog="dot")
    viz_graph.draw(str(dst))
    print(f"rendered {str(dst)}")


if __name__ == "__main__":
    DEFAULT_TARGETS = glob("./*simple-cfg.json")
    DEFAULT_SEARCH_LOCS = [Path(".")]

    parser = argparse.ArgumentParser()
    parser.add_argument('targets', nargs='*', default=DEFAULT_TARGETS,
        help="cfg json files to render")
    parser.add_argument('-s', '--search-locs', dest="search_locs", nargs='+', type=Path, default=DEFAULT_SEARCH_LOCS,
        help="locations to search for the binaries")
    parser.add_argument('-d', dest="outdir", type=Path, default=Path("./rendered-cfgs"),
        help="path to desired output directory")
    parser.add_argument('--tainted-locs', dest="tainted_locs_dir", type=Path, nargs='+', default=[],
        help="path to tainted-locs.tsv files")

    args = parser.parse_args()

    # make output directory if it doesn't exist
    args.outdir.mkdir(exist_ok=True, parents=True)

    for target_path in args.targets:
        target_path = Path(target_path)
        assert target_path.exists(), \
            "target does not exist: {}".format(target_path)

        bin_path = find_bin(target_path, args.search_locs)
        cfg = load_cfg(target_path)
        tainted = load_tainted(target_path, args.tainted_locs_dir)

        # build listing
        html_path = args.outdir / f"{target_path.stem}.html"
        html = build_listing(cfg, bin_path)
        with open(html_path, 'w') as f:
            f.write(html)
        print(f"rendered {str(html_path)}")

        dot_graph = build_dot(cfg, bin_path, tainted)

        dot_path = args.outdir / f"{target_path.stem}.dot"
        # nx.nx_agraph.write_dot(graph, str(dot_path))
        with open(dot_path, 'w') as f:
            f.write(dot_graph)
        
        svg_path = args.outdir / f"{dot_path.stem}.svg"
        draw_dot(dot_graph, svg_path)



