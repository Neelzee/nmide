use core_std_lib::{attrs::Attr, html::Html};
use scraper::{Html as ScraperHtml, Selector, ElementRef};
use anyhow::{anyhow, Result};

pub fn parse_html(html_str: &str) -> Result<Html> {
    let document = ScraperHtml::parse_document(html_str);
    
    let root_element = find_root_element(&document)?;
    parse_element(root_element)
}

fn find_root_element(document: &ScraperHtml) -> Result<ElementRef> {
    let content_selector = Selector::parse("*").unwrap();
    
    for element in document.select(&content_selector) {
        let tag_name = element.value().name();
        if !matches!(tag_name, "html" | "head" | "body") {
            return Ok(element);
        }
    }
    
    let body_selector = Selector::parse("body").unwrap();
    if let Some(body) = document.select(&body_selector).next() {
        for child in body.children() {
            if child.value().is_element() {
                if let Some(child_element) = ElementRef::wrap(child) {
                    return Ok(child_element);
                }
            }
        }
        return Err(anyhow!("No content found in body"));
    }
    
    let all_selector = Selector::parse("*").unwrap();
    document.select(&all_selector).next()
        .ok_or(anyhow!("No elements found in HTML"))
}

fn parse_element(element: ElementRef) -> Result<Html> {
    let tag_name = element.value().name();
    
    let mut attrs = Vec::new();
    for (name, value) in element.value().attrs() {
        let attr = match name {
            "id" => Attr::Id(value.to_string()),
            "class" => Attr::Class(value.to_string()),
            "type" => Attr::Type(value.to_string()),
            "checked" => Attr::Checked(value.parse().unwrap_or(false)),
            "src" => Attr::Src(value.to_string()),
            _ => Attr::Custom(name.to_string(), value.to_string()),
        };
        attrs.push(attr);
    }
    
    let mut kids = Vec::new();
    let mut text_content = String::new();
    
    for child in element.children() {
        match child.value() {
            scraper::Node::Element(_) => {
                if let Some(child_element) = ElementRef::wrap(child) {
                    kids.push(parse_element(child_element)?);
                }
            }
            scraper::Node::Text(text) => {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    text_content.push_str(trimmed);
                }
            }
            _ => {}
        }
    }
    
    let mut html_node = match tag_name.to_lowercase().as_str() {
        "div" => Html::Div(),
        "span" => Html::Span(),
        "p" => Html::P(),
        "h1" => Html::H1(),
        "h2" => Html::H2(),
        "img" => Html::Img(),
        "table" => Html::Table(),
        "thead" => Html::Thead(),
        "tbody" => Html::Tbody(),
        "tr" => Html::Tr(),
        "th" => Html::Th(),
        "td" => Html::Td(),
        "label" => Html::Label(),
        "input" => Html::Input(),
        "button" => Html::Button(),
        "form" => Html::Form(),
        "strong" => Html::Strong(),
        "em" => Html::Em(),
        "li" => Html::Li(),
        "ul" => Html::Ul(),
        "br" => Html::Br(),
        "nav" => Html::Nav(),
        "header" => Html::Header(),
        "article" => Html::Article(),
        "section" => Html::Section(),
        "footer" => Html::Footer(),
        _ => return Err(anyhow!("Unsupported tag: {}", tag_name)),
    };
    
    html_node = html_node.set_attrs(attrs);
    html_node = html_node.replace_kids(kids);
    
    if !text_content.is_empty() {
        html_node = html_node.set_text(text_content);
    }
    
    Ok(html_node)
}