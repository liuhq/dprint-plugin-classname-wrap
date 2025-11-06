use std::usize;

use dprint_core::formatting::{PrintItems, Signal, ir_helpers};
use dprint_core_macros::sc;

pub struct TailwindWrapperOption {
    pub line_width: u32,
    pub indent_to_quote: bool,
    pub allow_line_overflow: bool,
    pub indent_width: u8,
}

pub struct TailwindWrapper {
    option: TailwindWrapperOption,
    pre_jsx_element_line: usize,
    pre_indent_level: u32,
    jsxexpression: bool,
}

impl TailwindWrapper {
    pub fn new(option: TailwindWrapperOption) -> Self {
        Self {
            option,
            pre_jsx_element_line: 0,
            pre_indent_level: 0,
            jsxexpression: false,
        }
    }

    pub fn set_pre_jsx_element_line(&mut self, source_text: &str, node_span_start: usize) {
        self.pre_jsx_element_line = get_line_number(source_text, node_span_start);
    }

    pub fn set_pre_indent_level(&mut self, source_text: &str, node_span_start: usize) {
        let column = get_column_number(source_text, node_span_start);
        self.pre_indent_level = get_indent_level(column, self.option.indent_width);
    }

    pub fn enter_jsxexpression(&mut self) {
        self.jsxexpression = true
    }

    pub fn leave_jsxexpression(&mut self) {
        self.jsxexpression = false
    }
}

impl TailwindWrapper {
    pub fn format(
        &self,
        node_text: &str,
        source_text: &str,
        attr_name_span_start: usize,
        attr_value_span_start: usize,
    ) -> PrintItems {
        let wrapped_items = self.wrap_text(node_text, source_text, attr_value_span_start);

        if self.option.indent_to_quote {
            return wrapped_items;
        }

        let line = get_line_number(source_text, attr_value_span_start);
        let indent = if line == self.pre_jsx_element_line {
            self.pre_indent_level + 1
        } else {
            let column = get_column_number(source_text, attr_name_span_start);
            get_indent_level(column, self.option.indent_width) + 1
        };
        ir_helpers::with_indent_times(wrapped_items, indent)
    }
}

impl TailwindWrapper {
    fn wrap_text(
        &self,
        node_text: &str,
        source_text: &str,
        attr_value_span_start: usize,
    ) -> PrintItems {
        let indent_column = self
            .option
            .indent_to_quote
            .then(|| get_column_number(source_text, attr_value_span_start));

        let push_break_line = |items: &mut PrintItems| {
            push_jsxexpression_endl(items, self.jsxexpression);
            push_newline(items);
            if let Some(column) = indent_column {
                push_spaces(items, column);
            }
        };

        let mut current_width = 0;

        let parts: Vec<_> = node_text.split_whitespace().collect();
        let last_index = parts.len() - 1;

        parts
            .iter()
            .enumerate()
            .fold(PrintItems::new(), |mut items, (i, text)| {
                let text_width = text.chars().count() as u32;
                let next_width = current_width + text_width + 1;
                let exceeds = next_width > self.option.line_width;
                let not_first = i > 0;

                match (exceeds, self.option.allow_line_overflow) {
                    (true, true) => {
                        push_text(&mut items, text, current_width > 0);
                        current_width = 0;
                        if i < last_index {
                            push_break_line(&mut items);
                        }
                    }
                    (true, false) => {
                        push_break_line(&mut items);
                        push_text(&mut items, text, false);
                        current_width = text_width;
                    }
                    _ => {
                        push_text(&mut items, text, not_first);
                        current_width = next_width;
                    }
                }

                items
            })
    }
}

/// 0-indexed
fn get_column_number(text: &str, start: usize) -> usize {
    dprint_core::formatting::utils::string_utils::get_column_number_of_pos(text, start) - 1
}

/// 0-indexed
fn get_line_number(text: &str, start: usize) -> usize {
    dprint_core::formatting::utils::string_utils::get_line_number_of_pos(text, start) - 1
}

/// 0-indexed
fn get_indent_level(column: usize, indent_width: u8) -> u32 {
    let indent_width = indent_width as usize;
    let level = column / indent_width;
    level.try_into().unwrap_or(0)
}

fn push_spaces(items: &mut PrintItems, space_count: usize) {
    (0..space_count).for_each(|_| items.push_space());
}

fn push_text(items: &mut PrintItems, text: &str, leading_space: bool) {
    if leading_space {
        push_spaces(items, 1);
    }
    items.push_string(text.to_string());
}

fn push_jsxexpression_endl(items: &mut PrintItems, jsxexpression: bool) {
    if jsxexpression {
        items.push_space();
        items.push_sc(sc!("\\"));
    }
}

fn push_newline(items: &mut PrintItems) {
    items.push_signal(Signal::NewLine);
}
