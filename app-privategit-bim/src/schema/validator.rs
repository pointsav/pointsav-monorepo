use serde_json::Value;

pub fn validate_dtcg(doc: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    let bim = match doc.get("bim") {
        Some(b) if b.is_object() => b.as_object().unwrap(),
        Some(_) => {
            errors.push("top-level 'bim' must be an object".into());
            return Err(errors);
        }
        None => {
            errors.push("missing required top-level 'bim' object".into());
            return Err(errors);
        }
    };

    for (cat, cat_val) in bim {
        let entities = match cat_val.as_object() {
            Some(o) => o,
            None => {
                errors.push(format!("bim.{cat}: expected object"));
                continue;
            }
        };
        for (slug, entity) in entities {
            let path = format!("bim.{cat}.{slug}");

            let typ = entity.get("$type").and_then(|v| v.as_str()).unwrap_or("");
            if typ != "bim.entity" {
                errors.push(format!("{path}: $type must be 'bim.entity', got '{typ}'"));
            }

            match entity.get("$value") {
                None => errors.push(format!("{path}: missing $value")),
                Some(v) if !v.is_object() => {
                    errors.push(format!("{path}: $value must be an object"))
                }
                Some(v) => {
                    let obj = v.as_object().unwrap();
                    let has_ifc = obj.contains_key("ifc_class") || obj.contains_key("ifc_anchor");
                    let has_zone = obj.contains_key("zone1_depth_m");
                    let has_name = obj.contains_key("display_name");
                    if !has_ifc && !has_zone && !has_name {
                        errors.push(format!(
                            "{path}: $value missing required fields (ifc_class, ifc_anchor, or display_name)"
                        ));
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
