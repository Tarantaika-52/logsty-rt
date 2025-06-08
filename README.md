### logsty-rs
A lightweight and customizable rust logger

#### 📦 Installation

---
Add this to your `Cargo.toml`:
```toml
logsty-rt = { git = "https://github.com/Tarantaika-52/logsty-rt" }
```

#### 🚀 Usage

---
```rust
use logsty_rt::{Logger};

fn main() {
    let logger = Logger::new()
        .with_module_name("APP");
    
    logger.debug("Hello world!");
}
```