#[cfg(feature = "color")]
pub use termcolor::Color;

/** Configuration for the level colors

## Default mapping
| Level | Color                                               |
| --    | --                                                  |
| Error | [`Color::Red`](./enum.Color.html#variant.Red)       |
| Warn  | [`Color::Yellow`](./enum.Color.html#variant.Yellow) |
| Info  | [`Color::Green`](./enum.Color.html#variant.Green)   |
| Debug | [`Color::Cyan`](./enum.Color.html#variant.Cyan)     |
| Trace | [`Color::Blue`](./enum.Color.html#variant.Blue)     |
*/
#[cfg(feature = "color")]
#[derive(Debug, Copy, Clone)]
pub struct LevelColorConfig {
    pub error: Color,
    pub warn: Color,
    pub info: Color,
    pub debug: Color,
    pub trace: Color,
}

#[cfg(feature = "color")]
impl Default for LevelColorConfig {
    fn default() -> Self {
        Self {
            error: Color::Red,
            warn: Color::Yellow,
            info: Color::Green,
            debug: Color::Cyan,
            trace: Color::Blue,
        }
    }
}

/** Configuration for each part of a record

## Default mapping
| Field        | Color                                                      | RGB       |
| --           | --                                                         | --        |
| level        | See [`LevelColorConfig`](./struct.LevelColorConfig.html)   | --        |
| target       | [`Color::Ansi256(131)`](./enum.Color.html#variant.Ansi256) | `#AF5F5F` |
| timestamp    | [`Color::Ansi256(243)`](./enum.Color.html#variant.Ansi256) | `#767676` |
| continuation | [`Color::Ansi256(237)`](./enum.Color.html#variant.Ansi256) | `#3A3A3A` |
| message      | [`Color::Ansi256(231)`](./enum.Color.html#variant.Ansi256) | `#FFFFFF` |
*/
#[cfg(feature = "color")]
#[derive(Debug, Copy, Clone)]
pub struct RecordColorConfig {
    pub level: LevelColorConfig,
    pub target: Color,
    pub timestamp: Color,
    pub continuation: Color,
    pub message: Color,
}

#[cfg(feature = "color")]
impl Default for RecordColorConfig {
    fn default() -> Self {
        Self {
            target: Color::Ansi256(131),
            timestamp: Color::Ansi256(243),
            continuation: Color::Ansi256(237),
            message: Color::Ansi256(231),
            level: LevelColorConfig::default(),
        }
    }
}

#[cfg(not(feature = "color"))]
#[derive(Debug, Copy, Clone, Default)]
/// Empty color config when the _feature_ `color` is disabled
pub struct RecordColorConfig {}

#[cfg(not(feature = "color"))]
#[derive(Debug, Copy, Clone, Default)]
/// Empty color config when the _feature_ `color` is disabled
pub struct LevelColorConfig {}
