#!/usr/bin/env python3
"""Patch cargo-dist's palate-capi Homebrew formula for C library layout.

cargo-dist can generate a formula for library archives when package-libraries and
install-libraries are enabled, but unknown included directories are installed to
pkgshare. For C consumers, headers and pkg-config metadata must be installed into
Homebrew's conventional include/ and lib/pkgconfig/ locations so pkg-config works
without custom PKG_CONFIG_PATH.
"""

from __future__ import annotations

import pathlib
import sys


def patch_formula(path: pathlib.Path) -> None:
    text = path.read_text(encoding="utf-8")

    install_marker = """\n    install_binary_aliases!\n\n    # Homebrew will automatically install these, so we don't need to do that\n"""
    install_replacement = """\n    include.install "include/palate.h" if File.exist?("include/palate.h")\n    if File.exist?("lib/pkgconfig/palate-capi.pc")\n      (lib/"pkgconfig").install "lib/pkgconfig/palate-capi.pc"\n      # Archives keep libraries at the archive root for cargo-dist. Homebrew\n      # installs them under lib/, so adjust the relocatable pkg-config file.\n      inreplace lib/"pkgconfig/palate-capi.pc", 'libdir=${prefix}', 'libdir=${prefix}/lib'\n    end\n\n    install_binary_aliases!\n\n    # Homebrew will automatically install these, so we don't need to do that\n"""

    if install_marker not in text:
        raise SystemExit(f"could not find install marker in {path}")
    text = text.replace(install_marker, install_replacement, 1)

    leftover_marker = """    leftover_contents = Dir["*"] - doc_files\n"""
    leftover_replacement = """    leftover_contents = Dir["*"] - doc_files - ["include", "lib"]\n"""

    if leftover_marker not in text:
        raise SystemExit(f"could not find leftover marker in {path}")
    text = text.replace(leftover_marker, leftover_replacement, 1)

    path.write_text(text, encoding="utf-8")


def main() -> None:
    path = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "target/distrib/palate-capi.rb")
    if not path.exists():
        raise SystemExit(f"formula not found: {path}")
    patch_formula(path)


if __name__ == "__main__":
    main()
