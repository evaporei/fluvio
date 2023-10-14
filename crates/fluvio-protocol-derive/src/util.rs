use syn::{Attribute, Expr, Lit, LitStr, Meta, MetaNameValue};

pub(crate) fn find_attr(attrs: &[Attribute], name: &str) -> Option<Meta> {
    attrs.iter().find_map(|a| {
        let mut attr = None;
        a.parse_nested_meta(|meta| {
            if meta.path.is_ident(name) {
                let value = meta.value()?;
                attr = Some(value.parse()?);
            }
            Ok(())
        })
        .ok()?;
        attr
    })
}

pub(crate) fn find_name_attribute<'a>(meta: &'a Meta, name: &str) -> Option<MetaNameValue> {
    find_meta(meta, name).map(|meta| match meta {
        Meta::NameValue(name_value) => name_value,
        _ => panic!("should not happen"),
    })
}

pub(crate) fn find_meta<'a>(meta: &'a Meta, name: &str) -> Option<Meta> {
    let mut ret_meta = None;

    if let Meta::List(list) = meta {
        // for attr in list.parse_nested_meta(|attrs|.iter() {
        list.parse_nested_meta(|attr| {
            let value = attr.value()?;
            let named_meta: Meta = value.parse()?;
            match named_meta {
                Meta::NameValue(ref meta_name_value) => {
                    if meta_name_value.path.is_ident(name) {
                        ret_meta = Some(named_meta);
                    }
                }
                Meta::Path(ref path) => {
                    if path.is_ident(name) {
                        ret_meta = Some(named_meta);
                    }
                }
                Meta::List(_) => {}
            }
            Ok(())
        })
        .ok()?;
    }

    ret_meta
}

/// find name value with integer value
pub(crate) fn find_int_name_value(version_meta: &Meta, attr_name: &str) -> Option<u64> {
    if let Some(attr) = find_name_attribute(version_meta, attr_name) {
        if let Expr::Lit(expr_lit) = &attr.value {
            match &expr_lit.lit {
                Lit::Int(version_val) => {
                    //  println!("version value: {}",version_val.value());
                    version_val.base10_parse::<u64>().ok()
                }
                _ => unimplemented!(),
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// find name value with str value
pub(crate) fn find_string_name_value(version_meta: &Meta, attr_name: &str) -> Option<LitStr> {
    if let Some(attr) = find_name_attribute(version_meta, attr_name) {
        if let Expr::Lit(expr_lit) = &attr.value {
            match &expr_lit.lit {
                Lit::Str(val) => Some(val.clone()),
                _ => unimplemented!(),
            }
        } else {
            None
        }
    } else {
        None
    }
}
