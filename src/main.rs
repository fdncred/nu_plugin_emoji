mod emoji;
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Spanned, SyntaxShape, Value};
use std::io::Write;

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
            .required(
                "emoji-name",
                SyntaxShape::String,
                "name of the emoji shorthand with colons before and after",
            )
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
