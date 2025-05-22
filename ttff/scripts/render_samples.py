if __name__ == "__main__":
    from render_cfg import *

    OUTDIR = Path("./rendered-cfgs")
    OUTDIR.mkdir(exist_ok=True, parents=True)

    TARGETS = [ "uart-jump", "uart-address", "uart-int-overflow" ]

    for target in TARGETS:
        print(f"rendering target: {target}")
        cfg_path = Path(f"../examples/{target}/{target}.simple-cfg.json")
        assert cfg_path.exists(), \
            "target does not exist: {}".format(cfg_path)

        bin_search_loc = Path(f"../examples/samples/{target}")
        tainted_search_loc = Path(f"../examples/{target}")

        bin_path = find_bin(cfg_path, [bin_search_loc])
        cfg = load_cfg(cfg_path)
        tainted = load_tainted(cfg_path, [tainted_search_loc])

        dot_graph = build_dot(cfg, bin_path, tainted)

        dot_path = OUTDIR / f"{cfg_path.stem}.dot"
        svg_path = OUTDIR / f"{cfg_path.stem}.svg"

        with open(dot_path, 'w') as f:
            f.write(dot_graph)
        print(f"rendered {str(dot_path)}")

        draw_dot(dot_graph, svg_path)

