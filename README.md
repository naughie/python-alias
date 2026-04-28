# Python-alias

It gives an alias `python3 -> uv run`, useful for running a PEP-723-compatible script with a shebung, say `#!/usr/bin/env python3`.


**Note**: it compiles on Unix (that supports `std::os::unix`) only.
And it spawns the process of `uv run` as a child process, not running via `exec(2)` on the current process. This may cause a minor overhead in runtime.
