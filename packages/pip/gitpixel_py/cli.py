import os
import platform
import stat
import subprocess
import sys
import urllib.request

REPO = "Smasduq/git-pixel"
VERSION = "0.1.0"
TAG = f"v{VERSION}"


def _map_target():
    system = platform.system()
    machine = platform.machine().lower()
    if system == "Windows":
        return "x86_64-pc-windows-msvc"
    if system == "Darwin":
        if machine in ("arm64", "aarch64"):
            return "aarch64-apple-darwin"
        return "x86_64-apple-darwin"
    if system == "Linux":
        if machine in ("aarch64", "arm64"):
            return "aarch64-unknown-linux-gnu"
        return "x86_64-unknown-linux-gnu"
    raise RuntimeError(f"Unsupported platform: {system}/{machine}")


def _archive_name(target):
    return f"{target}.zip" if target == "x86_64-pc-windows-msvc" else f"{target}.tar.gz"


def _binary_dir():
    package_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(package_dir, "_bin")


def _bin_path():
    name = "gitpixel.exe" if platform.system() == "Windows" else "gitpixel"
    return os.path.join(_binary_dir(), name)


def ensure_binary():
    target = _map_target()
    bin_path = _bin_path()
    if os.path.exists(bin_path):
        return bin_path

    os.makedirs(_binary_dir(), exist_ok=True)
    archive = _archive_name(target)
    url = f"https://github.com/{REPO}/releases/download/{TAG}/{archive}"
    local = os.path.join(_binary_dir(), archive)

    print(f"[gitpixel] downloading binary for {target} ...", file=sys.stderr)
    urllib.request.urlretrieve(url, local)
    if archive.endswith(".zip"):
        import zipfile

        with zipfile.ZipFile(local, "r") as z:
            z.extractall(_binary_dir())
    else:
        import tarfile

        with tarfile.open(local, "r:gz") as t:
            extract_member = None
            for m in t.getmembers():
                if m.name.endswith("gitpixel"):
                    extract_member = m
                    break
            if extract_member is None:
                raise RuntimeError("gitpixel binary not found in archive")
            f = t.extractfile(extract_member)
            with open(bin_path, "wb") as out:
                out.write(f.read())
    os.remove(local)

    if os.path.exists(bin_path) and platform.system() != "Windows":
        os.chmod(bin_path, os.stat(bin_path).st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    if not os.path.exists(bin_path):
        raise RuntimeError(f"Installed binary not found at {bin_path}. Check {url}")
    return bin_path


def main():
    binary = ensure_binary()
    code = subprocess.call([binary] + sys.argv[1:])
    sys.exit(code)


if __name__ == "__main__":
    main()
