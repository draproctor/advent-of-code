#!/bin/env fbpython
import os
import subprocess
import argparse

PACKMAN = "/usr/local/bin/packman"


def main(target: str, binary: str, server: str, server_path: str) -> None:
    rpm_dir = packman_build_output_dir(args.target)
    rpm_path = newest_file(rpm_dir)
    binary_path = extract_rpm_binary(rpm_path)
    push_binary(binary_path, server, server_path)
    os.rm(binary_path)


def packman_build_output_dir(target: str) -> str:
    if not os.path.exists(PACKMAN):
        print(f"Packman is missing from {PACKMAN}")
        exit(1)
    if not os.path.exists(target):
        print(f"Target does not exist: {target}")
        exit(1)

    # All of Packman's output is written to stderr.
    _stdout, stderr = run_packman(target).communicate()
    stderr_lines = stderr.decode().split("\n")
    output_line = list(
        filter(lambda l: l.startswith("=> Copying built rpm(s)"), stderr_lines)
    )
    if not output_line:
        print("Unable to find output directory. Packman output:")
        print(stderr_lines)
        exit(1)

    output_dir_parts = list(filter(None, output_line[0].split(" ")))
    output_dir = output_dir_parts[-1].strip(":")

    if not os.path.exists(output_dir):
        print(f"Output directory does not exist: {output_dir}")
        print(stderr.decode())
        exit(1)

    print(f"Packman output directory: {output_dir}")
    return output_dir


def run_packman(target: str):
    cmd = f"{PACKMAN} build {target} --buck2"
    print(f"Running packman: {cmd}")
    return subprocess.Popen(
        cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )


def newest_file(dir_path: str) -> str:
    if not dir_path:
        print(f"Directory path is empty: {dir_path}")
        exit(1)
    files = [os.path.join(dir_path, f) for f in os.listdir(dir_path)]
    if not files:
        print(f"No files found in {dir_path}")
        exit(1)
    newest = max(files, key=os.path.getctime)
    return os.path.join(dir_path, newest)


def extract_rpm_binary(rpm_path: str) -> str:
    command = f"rpm2cpio {rpm_path} | cpio -idmv"
    _stdout, stderr = subprocess.Popen(
        command, shell=True, stderr=subprocess.PIPE
    ).communicate()
    if not stderr:
        print(f"Unable to extract binary from {rpm_path}")
        exit(1)
    output_lines = stderr.decode().split("\n")
    if not output_lines:
        print(f"Unable to find binary in {rpm_path}")
        print(stderr)
        exit(1)
    return output_lines[-1]


def push_binary(binary_path: str, server: str, server_path: str) -> None:
    scp_cmd = f"scp {binary_path} root@{server}:{server_path}"
    print(f"Running scp: {scp_cmd}")
    _ = subprocess.run(scp_cmd, shell=True, check=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="build_test_binary",
        description="""
        Builds a packman target and then moves the
        generated binary to the requested server.
        """,
    )
    parser.add_argument("target", help="The packman YAML file.")
    parser.add_argument("binary", help="The name of the binary to move.")
    parser.add_argument("server", help="The FQDN of the server to move the binary to.")
    parser.add_argument(
        "server_path", help="The path on the server to move the binary to."
    )
    args = parser.parse_args()

    main(args.target, args.binary, args.server, args.server_path)
