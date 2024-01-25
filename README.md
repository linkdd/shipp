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

## License

This project is released under the terms of the [MIT License](./LICENSE.txt).
