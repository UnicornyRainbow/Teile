// Copyright 2023 UnicornyRainbow
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![crate_name = "teile"]

use std::fmt;

/// This contains CSS related functions
pub trait Css {
    // TODO implement a default function, this should be doable...
    /// This can be implemented to convert an object to a CSS variable.
    /// It should return something in the form of `var(--some-variable)`
    fn to_css_var(&self) -> String;
}

/// All the default Adwaita colors that can / should be used by a programmer for specific tasks
#[allow(dead_code)]
pub enum Colors {
    /// Accent Color, ie submit button
    AccentColor,
    /// Accent Color Background, ie submit button
    AccentBgColor,
    /// Accent Color Foreground, ie submit button
    AccentFgColor,
    /// Destructive Color, ie delete button
    DestructiveColor,
    /// Destructive Color Background, ie delete button
    DestructiveBgColor,
    /// Destructive Color Foreground, ie delete button
    DestructiveFgColor,
    /// Success Color, ie successfully saved message
    SuccessColor,
    /// Success Color Background, ie successfully saved message
    SuccessBgColor,
    /// Success Color Foreground, ie successfully saved message
    SuccessFgColor,
    /// Warning Color, ie connection problems warning
    WarningColor,
    /// Warning Color Background, ie connection problems warning
    WarningBgColor,
    /// Warning Color Foreground, ie connection problems warning
    WarningFgColor,
    /// Error Color, ie save failed message
    ErrorColor,
    /// Error Color Background, ie save failed message
    ErrorBgColor,
    /// Error Color Foreground, ie save failed message
    ErrorFgColor,
}

impl fmt::Display for Colors {
    /// Converts a `Color` to a CSS compliant string
    ///
    /// ```
    /// use teile::Colors;
    ///
    /// let css_name: String = Colors::AccentColor.to_string();
    /// assert_eq!(css_name, "accent-color");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Colors::AccentColor => write!(f, "accent-color"),
            Colors::AccentBgColor => write!(f, "accent-bg-color"),
            Colors::AccentFgColor => write!(f, "accent-fg-color"),
            Colors::DestructiveColor => write!(f, "destructive-color"),
            Colors::DestructiveBgColor => write!(f, "destructive-bg-color"),
            Colors::DestructiveFgColor => write!(f, "destructive-fg-color"),
            Colors::SuccessColor => write!(f, "success-color"),
            Colors::SuccessBgColor => write!(f, "success-bg-color"),
            Colors::SuccessFgColor => write!(f, "success-fg-color"),
            Colors::WarningColor => write!(f, "warning-color"),
            Colors::WarningBgColor => write!(f, "warning-bg-color"),
            Colors::WarningFgColor => write!(f, "warning-fg-color"),
            Colors::ErrorColor => write!(f, "error-color"),
            Colors::ErrorBgColor => write!(f, "error-bg-color"),
            Colors::ErrorFgColor => write!(f, "error-fg-color"),
        }
    }
}

impl Css for Colors {
    /// Converts a `Color` to a CSS variable
    ///
    /// ```
    /// use teile::{Colors, Css};
    ///
    /// let css_var: String = Colors::AccentColor.to_css_var();
    /// assert_eq!(css_var, "var(--accent-color)");
    /// ```
    fn to_css_var(&self) -> String {
        format!("var(--{})", &self.to_string())
    }
}

/// Equivalent to a normal HTML button with some styling
#[allow(dead_code)]
pub struct Button {
    /// - Use `Option::None` to use the default color.
    /// - Use `Option::Some()` to set a certain color and give the button a certain visual purpose
    ///
    /// `bg_color` takes precedence over 'flat'
    pub bg_color: Option<Colors>,
    /// If `true` applies the flat style, meaning the button has no background color
    ///
    /// If `bg_color` is set to `Option::Some()`, this setting is ignored
    pub flat: bool,
    /// Converts the button to a toggle button
    ///
    /// **This is only for styling, you should handle the toggling on your own!**
    pub toggled: bool,
    /// The ID of the `Button`, because of the structure how `Button` is build, it will be converted in an `id-inner` and `id-outer`
    pub id: Option<String>,
    /// Whatever child you want the button to have, usually some text
    pub child: String,
}

impl Button {
    /// This renders the `Button` as an HTML formatted String
    /// The `<div class="input-bg></div>` is used to change the actual background color and
    /// the tinting on events like hover or click can be changed independently
    /// ```
    /// use teile::{Colors, Css, Button};
    ///
    /// let rendered_button: String = Button{
    ///     bg_color: Option::None,
    ///     flat: true,
    ///     toggled: true,
    ///     id: Option::Some("example".to_string()),
    ///     child: "Example".to_string(),
    /// }.render();
    /// let rendered_button_result = "\
    /// <div id=\"example-outer\" class=\"input-bg flat\" >
    ///     <button id=\"example-inner\" class=\"toggled\">Example</button>
    /// </div>".to_string();
    /// assert_eq!(rendered_button, rendered_button_result);
    ///
    /// let rendered_button_color: String = Button{
    ///     bg_color: Option::Some(Colors::AccentColor),
    ///     flat: false,
    ///     toggled: false,
    ///     id: Option::None,
    ///     child: "Example".to_string(),
    /// }.render();
    /// let rendered_button_color_result = "\
    /// <div  class=\"input-bg \" style=\"background=var(--accent-color)\">
    ///     <button  class=\"\">Example</button>
    /// </div>".to_string();
    /// assert_eq!(rendered_button_color, rendered_button_color_result);
    /// ```
    #[allow(dead_code)]
    pub fn render(&self) -> String {
        let bg_color: String = match &self.bg_color {
            None => {"".to_string()}
            Some(color) => {format!("style=\"background={}\"", color.to_css_var())}
        };
        let flat: &str = match &self.flat {
            true => {"flat"}
            false => {""}
        };
        let toggled: &str = match &self.toggled {
            true => {"toggled"}
            false => {""}
        };
        let (id_outer, id_inner): (String, String) = match &self.id {
            None => {("".to_string(), "".to_string())}
            Some(id) => {(format!("id=\"{}-outer\"", id.to_string()), format!("id=\"{}-inner\"", id.to_string()))}
        };
        let child: &String = &self.child;
        format!("\
<div {id_outer} class=\"input-bg {flat}\" {bg_color}>
    <button {id_inner} class=\"{toggled}\">{child}</button>
</div>").to_string()
    }
}