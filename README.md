
# Kita - Bulk Rename

Kita Rename Utility is a highly customizable renaming utlity written in rust. This project was created to fill a gap on Linux where I couldn't find a solid renaming option that allowed the features this project creates.


[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/)


## Stack

**GUI**: [egui](https://github.com/emilk/egui) - An immediate mode native GUI framework.

**File Hashing** [MD5 / Sha1 / Sha256](https://github.com/RustCrypto/hashes) - Collection of cryptographic hash functions written in pure Rust

**Config Files** [JSON](https://github.com/maciejhirsz/json-rust) - JSON implementation in Rust 







## Features

- Cross platform (Linux [x64, aarch64], Windows [x64])
- Highly Customizable:
  - ✓ Reordering Modifiers [Also changes their priorty] 
  - ✓ Dark mode / Light mode
  - ✓ Scaling for high DPI monitors
  - Low power mode [Reduces framerate and thread count for better efficiency]
- Presets:
  - Save / Load (In Modifiers panel, or in the Preset Manager)
  - Preset Manager:
    - Create / Edit / Remove Presets
- CLI:
  - Open target directory directly with launch options (X11 / Wayland compatible via [xdg-desktop-portals](https://github.com/flatpak/xdg-desktop-portal))
  - Apply Presets to target directory with preview and confirmation (optional)
- Multi-Threading
- Modifiers:
  - ✓ Add:
    - Prefix / Suffix / Insert at [String]
    - Customizable Seperator [Char]
  - ✓ Case:
    - Upper / Lower / Title / UpperFirst
    - Except [From -> To]
  - ✓ Date [Presets / User Defined]
    - Prefix / Suffic / Insert at [Index]
    - Preset Formats:
      - DMY / YMD / MDY / ISO....
      - Customizable Seperator [Char] (date)
      - Customizable Segregator [Char] day:month:year
  - ✓ File Extension
    - Upper / Lower / UpperFirst / Fixed (Replace [String]) / Extra (Append [String] / Remove
  - ✓ Hash [CRC32, MD5, Sha1, Sha256] [Endianness]
  - ✓ Move/Copy
    - Copy First / Last N [Chars]
    - Move First / Last N [Chars]
    - To (Start / End / at Index)
  - ✓ Name
    - Remove 
    - Fixed (Remove and Replace with [String])
    - Reverse (Reverses () {} [] <> too!!)
  - ✓ Numbering
    - Prefix / Suffix / Insert / Prefix & Suffix
    - Modes:
      - BaseTwo (Binary)
      - BaseEight (Octal)
      - BaseTen
      - BaseSixteen (Hex)
      - Roman Numeral
      - Alpha Lower
      - Alpha Upper
      - Alpha Lower to Upper
    - Customizable Seperator
    - Padding
  - ✓ Regex

✓ = Feature Implemented



## Upcoming Features

- CLI Agruments
- Docs
- Linux / Windows Context menu item
- TUI ? (maybe..)



## Lessons Learned

Being a high level language programmer, my first language was JS (NodeJS -> TypeScript), Rust has taught me a lot about how computers actually do the work under the hood. It's been very fun. On my journy to learn Rust, I wanted to create a meaningful real-world application. This is my first "Full Stack" application, using as many elements from Rust as I could. For anyone interested in learning Rust I suggest giving it a shot! It's been a blast. 


## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's `code of conduct`.


## Special Thanks

CheatFreak - UX Design help / Beta testing

Winter - Beta testing

[Readme Editor](https://readme.so/editor) - Creating this document easily
