# Building

## Build Dependencies

To run the build at all, you need both `meson` and `ninja`. The recommended installation
method is with `pip`:

```sh
$ python3 -m pip install meson ninja
```

You may also install them with `apt`, but keep in mind the version *may* be too old to
work correctly depending on your Linux Distribution and several other factors. If you
must install using `apt`, use:

```sh
$ sudo apt-get install meson ninja-build
```

## Dependencies

There are a few dependencies, for AFL++ primarily.

<!--TODO: Better docs for dependencies, specify for recent ubuntu, fedora, debian -->

```sh
$ sudo apt-get install build-essential python3-dev automake cmake git flex bison libglib2.0-dev libpixman-1-dev \
    python3-setuptools cargo libgtk-3-dev gcc-11 g++-11 gcc-11-plugin-dev libstdc++-11-dev \
    clang-14 clang-tools-14 libc++-14-dev:amd64 libc++-dev:amd64 libc++1:amd64 \
    libc++1-14:amd64 libc++abi-14-dev:amd64 libc++abi-dev:amd64 libc++abi1:amd64 \
    libc++abi1-14:amd64 libclang-14-dev libclang-common-14-dev libclang-cpp14 \
    libclang1-14 liblldb-14 liblldb-14-dev liblldb-dev:amd64 libllvm-14-ocaml-dev \
    libllvm14:amd64 lld-14 lldb-14 llvm-14 llvm-14-dev llvm-14-linker-tools \
    llvm-14-runtime llvm-14-tools python3-clang:amd64 python3-clang-14 python3-lldb-14
```

You will also need a working simics installation somewhere on your system. You can 
obtain this installation by following the directions [here](https://www.intel.com/content/www/us/en/developer/articles/guide/simics-simulator-installation.html).

After installing simics, you should have a directory (your installation directory) that
looks something like this:

```sh
rhart@rhart-ubuntu2204-dev:~/hub/tool.fuzzing.simics.simics-fuzzing$ ls ~/simics/simics
manifests       simics-docea-base-6.0.23   simics-pkg-mgr-tmp-rhart       simics-qsp-cpu-6.0.12     simics-qsp-x86-6.0.65      simics-viewer-6.0.16
simics-6.0.157  simics-oss-sources-6.0.50  simics-qsp-clear-linux-6.0.14  simics-qsp-isim-6.0.pre4  simics-training-6.0.pre30
```

When setting up the meson build, you will use the path to this directory.

## Build

This project uses the meson build system. Using the path to your simics installation,
you can build the project like so (see below for notes *first*, which you may need to 
successfully build the project):

```sh
$ meson setup builddir -Dsimics_home=/home/rhart/simics/simics/
$ meson compile -C builddir
```

### Build Notes

*NOTE*: If you installed llvm by specifying `llvm-14` instead of simply `llvm` (which
is the method you should most likely use), you will need to run define the 
`LLVM_CONFIG` variable when running `meson` like:

```sh
$ LLVM_CONFIG=$(which llvm-config-14) meson setup builddir
$ LLVM_CONFIG=$(which llvm-config-14) meson compile -C builddir
```

*NOTE*: On most modern distributions of Linux, `python` is no longer an alias to 
`python2`. If your system does not run `python2` when `python` is used, you need to
define the environment variable `PYTHON_COMMAND` when running `meson` like:


```sh
$ PYTHON_COMMAND=python2 meson setup builddir
$ PYTHON_COMMAND=python2 meson compile -C builddir

```