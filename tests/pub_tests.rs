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

#[cfg(test)]
mod tests {
    use teile::{Colors, Css, Button};

    #[test]
    fn colors_to_string() {
        let result = Colors::AccentColor.to_string();
        assert_eq!(result, "accent-color");
    }

    #[test]
    fn colors_to_css_var() {
        let result = Colors::AccentColor.to_css_var();
        assert_eq!(result, "var(--accent-color)");
    }

    #[test]
    fn render_button_default() {
        let rendered_button: String = Button{
            bg_color: Option::None,
            flat: false,
            toggled: false,
            id: Option::None,
            additional_args: Option::None,
            child: "".to_string(),
        }.render();
        let rendered_button_result = "\
<div  class=\"input-bg \" >
    <button  class=\"\" ></button>
</div>".to_string();
        assert_eq!(rendered_button, rendered_button_result);
    }

    #[test]
    fn render_button_all() {
        let rendered_button: String = Button{
            bg_color: Option::Some(Colors::AccentColor),
            flat: true,
            toggled: true,
            id: Option::Some("example".to_string()),
            additional_args: Option::Some(vec!["hx-post=\"/clicked\"".to_string(), "hx-swap=\"outerHTML\"".to_string()]),
            child: "Example".to_string(),
        }.render();
        let rendered_button_result = "\
<div id=\"example-outer\" class=\"input-bg flat\" style=\"background=var(--accent-color)\">
    <button id=\"example-inner\" class=\"toggled\" hx-post=\"/clicked\" hx-swap=\"outerHTML\">Example</button>
</div>".to_string();
        assert_eq!(rendered_button, rendered_button_result);
    }
}