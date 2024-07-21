use serde::{Deserialize, Serialize};
use super::util::threads::{HashMode, HashType, Endianness};

// Modifiers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Modifiers {
    pub add: Vec<ModAdd>,
    pub case: Vec<ModCase>,
    pub date: Vec<ModDate>,
    pub extension: ModExtension,
    pub hash: ModHashing,
    pub movecopy: Vec<ModMoveCopy>,
    pub name: Vec<ModName>,
    pub number: Vec<ModNumber>,
    pub regex: Vec<ModRegex>,
    pub remove: Vec<ModRemove>,
    pub replace: Vec<ModReplace>,
    pub add_enabled: bool,
    pub append_folder_enabled: bool,
    pub case_enabled: bool,
    pub date_enabled: bool,
    pub extension_enabled: bool,
    pub hash_enable: bool,
    pub movecopy_enabled: bool,
    pub name_enabled: bool,
    pub number_enabled: bool,
    pub regex_enabled: bool,
    pub remove_enabled: bool,
    pub replace_enabled: bool,
    pub allow_frame: bool,
    pub scroll_allowed: bool,
    pub drag_box_hovered: bool
}

// Generic Enum for ordering in GUI
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum ModsOrder {
    Add,
    Case,
    Date,
    Ext,
    Hash,
    MoveCopy,
    Name,
    Number,
    Regex,
    Remove,
    Replace
}

impl ModsOrder {
    pub fn iterate_over_oneness() -> impl Iterator<Item = ModsOrder> {
        [ModsOrder::Add, ModsOrder::Case, ModsOrder::Date, ModsOrder::Ext, 
            ModsOrder::Hash, ModsOrder::MoveCopy, ModsOrder::Name, ModsOrder::Number, 
            ModsOrder::Regex, ModsOrder::Remove, ModsOrder::Replace].iter().copied()
    }
}
impl Default for Modifiers {
    fn default() -> Self {
        Self {
            add: vec![ModAdd::default()],
            case: vec![ModCase::default()],
            date: vec![ModDate::default()],
            extension: ModExtension::default(),
            hash: ModHashing::default(),
            movecopy: vec![ModMoveCopy::default()],
            name: vec![ModName::default()],
            number: vec![ModNumber::default()],
            regex: vec![ModRegex::default()],
            remove: vec![ModRemove::default()],
            replace: vec![ModReplace::default()],
            add_enabled: true,
            append_folder_enabled: true,
            case_enabled: true,
            date_enabled: true,
            extension_enabled: true,
            hash_enable: true,
            movecopy_enabled: true,
            name_enabled: true,
            number_enabled: true,
            regex_enabled: true,
            remove_enabled: true,
            replace_enabled: true,
            allow_frame: true,
            scroll_allowed: true,
            drag_box_hovered: false
        }
    }
}

// Add
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModAdd {
    pub prefix: String,
    pub insert: String,
    pub insert_at: i32,
    pub suffix: String,
    pub seperator: char,
    pub seperator_enabled: bool,
}
impl Default for ModAdd {
    fn default() -> Self {
        Self {
            prefix: String::new(),
            insert: String::new(),
            insert_at: 0,
            suffix: String::new(),
            seperator: ' ',
            seperator_enabled: false
        }
    }
}

