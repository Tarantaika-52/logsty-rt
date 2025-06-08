use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{stdout, Stdout, Write};
use std::sync::{Arc, Mutex};
use crate::logger::types::Color;

impl Display for LogLevel{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            LogLevel::DEBUG => "DEBUG",
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
            LogLevel::ERROR => "ERROR",
            LogLevel::FATAL => "FATAL",
        };

        write!(f, "{as_str}")
    }
}
#[derive(Default)]
pub enum LogLevel {
    #[default]
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

impl Display for Logger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let m_name = &self.module_name;
        let fmt = &self.format;
        let lvl = &self.log_level;
        let u_c = &self.use_colors;

        write!(
            f,
            "Logger for {m_name}\n> Level: {lvl}\n> Format: \"{fmt:}\"\n> UseColors: {u_c}"
        )
    }
}

impl Logger {
    pub fn new() -> Self{
        let out = Arc::new(Mutex::from(stdout()));
        Logger{
            module_name: String::from("UNDEFINED"),
            log_level: LogLevel::DEBUG,
            format: String::from(
                "%style:accent;bold%>> [ %L% ]%style:reset% > %T%%style:dark% (%Mod%) -%style:reset% %Msg%"
            ),
            use_colors: true,
            out
        }
    }

    pub fn with_module_name(mut self, name: &str) -> Self {
        self.module_name = name.to_string();
        self
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }

    pub fn with_format(mut self, format: &str) -> Self {
        self.format = format.to_string();
        self
    }

    pub fn with_colors(mut self, with_colors: bool) -> Self {
        self.use_colors = with_colors;
        self
    }

    fn parse_style(accent_color: Color, style_keys: Vec<&str>) -> String {
        let accent_clr = format!("3{}", accent_color);

        let mut style = String::from("\x1b[");

        for key in style_keys {
            if !style.ends_with("[") {
                style.push_str(";");
            }
            let style_key = match key {
                "accent" => &accent_clr,
                "reset" => "0",
                "dark" => "90",
                "bold" => "1",
                "italic" => "3",
                "under" => "4",
                _ => "0"
            };

            style.push_str(style_key);
        }
        style.push_str("m");
        style
    }

    fn parse_format(
        fmt: &str,
        accent_color: Color,
        vars: &HashMap<&str, &str>,
        use_colors: bool
    ) -> String {
        let mut parsed = String::new();

        for token in fmt.split("%") {
            if token.starts_with("style:") {
                let style_keys = &token[6..];

                let keys: Vec<&str> = style_keys.split(";").collect();
                let code = Self::parse_style(accent_color, keys);

                if use_colors {
                    parsed.push_str(code.as_str());
                }
            }
            else if vars.contains_key(token) {
                let data = vars.get(token).unwrap();
                parsed.push_str(data);
            }
            else {
                parsed.push_str(token);
            }
        }

        format!("{parsed}\n")
    }

    fn get_timestamp() -> String {
        chrono::Local::now().format("%H:%M:%S").to_string()
    }

    fn log(
        &self,
        color: Color,
        level: LogLevel,
        msg: &str,
    ) {
        let fmt = &self.format;
        let time = Self::get_timestamp();
        let module = &self.module_name;
        let level = format!("{:<5}", format!("{level}"));

        let vars: HashMap<&str, &str> = HashMap::from([
            ("L", level.as_str()),
            ("Mod", module.as_str()),
            ("T", time.as_str()),
            ("Msg", msg)
        ]);

        let parsed = Self::parse_format(
            &fmt.as_str(),
            color,
            &vars,
            self.use_colors
        );


        match self.out.lock() {
            Ok(mut out) => {
                if let Err(e) = write!(out, "{parsed}") {
                    eprintln!("{e}");
                }
            }
            Err(_) => {
                println!("{parsed}");
            }
        }
    }

    pub fn debug(&self, message: &str) {
        self.log(Color::Purple,LogLevel::DEBUG,message)
    }

    pub fn info(&self, message: &str) {
        self.log(Color::Green, LogLevel::INFO, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(Color::Yellow, LogLevel::WARN, message);
    }

    pub fn error(&self, message: &str) {
        self.log(Color::Red, LogLevel::ERROR, message);
    }

    pub fn fatal(&self, message: &str) {
        self.log(Color::Fatal, LogLevel::FATAL, message);
    }
}

pub struct Logger {
    module_name: String,
    log_level: LogLevel,
    format: String,
    use_colors: bool,
    out: Arc<Mutex<Stdout>>
}