// element.rs

const BG_COLOR: &'static str = "#30333f";
const FG_COLOR: &'static str = "#ffffff";
const WARNING_FG_COLOR: &'static str = "#e0d060";
const ERROR_FG_COLOR: &'static str = "#d05040";
const INFO_FG_COLOR: &'static str = "#70a0ff";
const SEP_COLOR: &'static str = "#22242c";
const BLUE_COLOR: &'static str = "#5294e2";
const BLACK_COLOR: &'static str = "#000000";

pub enum ElementFormat {
    Normal,
    Info,
    Warning,
    Error,
    Separator,
    Time
}

pub enum PangoFormat {
    FontStyle(PangoFontStyle),
    FontWeight(PangoFontWeight),
    FontSize(PangoFontSize),
    Foreground(String),
    Background(String),
    BackgroundAlpha(String),
}

pub enum PangoFontStyle { Normal, Oblique, Italic }
pub enum PangoFontWeight { UltraLight, Light, Normal, Bold, UltraBold, Heavy }
pub enum PangoFontSize { XXSmall, XSmall, Small, Medium, Large, XLarge, XXLarge }

pub struct Element<'a> {
    name: Option<&'a str>,
    instance: Option<&'a str>,
    pub text: String,
    pub format: ElementFormat,
    use_pango: bool,
}

impl<'a> Element<'a> {

    pub fn new(name: &'a str, instance: &'a str, text: String, format: ElementFormat) -> Element<'a> {
        Element { name: Some(name),
                  instance: Some(instance),
                  text: text,
                  format: format,
                  use_pango: false }
    }

    pub fn sep() -> Element<'a> {
        Element { name: None,
                  instance: None,
                  text: String::from(" "),
                  format: ElementFormat::Separator,
                  use_pango: false }
    }

    pub fn space() -> Element<'a> {
        Element { name: None,
                  instance: None,
                  text: String::from(" "),
                  format: ElementFormat::Normal,
                  use_pango: false }
    }

    pub fn space_time() -> Element<'a> {
        Element { name: None,
                  instance: None,
                  text: String::from(" "),
                  format: ElementFormat::Time,
                  use_pango: false }
    }

    pub fn show(&self) {
        print!("{{");
        if self.name.is_some() {
            print!("\"name\": \"{}\", ", self.name.unwrap());
        }
        if self.instance.is_some() {
            print!("\"instance\": \"{0}\", ", self.instance.unwrap());
        }
        print!("\"full_text\": \"{}\"", self.text);
        match self.format {
            ElementFormat::Normal    => print!(", \"background\": \"{}\"", BG_COLOR),
            ElementFormat::Info      => print!(", \"color\": \"{}\", \"background\": \"{}\"", INFO_FG_COLOR, BG_COLOR),
            ElementFormat::Warning   => print!(", \"color\": \"{}\", \"background\": \"{}\"", WARNING_FG_COLOR, BG_COLOR),
            ElementFormat::Error     => print!(", \"color\": \"{}\", \"background\": \"{}\"", ERROR_FG_COLOR, BG_COLOR),
            //ElementFormat::Separator => print!(", \"color\": \"{}\", \"background\": \"{}\"", SEP_COLOR, SEP_COLOR),
            ElementFormat::Separator => {},
            ElementFormat::Time      => print!(", \"color\": \"{}\", \"background\": \"{}\"", BLACK_COLOR, BLUE_COLOR),
        }
        //print!(", \"separator\": false");
        print!(", \"separator_block_width\": 0");
        if self.use_pango {
            print!(", \"markup\": \"pango\"");
        }
        print!("}}");
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.text = text.into();
        self.use_pango = false;
    }

    pub fn clear_text(&mut self) {
        self.text.clear();
        self.use_pango = false;
    }

    pub fn append_text(&mut self, text: &str) {
        self.text.push_str(text.into());
    }

    pub fn append_pango(&mut self, text: &str, format: Vec<PangoFormat>) {
        self.text.reserve(13 + 20*format.len());
        self.use_pango = true;
        self.text.push_str("<span");
        for attribute in format {
            match attribute {
                PangoFormat::Background(color) => self.text.push_str(&format!(" background=\'{}\'", color)),
                PangoFormat::BackgroundAlpha(level) => self.text.push_str(&format!(" background_alpha=\'{}\'", level)),
                PangoFormat::FontSize(PangoFontSize::XXSmall) => self.text.push_str(" font_size=\'xx-small\'"),
                PangoFormat::FontSize(PangoFontSize::XSmall) => self.text.push_str(" font_size=\'x-small\'"),
                PangoFormat::FontSize(PangoFontSize::Small) => self.text.push_str(" font_size=\'small\'"),
                PangoFormat::FontSize(PangoFontSize::Medium) => self.text.push_str(" font_size=\'medium\'"),
                PangoFormat::FontSize(PangoFontSize::Large) => self.text.push_str(" font_size=\'large\'"),
                PangoFormat::FontSize(PangoFontSize::XLarge) => self.text.push_str(" font_size=\'x-large\'"),
                PangoFormat::FontSize(PangoFontSize::XXLarge) => self.text.push_str(" font_size=\'xx-large\'"),
                PangoFormat::FontStyle(PangoFontStyle::Normal) => self.text.push_str(" font_style=\'normal\'"),
                PangoFormat::FontStyle(PangoFontStyle::Oblique) => self.text.push_str(" font_style=\'oblique\'"),
                PangoFormat::FontStyle(PangoFontStyle::Italic) => self.text.push_str(" font_style=\'italic\'"),
                PangoFormat::FontWeight(PangoFontWeight::UltraLight) => self.text.push_str(" font_weight=\'ultralight\'"),
                PangoFormat::FontWeight(PangoFontWeight::Light) => self.text.push_str(" font_weight=\'light\'"),
                PangoFormat::FontWeight(PangoFontWeight::Normal) => self.text.push_str(" font_weight=\'normal\'"),
                PangoFormat::FontWeight(PangoFontWeight::Bold) => self.text.push_str(" font_weight=\'bold\'"),
                PangoFormat::FontWeight(PangoFontWeight::UltraBold) => self.text.push_str(" font_weight=\'ultrabold\'"),
                PangoFormat::FontWeight(PangoFontWeight::Heavy) => self.text.push_str(" font_weight=\'heavy\'"),
                PangoFormat::Foreground(color) => self.text.push_str(&format!(" background=\'{}\'", color)),
            }
        }
        self.text.push('>');
        self.text.push_str(text.into());
        self.text.push_str("</span>");
    }

    pub fn set_format(&mut self, format: ElementFormat) {
        self.format = format;
    }
}

