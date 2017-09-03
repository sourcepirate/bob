## Bob the builder

Builds linux distro os binaries from source files.

## Configuration file

Bob works on simple json file which defines how to build source depending on language.


```

{
  "packages": [
    {"name": "pylocated",
     "url": "https://pypi.python.org/packages/9e/c7/81cdae3cb574beb2e2f5f0b1019c752b45d9c7a50ad1a414a5e900696125/pylocated-2.0.1.tar.gz",
     "language": "python"},
    {"name": "pyrequest",
     "url": "https://pypi.python.org/packages/6f/94/5d2dabd104000530fddc8de92cf731a9706642d3879e28c09c27ae2b8758/pyrequest-0.5.tar.gz",
     "language": "python"}
  ]
}

```


## Cli Options

```

Bob the builder 0.5
Build rpms and debs with ease

USAGE:
    bob [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --binary <rpm>                                         Type of binary to build
    -f, --config <config.json>                                 Config file for building rpm
    -o, --out </Users/plasmashadow/github/bob/target/debug>    target directory for rpms
    -t, --temp </tmp>                                          Temporary directory for rpms


```

## RoadMap

* More language support (Currently support's only ruby and python)
* Better design.

## License
MIT