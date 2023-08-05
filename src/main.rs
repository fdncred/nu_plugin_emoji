mod emojis;

use itertools::Itertools;
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Span, Spanned, SyntaxShape, Value};
// use std::ascii::escape_default;
use std::io::Write;
// use unicode_segmentation::UnicodeSegmentation;

struct Implementation;

impl Implementation {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Implementation {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("emoji")
            .usage("Create emojis from text")
            .optional(
                "emoji-name",
                SyntaxShape::String,
                "name of the emoji shorthand with colons before and after",
            )
            .switch("list", "List stuff", Some('l'))
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "This is the example descripion".into(),
                example: "some pipeline involving emoji".into(),
                result: None,
            }])]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "emoji");
        let param: Option<Spanned<String>> = call.opt(0)?;
        let list = call.has_flag("list");

        if list {
            let mut rec = vec![];
            for emoji in emojis::iter() {
                let mut cols = vec![];
                let mut vals = vec![];
                let span = Span::unknown();
                let em = emoji.as_str().to_string();
                let cp = emoji.codepoints().to_string();
                let na = emoji.name().to_string();
                let unic = emoji.unicode_version();
                let gr = emoji.group();
                let bi = emoji.as_bytes();
                // let sh = emoji.shortcode();
                let shc = emoji.shortcodes();
                let sk = emoji.skin_tone();
                // let sks = emoji.skin_tones();
                // let s = emoji.to_string();

                cols.push("emoji".to_string());
                vals.push(Value::String { val: em, span });
                cols.push("name".to_string());
                vals.push(Value::String { val: na, span });
                cols.push("unicode_version".to_string());
                vals.push(Value::String {
                    val: format!("{}.{}", unic.major(), unic.minor()),
                    span,
                });
                cols.push("group".to_string());
                vals.push(Value::String {
                    val: format!("{:?}", gr),
                    span,
                });
                cols.push("utf-8 encoding".to_string());
                // let mut visible = String::new();
                // for &b in bi {
                //     let part: Vec<u8> = escape_default(b).collect();
                //     visible.push_str(std::str::from_utf8(&part).unwrap());
                // }

                // let codepoints: Vec<char> = bi.to_vec().into_iter().map(char::from).collect();
                // vals.push(Value::String {
                //     val: codepoints.into_iter().join(", "),
                //     span,
                // });

                // vals.push(Value::String {
                //     val: format!("\\U{{{}}}", bi.iter().format(" ")),
                //     span,
                // });

                // vals.push(Value::String {
                //     val: format!("{:X?}", bi),
                //     span,
                // });

                // vals.push(Value::String {
                //     val: char::from_u32(u32::from_ne_bytes(bi)),
                //     span,
                // });

                vals.push(Value::String {
                    // val: format!("{:#X}", bi.iter().map(|&b| b as u32).format(" ")),
                    val: format!("{:X?}", bi),
                    span,
                });

                cols.push("codepoints".to_string());
                vals.push(Value::String { val: cp, span });
                // cols.push("shortcode".to_string());
                // vals.push(if let Some(sc) = sh {
                //     Value::String {
                //         val: sc.to_string(),
                //         span,
                //     }
                // } else {
                //     Value::Nothing { span }
                // });
                cols.push("shortcodes".to_string());
                vals.push(Value::String {
                    val: shc.into_iter().join(", "),
                    span,
                });
                // vals.push(
                //     shc.into_iter()
                //         .fold(Value::Nothing { span }, |acc, sc| Value::List {
                //             vals: vec![
                //                 acc,
                //                 Value::String {
                //                     val: sc.to_string(),
                //                     span,
                //                 },
                //             ],
                //             span,
                //         }),
                // );
                cols.push("skin_tone".to_string());
                vals.push(if let Some(st) = sk {
                    Value::String {
                        val: format!("{:?}", st),
                        span,
                    }
                } else {
                    Value::Nothing { span }
                });
                // cols.push("skin_tones".to_string());
                // vals.push(
                //     sks.into_iter()
                //         .fold(Value::Nothing { span }, |acc, st| Value::List {
                //             vals: vec![
                //                 acc,
                //                 Value::String {
                //                     val: format!("{:?}", st.collect::<Emoji>()),
                //                     span,
                //                 },
                //             ],
                //             span,
                //         }),
                // );
                // cols.push("to_string".to_string());
                // vals.push(Value::String { val: s, span });
                rec.push(Value::Record { cols, vals, span });
            }

            return Ok(Value::List {
                vals: rec,
                span: call.head,
            });
        }

        if let Some(emoj) = param {
            let emoji = replace(&emoj.item).map_err(|op| LabeledError {
                label: "Error in emoji plugin".into(),
                msg: format!("Error in emoji plugin: {}", op),
                span: Some(emoj.span),
            })?;
            return Ok(Value::String {
                val: emoji,
                span: emoj.span,
            });
        } else {
            return Err(LabeledError {
                label: "Expected something from pipeline".into(),
                msg: format!("requires some input, got None"),
                span: Some(call.head),
            });
        }
        // let ret_val = match input {
        //     Value::String { val, span } => {
        //         let emoji = replace(val).map_err(|op| LabeledError {
        //             label: "Error in emoji plugin".into(),
        //             msg: format!("Error in emoji plugin: {}", op),
        //             span: Some(*span),
        //         })?;
        //         Value::String {
        //             val: emoji,
        //             span: *span,
        //         }
        //     }
        //     //crate::emoji::emoji_do_something(param, val, *span)?,
        //     v => {
        //         return Err(LabeledError {
        //             label: "Expected something from pipeline".into(),
        //             msg: format!("requires some input, got {}", v.get_type()),
        //             span: Some(call.head),
        //         });
        //     }
        // };

        // Ok(ret_val)
    }
}

fn main() {
    serve_plugin(&mut Implementation::new(), MsgPackSerializer);
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