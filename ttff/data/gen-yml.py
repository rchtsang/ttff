import argparse
from pathlib import Path
from svdparser.parse import load_svd
from svdparser.generate import generate_platform 

parser = argparse.ArgumentParser()
parser.add_argument('svdpath', type=Path)
parser.add_argument('dstpath', type=Path)

args = parser.parse_args()

svd = load_svd(args.svdpath)
generate_platform(args.dstpath, svd, "platform.yml")

