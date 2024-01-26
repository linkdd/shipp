# Shipp - Deadly simple package manager

Managing dependencies yourself in a C/C++ project can be a pain. Conan and vcpkg
might be a solution, but as far as I know, those requires a registry.

I just want to point to a git repository and say "this is my dependency, build it".

This is how **Shipp** was born. If you want to read more about it, I wrote an
article on my blog [here](https://david-delassus.medium.com/thats-it-i-m-making-my-own-c-package-manager-555eecbf7d2e?sk=0a6649fef2325de32ca8ad6f51aaefd5).

## Installation

```
$ cargo install --path .
```

Then make sure `$HOME/.cargo/bin` is in your `$PATH`.

## Usage

Create a package by adding a `shipp.json` file at the root of your Git
repository:

```json
{
  "name": "mypackage",
  "version": "0.1.0",
  "scripts": {
    "build": "make all DESTDIR=$SHIPP_DIST_DIR",
    "install": "make install DESTDIR=$SHIPP_DIST_DIR"
  },
  "dependencies": [
    {
      "name": "libfoo",
      "url": "https://github.com/example/libfoo.git",
      "version": "0.1.0"
    }
  ]
}
```

> **NB:** Your dependencies need to be Shipp packages as well, with their own
> `shipp.json` file at the root of the repository.

Then run the following commands:

**To fetch dependencies (`git clone` or `git pull`) in `.shipp/deps/`:**

```
$ shipp deps.get
```

**To build dependencies and install them in `.shipp/dist/`:**

```
$ shipp deps.build
```

**To build the current project and install it in `.shipp/dist/`:**

```
$ shipp build
```

**To create a `tar.gz` archive of the `.shipp/dist/` folder:**

```
$ shipp dist
```

## Design

When running `shipp deps.get`, Shipp will clone all your dependencies into the
`.shipp/deps` folder. Then, recursively, it will read your dependencies's
`shipp.json` file to pull the transitive dependencies into that same folder.

All your direct and transitive dependencies are therefore automatically vendored
in your project (similar to `node_modules` with `npm`).

> **NB:** Your dependencies should also be a Shipp package (aka: have a
> `shipp.json` file at the root of their repository).

The command `shipp deps.build` will run the `build` command and the `install`
command of your dependencies (in the correct order). Those commands rely on the
dependencies's build system, without you needing to add too much plumbing.

The `build` command is in charge of building the package, and the `install`
command is in charge of installing the built artifacts into the `.shipp/dist`
folder.

This folder should contain the following:

 - `include/`: where your C/C++ headers will be installed
 - `lib/`: where your static/shared libraries will be installed
 - `bin/`: where your executables will be installed
 - `share/` & ...: your resources and other files

This folder is where you should look for to configure the `-I include_dir` and
`-L library_dir` compiler flags (or equivalent for your build system).

Both the `build` and `install` commands are given the following environment
variables:

| Variable | Example value | Description |
| --- | --- | --- |
| `SHIPP_TARGET_ARCH` | `x86_64` | Current platform architecture |
| `SHIPP_TARGET_FAMILY` | `unix` or `windows` | Current platform OS family |
| `SHIPP_TARGET_OS` | `linux` | Current platform OS |
| `SHIPP_DIST_DIR` | `/path/to/.shipp/dist` | Absolute path to the install directory |

See those pages for more information:

 - https://doc.rust-lang.org/std/env/consts/constant.ARCH.html
 - https://doc.rust-lang.org/std/env/consts/constant.FAMILY.html
 - https://doc.rust-lang.org/std/env/consts/constant.OS.html

The `shipp build` command works like `deps.build` but for your current project.

Finally, the `shipp dist` command will archive the `.shipp/dist` folder into an
archive named `myproject-version.tar.gz`, which you should be able to distribute.

## License

This project is released under the terms of the [MIT License](./LICENSE.txt).
