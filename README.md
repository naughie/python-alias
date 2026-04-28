# Python-alias

It gives an alias `python3 -> uv run`, useful for running a PEP-723-compatible script with a shebung, say `#!/usr/bin/env python3`.


**Note**: it compiles on Unix (that supports `std::os::unix`) only.
And it spawns the process of `uv run` as a child process, not running via `exec(2)` on the current process. This may cause a minor overhead in runtime.


# Install

One-line command to install:

```
curl -L 'https://github.com/naughie/python-alias/releases/latest/download/python-alias-x86_64-unknown-linux-musl.tar.xz' | tar xvJ --strip-components=1 python-alias-x86_64-unknown-linux-musl/python3
```

Or download from the [Release](https://github.com/naughie/python-alias/releases) page.

Then copy `python3` binary to wherever you want, so long as it takes higher precedence over the real `python3` binary (like `/usr/bin/python3` or `$UV_PYTHON_BIN_DIR/python3`).
