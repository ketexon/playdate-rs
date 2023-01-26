import argparse
from pathlib import Path
import tomllib
from platform import system
import subprocess
from sys import stdout, stderr
from shutil import rmtree, copytree, copy, which

def build(args):
    if system() != "Windows":
        print("Only works on Windows. Aborting...")
        return None

    if which("pdc") is None:
        print("Cannot find PDC. Aborting...")
        return None

    if which("PlaydateSimulator") is None:
        print("Cannot find PlaydateSimulator. Aborting...")
        return None

    lib_ext = ".dll"
    mode = "debug"

    asset_dir = Path.cwd() / "assets"
    source_dir = Path.cwd() / "target" / "playdate" / "source"
    game_dir = Path.cwd() / "target" / "playdate" / "game.pdx"

    # Parse cargo.toml

    package_name = None

    cargo_toml_path = Path.cwd() / "Cargo.toml"
    if cargo_toml_path.is_file():
        with cargo_toml_path.open("rb") as cargo_toml:
            cargo_dict = tomllib.load(cargo_toml)
            if "package" not in cargo_dict or "name" not in cargo_dict["package"]:
                print("Cannot find package name in Cargo.toml. Aborting...")
                return None
            package_name = cargo_dict["package"]["name"]
    else:
        print("Cannot find Cargo.toml. Aborting...")
        return None

    # Run cargo build

    print("Buidling project...")
    cargo_build_result = subprocess.run(["cargo", "build"], stdout=stdout, stderr=stderr)
    if cargo_build_result.returncode != 0:
        print(f"cargo build returned with code {cargo_build_result.returncode}. Aborting...")

    lib_path = Path.cwd() / "target" / mode / f"{package_name}{lib_ext}"
    if not lib_path.is_file():
        print(f"Cannot find output library at \"{str(lib_path)}\". Aborting...")


    if source_dir.is_dir():
        rmtree(source_dir)

    source_dir.mkdir()

    if asset_dir.is_dir():
        copytree(asset_dir, source_dir, dirs_exist_ok=True)

    copy(lib_path, source_dir / f"pdex{lib_ext}")

    bin_path = source_dir / "pdex.bin"
    if not bin_path.is_file():
        bin_path.touch()

    if game_dir.is_dir():
        rmtree(game_dir)
    game_dir.mkdir()

    print("\nRunning PDC...")
    pdc_result = subprocess.run(["pdc", str(source_dir), str(game_dir)], stderr=stderr, stdout=stdout)

    if pdc_result.returncode != 0:
        print(f"PDC returned exit code {pdc_result.returncode}")
        return None

    return game_dir


def run(args):
    game_dir = build(None)
    subprocess.run(["PlaydateSimulator", game_dir])


class DefaultHelpParser(argparse.ArgumentParser):
    def error(self, message):
        stderr.write('error: %s\n' % message)
        self.print_help()
        exit(2)

parser = DefaultHelpParser(
    prog = "playdate-rs-build"
)

subparsers = parser.add_subparsers()
subparsers.required = True

run_parser = subparsers.add_parser("run", aliases=["r"])
run_parser.set_defaults(func=run)


build_parser = subparsers.add_parser("build", aliases=["b"])
build_parser.set_defaults(func=build)

args = parser.parse_args()
args.func(args)