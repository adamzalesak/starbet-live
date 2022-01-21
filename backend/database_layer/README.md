# Database Layer Documentation

## Folder structure

```txt
.
├── examples_and_testing   # an example binary with repository use cases
│   ├── src
│   │  └── main.rs         # all examples here
│   └── Cargo.toml         # dependencies intended to work together with the library
│
├── migrations             # a migrations folder, used for creating database tables
│   └── ...
├── src                    # an actual source folder for the library
│   └── ...
├── Cargo.toml             # library dependencies
├── diesel.toml            # diesel configuration file
└── README.md              # this readme
```
