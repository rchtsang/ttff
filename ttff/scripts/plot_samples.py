#!/usr/bin/env python3
import os
import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path

def get_coverage_data(path: Path):
    times = []
    count = []
    addrs = []
    sizes = []
    with open(path, 'r') as f:
        for i, line in enumerate(f.readlines()):
            time, block = line.strip().split('\t')
            time = float(time)
            block_address, block_size = block.split(',')
            address = int(block_address, 0)
            size = int(block_size, 0)
            times.append(time)
            count.append(i)
            addrs.append(address)
            sizes.append(size)
    return [np.array(times), np.array(count), np.array(addrs), np.array(sizes)]

if __name__ == "__main__":
    OUTDIR = Path("./coverage-plots")
    OUTDIR.mkdir(exist_ok=True, parents=True)

    TARGETS = {
        "uart-jump": {
            "color": "blue",
            "linewidth": 2,
            "label": "Tainted Jump",
        },
        "uart-address": {
            "color": "green",
            "linewidth": 2,
            "label": "Tainted Address",
        },
        "uart-int-overflow": {
            "color": "purple",
            "linewidth": 2,
            "label": "Tainted Overflow",
        },
    }

    with plt.rc_context({'font.family': 'Arial Rounded MT Bold'}):
        fig, ax = plt.subplots(figsize=(6, 4), layout='tight')

        ax.set_xlabel("Time (s)",
            # fontname="Arial Rounded MT Bold",
            fontsize=18)
        ax.set_ylabel("Blocks Found",
            # fontname="Arial Rounded MT Bold",
            fontsize=18)

        for target, kwargs in TARGETS.items():
            path = Path(f"../examples/{target}/{target}.blk-cov.tsv")
            assert path.exists(), \
                "file not found: {}".format(path)

            data = get_coverage_data(path)

            times = data[0]
            count = data[1]

            print(f"plotting {target}...")
            ax.plot(times, count, **kwargs)

        fig.legend(loc='lower right', bbox_to_anchor=(0, 0.2, 0.95, 0.75))

        dst = OUTDIR / "block-coverage.svg"
        fig.savefig(dst, transparent=True)


