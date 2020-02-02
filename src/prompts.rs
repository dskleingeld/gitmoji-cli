//! Collection of all used prompts
use dialoguer::{Select, Input, Confirmation};
use enquirer::ColoredTheme;
use json::JsonValue;
use std::io;
use crate::configuration::EmojiFormat;

/// Struct for emoji data
#[derive(Clone, Debug)]
pub struct Emoji {
    pub code: String,
    description: String,
    pub emoji: String,
    name: String
}

impl ToString for Emoji {
    fn to_string(&self) -> String {
        format!("{} - {}", self.emoji, self.description)
    }
}

impl From<&JsonValue> for Emoji {
    fn from(val: &JsonValue) -> Self {
        Emoji {
            code: val["code"].to_string(),
            description: val["description"].to_string(),
            emoji: val["emoji"].to_string(),
            name: val["name"].to_string()
        }
    }
}

/// Asks what emoji should be used for a commit
pub fn ask_for_emoji(emojis: &Vec<Emoji>) -> Result<&Emoji, io::Error> {
    let theme = ColoredTheme::default();
    let mut select = Select::with_theme(&theme);
    select.with_prompt("Choose a gitmoji:");
    select.items(emojis);
    select.paged(true);
    select.default(0);

    // TODO autocomplete

    let res = select.interact()?;
    Ok(emojis.get(res).expect("Should be in list"))
}

/// Asks for scope of commit
pub fn ask_for_scope() -> Result<String, io::Error> {
    let theme = ColoredTheme::default();
    let mut input = Input::with_theme(&theme);
    input.with_prompt("Enter the scope of current changes:");
    input.validate_with(|v: &str| {
        if v.contains("`") {
            Err("Enter a valid scope")
        } else {
            Ok(())
        }
    });

    input.interact()
}

/// Asks for title of commit
pub fn ask_for_title() -> Result<String, io::Error> {
    let theme = ColoredTheme::default();
    let mut input = Input::with_theme(&theme);
    input.with_prompt("Enter the commit title:");
    input.validate_with(|v: &str| {
        if v.contains("`") || v.is_empty() {
            Err("Enter a valid title")
        } else {
            Ok(())
        }
    });

    input.interact()
}

/// Asks for commit message
pub fn ask_for_message() -> Result<String, io::Error> {
    let theme = ColoredTheme::default();
    let mut input = Input::with_theme(&theme);
    input.with_prompt("Enter the commit message:");
    input.validate_with(|v: &str| {
        if v.contains("`") {
            Err("Enter a valid message")
        } else {
            Ok(())
        }
    });

    input.interact()
}

/// Asks for referred issue
pub fn ask_for_issue() -> Result<String, io::Error> {
    let theme = ColoredTheme::default();
    let mut input = Input::with_theme(&theme);
    input.with_prompt("Enter the referring issue:");
    input.validate_with(|v: &str| {
        if v.contains("`") || v.is_empty() {
            Err("Enter a valid issue")
        } else {
            Ok(())
        }
    });

    input.interact()
}

/// Configure prompt for automatic commit
pub fn config_for_auto_add(default: bool) -> Result<bool, io::Error> {
    let theme = ColoredTheme::default();
    let mut confirm = Confirmation::with_theme(&theme);
    confirm.with_text("Enable automatic \"git add .\":");
    confirm.default(default);

    confirm.interact()
}

/// Configure prompt for signed commits
pub fn config_for_signed_commit(default: bool) -> Result<bool, io::Error> {
    let theme = ColoredTheme::default();
    let mut confirm = Confirmation::with_theme(&theme);
    confirm.with_text("Enable signed commits:");
    confirm.default(default);

    confirm.interact()
}

/// Configure prompt for the scope prompts
pub fn config_for_scope_prompt(default: bool) -> Result<bool, io::Error> {
    let theme = ColoredTheme::default();
    let mut confirm = Confirmation::with_theme(&theme);
    confirm.with_text("Enable scope prompt:");
    confirm.default(default);

    confirm.interact()
}

/// Configure prompt for the issue prompt
pub fn config_for_issue_prompt(default: bool) -> Result<bool, io::Error> {
    let theme = ColoredTheme::default();
    let mut confirm = Confirmation::with_theme(&theme);
    confirm.with_text("Enable referring issue prompt:");
    confirm.default(default);

    confirm.interact()
}

/// Temporary data holder for the emoji selection prompt
struct EmojiFormatSelection {
    emoji_format: EmojiFormat,
    display: String,
}

impl ToString for EmojiFormatSelection {
    fn to_string(&self) -> String {
        format!("{}", self.display)
    }
}

/// Configure prompt for emoji format
pub fn config_for_emoji_format(default: EmojiFormat) -> Result<EmojiFormat, io::Error> {
    let theme = ColoredTheme::default();
    let mut select = Select::with_theme(&theme);
    select.with_prompt("Select how emojis should be used in commits:");
    let code = EmojiFormatSelection {emoji_format: EmojiFormat::CODE, display: ":smile:".to_owned()};
    let emoji = EmojiFormatSelection { emoji_format: EmojiFormat::EMOJI, display: "😄".to_owned()};
    let items = [code, emoji];
    select.items(&items);
    let default_selection = match default {
        EmojiFormat::CODE => 0,
        _ => 1
    };
    select.default(default_selection);

    let selection = select.interact()?;
    let selected_item = items.get(selection).expect("selected item should be in item range");
    match selected_item.emoji_format {
        EmojiFormat::CODE => Ok(EmojiFormat::CODE),
        _ => Ok(EmojiFormat::EMOJI)
    }
}