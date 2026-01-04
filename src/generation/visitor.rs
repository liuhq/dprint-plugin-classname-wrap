use dprint_core::formatting::{PrintItems, ir_helpers};
use oxc::{
    ast::{
        AstKind,
        ast::{JSXAttribute, JSXAttributeValue, JSXElement, JSXExpression},
    },
    ast_visit::{
        Visit,
        walk::{walk_jsx_attribute, walk_jsx_element},
    },
    span::{Atom, Span},
};

use crate::{configuration::Configuration, generation::wrapper::Wrapper};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeContext {
    StringLiteral,
    JSXExpression,
}

#[derive(Debug, Clone, Copy)]
pub struct AttributePos<'a> {
    source_text: &'a str,
    attr_name_span: &'a Span,
    attr_value_span: &'a Span,
}

impl<'a> AttributePos<'a> {
    pub fn new(source_text: &'a str, attr_name_span: &'a Span, attr_value_span: &'a Span) -> Self {
        Self {
            source_text,
            attr_name_span,
            attr_value_span,
        }
    }

    pub fn source_text(&self) -> &str {
        self.source_text
    }

    pub fn attr_name_span_start(&self) -> usize {
        self.attr_name_span.start as usize
    }

    pub fn attr_value_span_start(&self) -> usize {
        self.attr_value_span.start as usize
    }
}

pub struct Visitor<'a> {
    source_text: &'a str,
    print_items: PrintItems,
    wrapper: Option<Wrapper>,
    last_offset: usize,
    config: &'a Configuration,
}

impl<'a> Visitor<'a> {
    pub fn new(source_text: &'a str, config: &'a Configuration) -> Self {
        Self {
            source_text,
            print_items: PrintItems::new(),
            wrapper: None,
            last_offset: 0,
            config,
        }
    }

    #[must_use]
    pub fn with_wrapper(mut self, wrapper: Option<Wrapper>) -> Self {
        self.wrapper = wrapper;
        self
    }

    #[must_use]
    pub fn print_items(self) -> PrintItems {
        self.print_items
    }
}

impl<'a> Visitor<'a> {
    #[inline]
    fn match_attr(&self, target: &str) -> bool {
        self.config.classname_attributes.contains(target)
    }

    fn print_pre_text(&mut self, current_span: &Span) {
        let start = current_span.start as usize;
        let range = self.last_offset..start;

        if let Some(pre_text) = self.source_text.get(range) {
            self.print_items
                .extend(ir_helpers::gen_from_string(pre_text));
            self.last_offset = current_span.end as usize;
        }
    }

    fn print_current_text(
        &mut self,
        text: &Atom<'_>,
        attr_name_span: &Span,
        attr_value_span: &Span,
        context: AttributeContext,
    ) {
        match &mut self.wrapper {
            Some(wrapper) => {
                let attr_pos = AttributePos::new(self.source_text, attr_name_span, attr_value_span);

                self.print_items
                    .extend(wrapper.format(text, attr_pos, context));
            }
            None => self.print_items.extend(ir_helpers::gen_from_string(text)),
        }
    }

    fn print_post_text(&mut self) {
        let range = self.last_offset..;

        if let Some(post_text) = self.source_text.get(range) {
            self.print_items
                .extend(ir_helpers::gen_from_string(post_text));
        }
    }

    fn handle_string_literal(
        &mut self,
        string_literal_span: &Span,
        raw_text: &Atom<'_>,
        attr_name_span: &Span,
        context: AttributeContext,
    ) {
        self.print_pre_text(string_literal_span);
        self.print_current_text(raw_text, attr_name_span, string_literal_span, context);
    }
}

impl<'a> Visit<'a> for Visitor<'a> {
    fn leave_node(&mut self, kind: AstKind<'a>) {
        if matches!(kind, AstKind::Program(_)) {
            self.print_post_text();
        }
    }

    fn visit_jsx_element(&mut self, it: &JSXElement<'a>) {
        if let Some(wrapper) = &mut self.wrapper {
            let source_text = self.source_text;
            let node_span_start = it.opening_element.span.start as usize;
            wrapper.set_pre_jsx_element_line(source_text, node_span_start);
            wrapper.set_pre_indent_count(source_text, node_span_start);
        }
        walk_jsx_element(self, it);
    }

    fn visit_jsx_attribute(&mut self, it: &JSXAttribute<'a>) {
        let attr_name = it.name.get_identifier();

        if self.match_attr(attr_name.name.as_str()) {
            if let Some(value) = it.value.as_ref() {
                match value {
                    JSXAttributeValue::StringLiteral(literal) => {
                        if let Some(raw) = &literal.raw {
                            self.handle_string_literal(
                                &literal.span,
                                raw,
                                &attr_name.span,
                                AttributeContext::StringLiteral,
                            );
                        }
                    }
                    JSXAttributeValue::ExpressionContainer(container) => {
                        if let JSXExpression::StringLiteral(literal) = &container.expression {
                            if let Some(raw) = &literal.raw {
                                self.handle_string_literal(
                                    &literal.span,
                                    raw,
                                    &attr_name.span,
                                    AttributeContext::JSXExpression,
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        walk_jsx_attribute(self, it);
    }
}
