
# Kita - Bulk Rename

Kita Rename Utility is a highly customizable renaming utlity written in rust. This project was created to fill a gap on Linux where I couldn't find a solid renaming option that allowed the features this project creates.


[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/)


## Stack

**GUI**: [egui](https://github.com/emilk/egui) - An immediate mode native GUI framework.

**File Hashing** [MD5 / Sha1 / Sha256](https://github.com/RustCrypto/hashes) - Collection of cryptographic hash functions written in pure Rust



## Features

- ✓ Cross platform (Linux [x64, aarch64], Windows [x64])
- ✓ Presets
- CLI:
  - ✓ `-o <path>` Open Directory (use quotes `""` if you have spaces)
  - ✓ `-op <path> <preset>` Open Directory and Preset (use quotes `""` if you have spaces)
  -   `-ap <path> <preset>` Apply Preset to a Directory (use quotes `""` if you have spaces)
- Context Menus:
  -   Linux ( Dolphin / Thunar support coming )
  - ✓ Windows File Explorer
- Modifiers:
  - ✓ Add
  - ✓ Case
  - ✓ Date [Presets / User Defined]
  - ✓ File Extension
  - ✓ Hash [CRC32, MD5, Sha1, Sha256] [Endianness]
  - ✓ Move/Copy
  - ✓ Name
  - ✓ Numbering
  - ✓ Regex

✓ = Feature Implemented

For a more detailed list of features, check the docs!

## Building

To Build this project on run

**Linux**
```bash
  git clone https://github.com/Auvrae/Kita
  cd Kita
  mkdir dependencies
  cd dependencies
  git clone https://github.com/Auvrae/rust-utils
  git clone https://github.com/emilk/egui
  cd ..
  cargo build -r
  mv ./target/release/kita ./Kita
  rm -r ./target && rm -r dependencies
```

**Windows**
```bat
   TBD
```

Or run the `build.sh` or `build.bat` 

!! **Always read scripts you download from the internet before running them** !!

If you don't feel like building yourself you can download the latest version [here](https://github.com/Auvrae/Kita/releases).
## Upcoming Features

- Right-click open menus!
- build.sh and build.bat
- Docs


## Known Issues

**Files in the root of a drive on Windows don't get listed**
-  This program was developed solely for Linux at first. Therefore it hadn't occured to me that a user would want to rename files at the root of an drive. On Linux everything is mounted in at least one folder.. This may or may not *ever* get fixed, unless there's user demand for it.

## Lessons Learned

Being a high level language programmer, my first language was JS (NodeJS -> TypeScript), Rust has taught me a lot about how computers actually do the work under the hood. It's been very fun. On my journy to learn Rust, I wanted to create a meaningful real-world application. This is my first "Full Stack" application, using as many elements from Rust as I could. For anyone interested in learning Rust I suggest giving it a shot! It's been a blast. 


## FAQ

#### Why egui and not xyz?

I wanted to use something new to give the UI a fresh look instead of going with conventional frameworks like GKT or QT.
## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's `code of conduct`.


## Special Thanks

[CheatFreak](https://github.com/cheatfreak47/) - UX Design help / Beta testing

[Winter](https://github.com/winterkid09/) - Beta testing

[Readme Editor](https://readme.so/editor) - Creating this document easily
