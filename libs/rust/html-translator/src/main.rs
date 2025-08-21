use core_std_lib::{attrs::Attr, html::Html};

#[cfg(test)]
mod tests;

mod translator;

fn main () {
    println!("Hello, world!");
}

pub fn translate(ui: Html) -> String {
    _translate(ui, 0)
}

fn _translate(ui: Html, indent_level: usize) -> String {
    let tag_name = ui.tag_name();
    let indent = "  ".repeat(indent_level);
    
    let opening_tag = if ui.attrs().is_empty() {
        format!("<{}>", tag_name)
    } else {
        let attrs_str = ui.attrs()
            .into_iter()
            .filter_map(translate_attr)
            .collect::<Vec<_>>()
            .join(" ");
        format!("<{} {}>", tag_name, attrs_str)
    };
    
    let closing_tag = format!("</{}>", tag_name);
    
    let text_content = ui.text();
    if !text_content.is_empty() {
        return format!("{}{}{}{}", indent, opening_tag, text_content, closing_tag);
    }
    
    if ui.kids().is_empty() {
        format!("{}{}{}", indent, opening_tag, closing_tag)
    } else {
        let children_html = ui.kids()
            .into_iter()
            .map(|child| _translate(child, indent_level + 1))
            .collect::<Vec<_>>()
            .join("\n");
        
        format!("{}{}\n{}\n{}{}", 
            indent, opening_tag,
            children_html,
            indent, closing_tag
        )
    }
}

pub fn translate_attr(a: Attr) -> Option<String> {
    match a {
        Attr::Id(id) => Some(format!(r#"id="{}""#, id)),
        Attr::Class(class) => Some(format!(r#"class="{}""#, class)),
        Attr::Type(t) => Some(format!(r#"type="{}""#, t)),
        Attr::Checked(chk) => Some(format!(r#"checked="{}""#, chk)),
        Attr::Src(src) => Some(format!(r#"src="{}""#, src)),
        Attr::Custom(k, v) => Some(format!(r#"{}="{}""#, k, v)),
        _ => None,
    }
}