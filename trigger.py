import subprocess
import sys


def main() -> int:
    event = sys.argv[1]
    
    subprocess.call("twitch event trigger ")

    return 0


if __name__ == "__main__":
    exit(main())
