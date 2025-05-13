#!/usr/bin/env python3

import os
import re
import argparse
from pathlib import Path
from datetime import datetime
from collections import namedtuple
from string import Template

"""
parse generated fuzzing log files and convert to structured data 
"""

# ansi 7-bit escape pattern courtesy of martijn pieters
# https://stackoverflow.com/questions/14693701/
ANSI_ESC_PTRN = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')
TIMESTAMP_PTRN = re.compile(r"(?=\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}.\d{6}Z)")
LOG_ENTRY_PTRN = re.compile(
    r"^(?P<timestamp>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}.\d{6}Z)\s+"
    r"(?P<log_level>\w+)\s+"
    r"(?P<call_stack>[^\n]+):\s+"
    r"(?P<source_file>[^:]+):(?P<line>\d+):\s+"
    r"(?P<message>.*)",
    re.DOTALL)


# https://stackoverflow.com/questions/8906926
class DeltaTemplate(Template):
    delimiter = "%"

def strfdelta(tdelta, fmt):
    d = {"D": tdelta.days}
    hours, rem = divmod(tdelta.seconds, 3600)
    minutes, seconds = divmod(rem, 60)
    d["H"] = '{:02d}'.format(hours)
    d["M"] = '{:02d}'.format(minutes)
    d["S"] = '{:02d}'.format(seconds)
    d["f"] = '{:06d}'.format(tdelta.microseconds)
    t = DeltaTemplate(fmt)
    return t.substitute(**d)


class LogEntry:
    __slots__ = ["delta", "time", "level", "calls", "src", "line", "msg"]

    def __init__(self, delta, time, level, calls, src, line, msg):
        self.delta = delta
        self.time = time
        self.level = level
        self.calls = calls
        self.src = src
        self.line = line
        self.msg = msg

    def __repr__(self):
        return "{delta} {time} {lvl:>5} {calls}: {src}:{line}: {msg}".format(
            delta=strfdelta(self.delta, "%H:%M:%S.%f"),
            time=self.time.strftime("%Y-%m-%dT%H:%M:%S.%fZ"),
            lvl=self.level,
            calls=self.calls,
            src=self.src,
            line=self.line,
            msg=self.msg)



def strip_ansi(text: str):
    return ANSI_ESC_PTRN.sub('', text)

def parse_log(text: str):
    entries = TIMESTAMP_PTRN.split(text)[1:]
    start = None
    log = []
    for i, entry in enumerate(entries):
        entry = entry.strip()
        if not (match := LOG_ENTRY_PTRN.search(entry)):
            print(f"failed to match line {i}/{len(entries)}: {entry}")
            continue
        time = datetime.strptime(
            match.group("timestamp"),
            "%Y-%m-%dT%H:%M:%S.%fZ")
        level = match.group("log_level")
        calls = match.group("call_stack")
        src = match.group("source_file")
        line = match.group("line")
        msg = match.group("message")
        
        if i == 0:
            start = time
        delta = time - start

        entry = LogEntry(delta, time, level, calls, src, line, msg)

        log.append(entry)

    return log

def block_discovery_data(log: list[LogEntry]):
    BLOCK_PTRN = re.compile(
        r">> new block found: \((?P<address>0x[0-9a-fA-F]+|\d+), (?P<size>\d+)\)")

    data = []
    for entry in log:
        if not (match := BLOCK_PTRN.search(entry.msg)):
            continue
        address = int(match.group("address"), 0)
        size = int(match.group("size"), 0)
        data.append((entry.delta.total_seconds(), (address, size)))

    return data

def monitor_logs(log: list[LogEntry]):
    monitor_log = []
    for entry in log:
        if not re.search(r"(?=\(GLOBAL\)|\(CLIENT\))", entry.msg):
            continue
        monitor_log.append((entry.delta, entry.msg))
    return monitor_log

if __name__ == "__main__":
    parser = argparse.ArgumentParser("parse-logs",
        description=__doc__)
    parser.add_argument('path', type=Path,
        help="path to log file")
    
    args = parser.parse_args()

    with open(args.path, 'r') as f:
        log_content = f.read()

    log = parse_log(strip_ansi(log_content))

    with open(args.path.with_suffix('.txt.log'), 'w') as f:
        f.write('\n'.join([repr(e) for e in log]))

    block_data = block_discovery_data(log)

