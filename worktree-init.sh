#!/bin/sh

if [ ! -e worktree-init.sh ]; then
    echo >&2 "worktree-init must be invoked from root of the repository."
    exit 1
fi

# initialize fugue-core submodule
if [ ! -d fugue-core/.git ]; then
    git submodule update --init --recursive fugue-core/ \
        && cd fugue-core \
        && git pull origin cme \
        && git checkout cme
    cd -
fi

# extract ghidra processor definitions
if [ ! -d libcme/data/processors ]; then
    cd libcme/data \
        && tar xvf processors.tar.gz
    cd -
fi

