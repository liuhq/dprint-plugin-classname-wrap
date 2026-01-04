use dprint_core::formatting::{PrintItems, Signal, ir_helpers, utils::string_utils};
use dprint_core_macros::sc;

use crate::generation::{
    types::{IntoU32, IntoUsize},
    visitor::{AttributeContext, AttributePos},
};

#[derive(Debug, Clone, Copy)]
enum IndentCount {
    IndentToQuote(u32),
    IndentToPre(u32),
}

impl IndentCount {
    #[inline]
    const fn value(&self) -> u32 {
        match self {
            Self::IndentToQuote(n) | Self::IndentToPre(n) => *n,
        }
    }

    #[inline]
    fn level(&self, indent_width: u8) -> u32 {
        calculate_indent_level(self.value().into_usize(), indent_width)
    }
}

pub struct WrapperOption {
    pub allow_line_overflow: bool,
    pub indent_to_quote: bool,
    pub indent_width: u8,
    pub line_width_includes_indent: bool,
    pub line_width: u32,
}

pub struct Wrapper {
    option: WrapperOption,
    pre_jsx_element_line: u32,
    pre_indent_count: u32,
}

impl Wrapper {
    pub fn new(option: WrapperOption) -> Self {
        Self {
            option,
            pre_jsx_element_line: 0,
            pre_indent_count: 0,
        }
    }

    pub fn set_pre_jsx_element_line(&mut self, source_text: &str, node_span_start: usize) {
        self.pre_jsx_element_line = calculate_line_number(source_text, node_span_start);
    }

    pub fn set_pre_indent_count(&mut self, source_text: &str, node_span_start: usize) {
        self.pre_indent_count = calculate_column_number(source_text, node_span_start)
    }
}

impl Wrapper {
    pub fn format(
        &self,
        node_text: &str,
        attr_pos: AttributePos,
        context: AttributeContext,
    ) -> PrintItems {
        let indent_count = self.parse_indent(attr_pos);

        let attr_value_column =
            calculate_column_number(attr_pos.source_text(), attr_pos.attr_value_span_start());
        let wrapped_items = self.wrap_text(node_text, &indent_count, attr_value_column, context);

        if self.option.indent_to_quote {
            wrapped_items
        } else {
            ir_helpers::with_indent_times(
                wrapped_items,
                indent_count.level(self.option.indent_width),
            )
        }
    }
}

impl Wrapper {
    fn parse_indent(&self, attr_pos: AttributePos) -> IndentCount {
        if self.option.indent_to_quote {
            return IndentCount::IndentToQuote(calculate_column_number(
                attr_pos.source_text(),
                attr_pos.attr_value_span_start(),
            ));
        }

        let line = calculate_line_number(attr_pos.source_text(), attr_pos.attr_value_span_start());
        let indent_width = u32::from(self.option.indent_width);

        let indent_count = if line == self.pre_jsx_element_line {
            self.pre_indent_count + indent_width
        } else {
            calculate_column_number(attr_pos.source_text(), attr_pos.attr_name_span_start())
                + indent_width
        };

        IndentCount::IndentToPre(indent_count)
    }

    fn wrap_text(
        &self,
        node_text: &str,
        indent_count: &IndentCount,
        first_lint_column: u32,
        context: AttributeContext,
    ) -> PrintItems {
        let parts: Vec<_> = node_text
            .split_whitespace()
            .filter(|c| *c != "\\")
            .collect();

        if parts.is_empty() {
            return PrintItems::new();
        }

        let last_index = parts.len() - 1;
        let mut current_width = first_lint_column;

        parts
            .iter()
            .enumerate()
            .fold(PrintItems::new(), |mut items, (i, text)| {
                let text_width = text.chars().count().into_u32();
                let next_width = current_width + text_width + 1;

                let exceeds_width = if self.option.line_width_includes_indent {
                    next_width > self.option.line_width.saturating_sub(indent_count.value())
                } else {
                    next_width > self.option.line_width
                };

                match (exceeds_width, self.option.allow_line_overflow) {
                    (true, true) => {
                        append_text(&mut items, text, current_width > 0);
                        current_width = 0;

                        if i < last_index {
                            append_break_line(&mut items, context, indent_count);
                        }
                    }
                    (true, false) => {
                        append_break_line(&mut items, context, indent_count);
                        append_text(&mut items, text, false);
                        current_width = text_width;
                    }
                    (false, _) => {
                        append_text(&mut items, text, i > 0);
                        current_width = next_width;
                    }
                }

                items
            })
    }
}

/// 0-indexed
#[inline]
fn calculate_column_number(text: &str, start: usize) -> u32 {
    string_utils::get_column_number_of_pos(text, start)
        .into_u32()
        .saturating_sub(1)
}

/// 0-indexed
#[inline]
fn calculate_line_number(text: &str, start: usize) -> u32 {
    string_utils::get_line_number_of_pos(text, start)
        .into_u32()
        .saturating_sub(1)
}

/// 0-indexed
#[inline]
fn calculate_indent_level(column: usize, indent_width: u8) -> u32 {
    column.into_u32() / u32::from(indent_width)
}

#[inline]
fn append_spaces(items: &mut PrintItems, count: u32) {
    (0..count).for_each(|_| items.push_space());
}

#[inline]
fn append_text(items: &mut PrintItems, text: &str, leading_space: bool) {
    if leading_space {
        items.push_space();
    }
    items.push_string(text.to_string());
}

// fn push_jsxexpression_endl(items: &mut PrintItems, jsxexpression: bool) {
//     if jsxexpression {
//         items.push_space();
//         items.push_sc(sc!("\\"));
//     }
// }
//
// fn push_newline(items: &mut PrintItems) {
//     items.push_signal(Signal::NewLine);
// }

fn append_break_line(
    items: &mut PrintItems,
    context: AttributeContext,
    indent_count: &IndentCount,
) {
    if matches!(context, AttributeContext::JSXExpression) {
        items.push_space();
        items.push_sc(sc!("\\"));
    }

    items.push_signal(Signal::NewLine);

    if let IndentCount::IndentToQuote(column) = indent_count {
        append_spaces(items, *column);
    }
}
