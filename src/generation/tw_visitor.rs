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

use crate::{configuration::Configuration, generation::tw_wrapper::TailwindWrapper};

enum PrintKind {
    StringLiteral,
    JSXExpression,
    None,
}

pub struct TailwindVisitor<'a> {
    source_text: &'a str,
    print_items: PrintItems,
    wrapper: Option<TailwindWrapper>,
    last_offset: usize,
    config: &'a Configuration,
    print_kind: PrintKind,
}

impl<'a> TailwindVisitor<'a> {
    pub fn new(source_text: &'a str, config: &'a Configuration) -> Self {
        Self {
            source_text,
            print_items: PrintItems::new(),
            wrapper: None,
            last_offset: 0,
            config,
            print_kind: PrintKind::None,
        }
    }

    pub fn with_wrapper(mut self, wrapper: Option<TailwindWrapper>) -> Self {
        self.wrapper = wrapper;
        self
    }

    pub fn print_items(self) -> PrintItems {
        self.print_items
    }
}

impl<'a> TailwindVisitor<'a> {
    fn r#match_attr(&self, target: &str) -> bool {
        self.config.classname_attributes.contains(target)
    }

    fn print_pre_text(&mut self, current_span: &Span) {
        let range = self.last_offset..current_span.start as usize;
        let pre_text = self.source_text.get(range).unwrap();

        self.print_items
            .extend(ir_helpers::gen_from_string(pre_text));

        self.last_offset = current_span.end as usize;
    }

    fn print_current_text(
        &mut self,
        text: &Atom<'_>,
        attr_name_span: &Span,
        attr_value_span: &Span,
    ) {
        match &mut self.wrapper {
            Some(wrapper) => {
                if let PrintKind::JSXExpression = self.print_kind {
                    wrapper.enter_jsxexpression();
                }
                self.print_items.extend(wrapper.format(
                    text,
                    self.source_text,
                    attr_name_span.start as usize,
                    attr_value_span.start as usize,
                ));
                if let PrintKind::JSXExpression = self.print_kind {
                    wrapper.leave_jsxexpression();
                }
            }
            None => self.print_items.extend(ir_helpers::gen_from_string(text)),
        }
    }

    fn print_post_text(&mut self) {
        let range = self.last_offset..;
        let post_text = self.source_text.get(range).unwrap();

        self.print_items
            .extend(ir_helpers::gen_from_string(post_text));
    }
}

impl<'a> Visit<'a> for TailwindVisitor<'a> {
    fn leave_node(&mut self, kind: AstKind<'a>) {
        if let AstKind::Program(_) = kind {
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
        if self.match_attr(it.name.get_identifier().name.as_str()) {
            let value = it.value.as_ref().unwrap();
            match value {
                JSXAttributeValue::StringLiteral(string_literal) => {
                    self.print_kind = PrintKind::StringLiteral;
                    self.print_pre_text(&string_literal.span);
                    self.print_current_text(
                        &string_literal.raw.unwrap(),
                        &it.name.get_identifier().span,
                        &string_literal.span,
                    );
                }
                JSXAttributeValue::ExpressionContainer(jsxexpression_container) => {
                    if let JSXExpression::StringLiteral(string_literal) =
                        &jsxexpression_container.expression
                    {
                        self.print_kind = PrintKind::JSXExpression;
                        self.print_pre_text(&string_literal.span);
                        self.print_current_text(
                            &string_literal.raw.unwrap(),
                            &it.name.get_identifier().span,
                            &string_literal.span,
                        );
                    }
                }
                _ => {}
            }
        }
        walk_jsx_attribute(self, it);
    }
}
