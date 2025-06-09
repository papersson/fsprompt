# Linux Binary Compatibility

## glibc Version Requirements

Our Linux binaries are built on Ubuntu 22.04, which uses **glibc 2.35**.

This means the binaries are compatible with:
- Ubuntu 22.04 and newer
- Debian 12 (Bookworm) and newer
- Fedora 36 and newer
- RHEL/CentOS 9 and newer
- Most Linux distributions from 2022 onwards

## Common glibc Versions

| Distribution | glibc Version |
|-------------|---------------|
| Ubuntu 20.04 | 2.31 |
| Ubuntu 22.04 | 2.35 |
| Ubuntu 24.04 | 2.39 |
| Debian 11 | 2.31 |
| Debian 12 | 2.36 |
| WSL Ubuntu 20.04 | 2.31 |
| WSL Ubuntu 22.04 | 2.35 |

## Checking Your glibc Version

```bash
ldd --version
```

## If You Get glibc Errors

If you see errors like:
```
fsprompt: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.35' not found
```

You have three options:

### 1. Build from Source (Recommended)
This will use your system's glibc version:
```bash
cargo install --git https://github.com/papersson/fsprompt.git
```

### 2. Upgrade Your Distribution
For WSL:
```bash
# Check current version
lsb_release -a

# Upgrade Ubuntu 20.04 to 22.04
sudo do-release-upgrade
```

### 3. Use Docker/Podman
```bash
# Run in a container with newer glibc
docker run -it --rm -v $(pwd):/workspace ubuntu:22.04
```

## Why We Use Ubuntu 22.04

We chose Ubuntu 22.04 for building because:
- It's LTS (Long Term Support) until 2027
- Provides good compatibility (glibc 2.35)
- Supported by GitHub Actions until at least 2027
- Most modern systems have glibc 2.35+

Ubuntu 20.04 (glibc 2.31) will be deprecated in GitHub Actions by April 2025, so we're using 22.04 for future-proofing while maintaining reasonable compatibility.