// Case
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModCase{
    pub mode: CaseMode,
    pub widgets_enabled: bool,
    pub mode_name: String,
    pub except: String,
    pub except_mode: CaseExecptMode,
    pub except_enabled: bool,
    pub except_mode_name: String,
    pub except_from: u32,
    pub except_to: u32
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CaseMode {
    Same,
    Upper,
    Lower,
    Title,
    UpperFirst
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CaseExecptMode {
    None,
    FromTo,
    Match
}
impl Default for ModCase {
    fn default() -> Self {
        Self {
            mode: CaseMode::Same,
            widgets_enabled: false,
            mode_name: String::from("Same"),
            except: String::new(),
            except_mode: CaseExecptMode::None,
            except_enabled: false,
            except_mode_name: String::from("None"),
            except_from: 0,
            except_to: 0
        }
    }
}

// Date
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModDate {
    pub mode: DateMode,
    pub mode_name: String,
    pub widgets_enabled: bool,
    pub format: DateFormatMode,
    pub format_name: String,
    pub at_enabled: bool,
    pub at_pos: i32,
    pub seperator: DateSeperator,
    pub seperator_name: String,
    pub seperator_enabled: bool,
    pub segregator_year: DateSeperator,
    pub segregator_year_name: String,
    pub segregator_year_enabled: bool,
    pub segregator_hour: DateSeperator,
    pub segregator_hour_name: String,
    pub segregator_hour_enabled: bool,
    pub custom: String,
    pub custom_enabled: bool,
    pub century: bool
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DateMode {
    None,
    Prefix,
    Suffix,
    Insert
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DateFormatMode {
    Y,
    MY,
    DMY,
    DMYH,
    DMYHM,
    DMYHMS,
    YM,
    YMD,
    YMDH,
    YMDHM,
    YMDHMS,
    MDY,
    MDYH,
    MDYHM,
    MDYHMS,
    Custom
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DateSeperator {
    None,
    Space,
    Bracket,
    Parenthesis,
    CurlyBracket,
    Colan,
    Minus,
    Plus,
    Underscore,
    Sign,
    Line,
    Asterisk
}

impl Default for ModDate {
    fn default() -> Self {
        Self {
            mode: DateMode::None,
            mode_name: String::from("None"),
            widgets_enabled: false,
            format: DateFormatMode::YMDHM,
            format_name: String::from("YMDHM"),
            at_enabled: false,
            at_pos: 0,
            seperator: DateSeperator::Bracket,
            seperator_name: String::from("Bracket []"),
            seperator_enabled: false,
            segregator_hour: DateSeperator::Minus,
            segregator_hour_name: String::from("Period ."),
            segregator_hour_enabled: false,
            segregator_year: DateSeperator::Minus,
            segregator_year_name: String::from("Minus -"),
            segregator_year_enabled: false,
            custom: String::new(),
            custom_enabled: false,
            century: true,
        }
    }
}

// Extension
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModExtension {
    pub mode: ExtensionMode,
    pub mode_name: String,
    pub widgets_enabled: bool,
    pub fixed: String,
    pub extra: String
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExtensionMode {
    Same,
    Upper,
    Lower,
    UpperFirst,
    Fixed,
    Extra,
    Remove
}
impl Default for ModExtension {
    fn default() -> Self {
        Self {
            mode: ExtensionMode::Same,
            mode_name: String::from("Same"),
            widgets_enabled: false,
            fixed: String::new(),
            extra: String::new()
        }
    }
}

// Hashing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModHashing {
    pub mode: HashMode,
    pub mode_name: String,
    pub widgets_enabled: bool,
    pub algorithm: HashType,
    pub algorithm_name: String,
    pub endianness: Endianness,
    pub seperator: HashSeperator,
    pub seperator_name: String
}
impl Default for ModHashing {
    fn default() -> Self {
        Self {
            mode: HashMode::None,
            mode_name: String::from("None"),
            widgets_enabled: false,
            algorithm: HashType::CRC32,
            algorithm_name: String::from("CRC32"),
            endianness: Endianness::BigEndian,
            seperator: HashSeperator::Bracket,
            seperator_name: String::from("Bracket []")
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HashSeperator {
    None,
    Space,
    Bracket,
    Parenthesis,
    CurlyBracket,
    Colan,
    Minus,
    Plus,
    Underscore,
    Sign,
    Line,
    Asterisk
}

// Move / Copy
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModMoveCopy {
    pub mode_from: MoveCopyFromMode,
    pub mode_from_name: String,
    pub widgets_enabled: bool,
    pub letters_count: u32,
    pub mode_to: MoveCopyToMode,
    pub mode_to_name: String,
    pub widgets_enabled_two: bool,
    pub mode_to_pos: u32,
    pub seperator: char,
    pub seperator_enabled: bool
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MoveCopyFromMode {
    None,
    CopyFirstN,
    CopyLastN,
    MoveFirstN,
    MoveLastN
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MoveCopyToMode {
    None,
    ToStart,
    ToEnd,
    ToPos
}
impl Default for ModMoveCopy {
    fn default() -> Self {
        Self {
            mode_from: MoveCopyFromMode::None,
            mode_from_name: String::from("None"),
            widgets_enabled: false,
            letters_count: 0,
            mode_to: MoveCopyToMode::None,
            mode_to_name: String::from("None"),
            widgets_enabled_two: false,
            mode_to_pos: 0,
            seperator: ' ',
            seperator_enabled: false,
        }
    }
}


// Name
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModName {
    pub mode: NameMode,
    pub mode_name: String,
    pub widgets_enabled: bool,
    pub fixed: String
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NameMode {
    Keep,
    Remove,
    Fixed,
    Reverse
}
impl Default for ModName {
    fn default() -> Self {
        Self {
            mode: NameMode::Keep,
            mode_name: String::from("Keep"),
            widgets_enabled: false,
            fixed: String::new()
        }
    }
}

// Numbering
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModNumber {
    pub mode: NumberMode,
    pub mode_name: String,
    pub widgets_enabled: bool,
    pub insert_enabled: bool,
    pub insert_at: i32,
    pub starting_num: u32,
    pub increment_num: u32,
    pub padding: u32,
    pub seperator: char,
    pub seperator_enabled: bool,
    pub mode_type: NumberTypeMode,
    pub mode_type_name: String
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NumberMode {
    None,
    Prefix,
    Suffix,
    Insert,
    PrefixAndSuffix
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NumberTypeMode {
    BaseTwo,
    BaseEight,
    BaseTen,
    BaseSixteen,
    RomanNumeral,
    AlphaLower,
    AlphaUpper,
    AlphaLowerToUpper
}
impl Default for ModNumber {
    fn default() -> Self {
        Self {
            mode: NumberMode::None,
            mode_name: String::from("None"),
            widgets_enabled: false,
            insert_enabled: true,
            insert_at: 0,
            starting_num: 1,
            increment_num: 1,
            padding: 0,
            seperator: ' ',
            seperator_enabled: false,
            mode_type: NumberTypeMode::BaseTen,
            mode_type_name: String::from("Base 10")
        }
    }
}

// Regex
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModRegex {
    pub replace_match: String,
    pub replace_with: String
}

impl Default for ModRegex {
    fn default() -> Self {
        Self {
            replace_match: String::new(),
            replace_with: String::new()
        }
    }
}

// Remove
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModRemove {
    pub first_n: u32,
    pub last_n: u32,
    pub from_x: u32,
    pub to_y: u32,
    pub chars_comma_seperated: String,
    pub words_comma_seperated: String,
    pub crop: RemoveCropMode,
    pub crop_name: String,
    pub crop_match: String,
    pub crop_enabled: bool,
    pub crop_special: String,
    pub digits: bool,
    pub trim: bool,
    pub double_spaces: bool,
    pub accented_chars: bool,
    pub symbols: bool,
    pub leading_dots: bool
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RemoveCropMode {
    None,
    Before,
    After
}
impl Default for ModRemove {
    fn default() -> Self {
        Self {
            first_n: 0,
            last_n: 0,
            from_x: 0,
            to_y: 0,
            chars_comma_seperated: String::new(),
            words_comma_seperated: String::new(),
            crop: RemoveCropMode::None,
            crop_name: String::from("None"),
            crop_match: String::new(),
            crop_enabled: false,
            crop_special: String::new(),
            digits: false,
            trim: false,
            double_spaces: false,
            accented_chars: false,
            symbols: false,
            leading_dots: false
        }
    }
}

// Replace
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModReplace {
    pub replace_match: String,
    pub replace_with: String,
    pub first_occurance: bool
}
impl Default for ModReplace {
    fn default() -> Self {
        Self {
            replace_match: String::new(),
            replace_with: String::new(),
            first_occurance: false
        }
    }
}