//! This module contains a configuration of a [`Border`] or a [`Table`] to set its borders color via [`Color`].
//!
//! [`Border`]: crate::Border

use std::{borrow::Cow, convert::TryFrom};

use papergrid::{records::Records, AnsiColor, Entity};

use crate::{CellOption, Table, TableOption};

/// Color represents a color which can be set to things like [`Border`], [`Padding`] and [`Margin`].
///
/// # Example
///
/// ```
/// use std::convert::TryFrom;
/// use owo_colors::OwoColorize;
/// use tabled::{color::Color, TableIteratorExt};
///
/// let data = [
///     (0u8, "Hello"),
///     (1u8, "World"),
/// ];
///
/// let table = data.table()
///     .with(Color::try_from(" ".red().to_string()).unwrap())
///     .to_string();
///
/// println!("{}", table);
/// ```
///
/// [`Padding`]: crate::Padding
/// [`Margin`]: crate::Margin
/// [`Border`]: crate::Border
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Color(AnsiColor<'static>);

#[rustfmt::skip]
impl Color {
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BLACK:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[30m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BLUE:           Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[34m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_BLACK:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[90m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_BLUE:    Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[94m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_CYAN:    Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[96m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_GREEN:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[92m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_MAGENTA: Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[95m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_RED:     Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[91m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_WHITE:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[97m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_YELLOW:  Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[93m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_CYAN:           Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[36m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_GREEN:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[32m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_MAGENTA:        Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[35m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_RED:            Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[31m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_WHITE:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[37m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_YELLOW:         Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[33m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.

    pub const BG_BLACK:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[40m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BLUE:           Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[44m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_BLACK:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[100m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_BLUE:    Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[104m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_CYAN:    Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[106m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_GREEN:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[102m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_MAGENTA: Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[105m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_RED:     Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[101m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_WHITE:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[107m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_YELLOW:  Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[103m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_CYAN:           Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[46m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_GREEN:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[42m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_MAGENTA:        Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[45m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_RED:            Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[41m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_WHITE:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[47m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_YELLOW:         Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[43m"),  Cow::Borrowed("\u{1b}[49m")));
}

impl Color {
    /// Creates a new [`Color`]` instance, with ANSI prefix and ANSI suffix.
    /// You can use [`TryFrom`] to construct it from [`String`].
    pub fn new(prefix: String, suffix: String) -> Self {
        Self(AnsiColor::new(prefix.into(), suffix.into()))
    }
}

impl From<Color> for AnsiColor<'static> {
    fn from(c: Color) -> Self {
        c.0
    }
}

impl From<AnsiColor<'static>> for Color {
    fn from(c: AnsiColor<'static>) -> Self {
        Self(c)
    }
}

impl TryFrom<&str> for Color {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AnsiColor::try_from(value).map(Color)
    }
}

impl TryFrom<String> for Color {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        AnsiColor::try_from(value).map(Color)
    }
}

impl<R> TableOption<R> for Color {
    fn change(&mut self, table: &mut Table<R>) {
        let color = self.0.clone();
        table.get_config_mut().set_border_color_global(color);
    }
}

impl<R> CellOption<R> for Color
where
    R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let border = border_color(self);

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            table.get_config_mut().set_border_color(pos, border.clone());
        }
    }
}

impl<'b, R> CellOption<R> for &'b Color
where
    R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let border = border_color(self);

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            table.get_config_mut().set_border_color(pos, border.clone());
        }
    }
}

fn border_color(color: &Color) -> papergrid::Border<AnsiColor<'static>> {
    papergrid::Border::full(
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
    )
}
