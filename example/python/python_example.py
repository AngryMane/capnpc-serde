#!/usr/bin/env python3

import sys
import json

def main() -> None:
    serialized_data: str = sys.stdin.read()
    json_data: dict = json.loads(serialized_data)
    print(json.dumps(json_data, indent=2))

if __name__ == "__main__":
    main()