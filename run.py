from os import system
from sys import argv

def main():
    full_cmd = "cd runner && cargo "

    ## Make better user interaction by checking len(argv) etc.

    cmd = argv[1]

    if cmd == "build":
        full_cmd += "build"
    elif cmd == "run":
        full_cmd += "run"
    elif cmd == "test":
        full_cmd += "run --features=test"
    
    exit(system(full_cmd))

if __name__ == "__main__":
    main()