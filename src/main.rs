use itertools::Itertools;
use nu_plugin::{
    serve_plugin, EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand,
    SimplePluginCommand,
};
use nu_protocol::{
    record, Category, Example, LabeledError, Signature, Span, Spanned, SyntaxShape, Value,
};
use std::io::Write;

struct EmojiPlugin;

impl Plugin for EmojiPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Emoji)]
    }
}

struct Emoji;

impl SimplePluginCommand for Emoji {
    type Plugin = EmojiPlugin;

    fn name(&self) -> &str {
        "emoji"
    }

    fn description(&self) -> &str {
        "Create emojis from text."
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .optional(
                "emoji-name",
                SyntaxShape::String,
                "name of the emoji shorthand with colons before and after e.g. :grinning:",
            )
            .switch("list", "List stuff", Some('l'))
            .category(Category::Experimental)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Show the smirk emoji".into(),
                example: "emoji :smirk:".into(),
                result: None,
            },
            Example {
                description: "List all known emojis".into(),
                example: "emoji --list".into(),
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _plugin: &EmojiPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let param: Option<Spanned<String>> = call.opt(0)?;
        let list = call.has_flag("list")?;

        if list {
            let mut rec = vec![];
            for emoji in emojis::iter() {
                let span = Span::unknown();
                let em = emoji.as_str().to_string();

                let emoji_chars = em.chars().collect::<Vec<char>>();
                let mut cp = String::new();
                for c in emoji_chars {
                    cp.push_str(&format!("{:X} ", c as u32));
                }

                let na = emoji.name().to_string();
                let unic = emoji.unicode_version();
                let gr = emoji.group();
                let bi = emoji.as_bytes();
                // let sh = emoji.shortcode();
                let shc = emoji.shortcodes();
                // let sk = emoji.skin_tone();
                // 1F44B       ; fully-qualified # 👋 E0.6 waving hand
                // 1F44B 1F3FB ; fully-qualified # 👋🏻 E1.0 waving hand: light skin tone
                // 1F44B 1F3FC ; fully-qualified # 👋🏼 E1.0 waving hand: medium-light skin tone
                // 1F44B 1F3FD ; fully-qualified # 👋🏽 E1.0 waving hand: medium skin tone
                // 1F44B 1F3FE ; fully-qualified # 👋🏾 E1.0 waving hand: medium-dark skin tone
                // 1F44B 1F3FF ; fully-qualified # 👋🏿 E1.0 waving hand: dark skin tone
                let mut sks = vec![];
                if let Some(st) = emoji.skin_tones() {
                    for s in st {
                        sks.push(s.as_str());
                    }
                };

                rec.push(Value::record(
                    record! {
                        "emoji" => Value::string(em, span),
                        "name" => Value::string(na, span),
                        "unicode_version" => Value::string(format!("{}.{}", unic.major(), unic.minor()), span),
                        "group" => Value::string(format!("{:?}", gr), span),
                        "utf8_bytes" => Value::string(format!("{:X?}", bi), span),
                        "codepoints" => Value::string(cp.trim().to_string(), span),
                        "shortcodes" => Value::string(shc.into_iter().join(", "), span),
                        "skin_tones" => Value::string(sks.join(", "), span),
                    },
                    span,
                ))
            }

            return Ok(Value::list(rec, call.head));
        }

        if let Some(emoj) = param {
            let emoji = replace(&emoj.item).map_err(|op| {
                LabeledError::new(format!("Error in emoji plugin: {}", op))
                    .with_label("Error in emoji plugin", emoj.span)
            })?;
            return Ok(Value::string(emoji, emoj.span));
        } else {
            return Err(LabeledError::new("requires some input, got None")
                .with_label("Expected something from pipeline", call.head));
        }
    }
}

fn main() {
    serve_plugin(&EmojiPlugin, MsgPackSerializer);
}

fn replace(mut s: &str) -> Result<String, std::io::Error> {
    // The meaning of the index values is as follows.
    //
    //  : r o c k e t :
    // ^ ^           ^ ^
    // i m           n j
    //
    // i..j gives ":rocket:"
    // m..n gives "rocket"
    let mut o = Vec::new();
    while let Some((i, m, n, j)) = s
        .find(':')
        .map(|i| (i, i + 1))
        .and_then(|(i, m)| s[m..].find(':').map(|x| (i, m, m + x, m + x + 1)))
    {
        match emojis::get_by_shortcode(&s[m..n]) {
            Some(emoji) => {
                // Output everything preceding, except the first colon.
                o.write_all(s[..i].as_bytes())?;
                // Output the emoji.
                o.write_all(emoji.as_bytes())?;
                // Update the string to past the last colon.
                s = &s[j..];
            }
            None => {
                // Output everything preceding but not including the colon.
                o.write_all(s[..n].as_bytes())?;
                // Update the string to start with the last colon.
                s = &s[n..];
            }
        }
    }
    o.write_all(s.as_bytes())?;
    Ok(String::from_utf8(o).unwrap())
}
