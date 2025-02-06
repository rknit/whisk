use crate::{
    ast::{location::Located, nodes::attributes::Attributes, parsing::token::Keyword},
    lowering::errors::IdentResolveError,
    old_symbol_table::SymbolAttribute,
};

use super::ResolveContext;

impl Attributes {
    pub(in super::super) fn resolve(
        &self,
        ctx: &mut ResolveContext,
        allowed_attributes: &[SymbolAttribute],
    ) -> Vec<SymbolAttribute> {
        let mut attribs = Vec::new();

        for Located(kw, loc) in &self.attribs {
            let attrib = match kw {
                Keyword::Pub => SymbolAttribute::Public,
                _ => unimplemented!("attrib kw: '{}'", kw),
            };

            if allowed_attributes.contains(&attrib) {
                attribs.push(attrib);
            } else {
                ctx.push_error(
                    IdentResolveError::UnexpectedAttrib {
                        attribute: Located(attrib, *loc),
                        allowed_attributes: allowed_attributes.into(),
                    }
                    .into(),
                );
            }
        }

        attribs
    }
}
