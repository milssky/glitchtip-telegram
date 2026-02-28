//! Format Glitchtip payload into Telegram message text.
//!
//! Placeholders in templates: message — {text}, {attachments};
//! attachment — {title}, {link}, {attach_text}, {fields}; field — {title}, {value}.

use crate::{Attachment, Field, Payload};

const MESSAGE_TEMPLATE: &str = r#"🚨 <b>{text}</b> 🚨

{attachments}"#;
const ATTACHMENT_TEMPLATE: &str = r#"📌 <b>{title}</b>
🔗 <b>Link:</b> {link}

{attach_text}
{fields}"#;
const FIELD_TEMPLATE: &str = "➡️ {title}: {value}";

/// Build Telegram message string from webhook payload.
pub fn format_payload(payload: &Payload) -> String {
    let attachments_text: String = payload
        .attachments
        .iter()
        .map(format_attachment)
        .collect::<Vec<_>>()
        .join("\n");

    MESSAGE_TEMPLATE
        .replace("{text}", &payload.text)
        .replace("{attachments}", &attachments_text)
        .trim()
        .to_string()
}

fn format_attachment(att: &Attachment) -> String {
    let fields = att
        .fields
        .as_ref()
        .map(|fs| fs.iter().map(format_field).collect::<Vec<_>>().join("\n"))
        .unwrap_or_default();

    ATTACHMENT_TEMPLATE
        .replace("{title}", &att.title)
        .replace("{link}", att.title_link.as_deref().unwrap_or(""))
        .replace("{attach_text}", att.text.as_deref().unwrap_or(""))
        .replace("{fields}", &fields)
}

fn format_field(field: &Field) -> String {
    FIELD_TEMPLATE
        .replace("{title}", &field.title)
        .replace("{value}", &field.value)
}
