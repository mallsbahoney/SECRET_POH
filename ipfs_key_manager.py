#!/usr/bin/env python3
# Auto IPFS Key Management Module

import os
import subprocess

def generate_key(name="poh_node_key"):
    result = subprocess.run(["ipfs", "key", "gen", name], capture_output=True, text=True)
    return result.stdout if result.returncode == 0 else result.stderr

def list_keys():
    result = subprocess.run(["ipfs", "key", "list", "-l"], capture_output=True, text=True)
    return result.stdout if result.returncode == 0 else result.stderr

if __name__ == "__main__":
    print("Available IPFS Keys:")
    print(list_keys())