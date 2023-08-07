# nu_plugin_emoji

This nushell plugin wraps the `emojis` create in an effort to help create emojis more easily.

## Usage:

```
❯ emoji --help
Create emojis from text

Usage:
  > emoji {flags} (emoji-name)

Flags:
  -h, --help - Display the help message for this command
  -l, --list - List stuff

Parameters:
  emoji-name <string>: name of the emoji shorthand with colons before and after e.g. :grinning: (optional)

Examples:
  This is the example descripion
  > some pipeline involving emoji
```

```
❯ emoji :wave:
👋
```

```
❯ emoji --list | where shortcodes =~ wave
╭─#─┬emoji┬────name─────┬unicode_version┬─────group─────┬────utf8_bytes────┬codepoints┬shortcodes┬────────────skin_tones────────────╮
│ 0 │ 👋  │ waving hand │ 0.6           │ PeopleAndBody │ [F0, 9F, 91, 8B] │ 1F44B    │ wave     │ 👋, 👋🏻, 👋🏼, 👋🏽, 👋🏾, 👋🏿 │
╰───┴─────┴─────────────┴───────────────┴───────────────┴──────────────────┴──────────┴──────────┴──────────────────────────────────╯
```

```
❯ emoji --list | first 20
╭─#──┬─emo┬──────────────name──────────────┬─unico┬───────group───────┬────────utf8_bytes────────┬─codepoints┬───────────shortcodes───────────┬─s╮
│ 0  │ 😀 │ grinning face                  │ 1.0  │ SmileysAndEmotion │ [F0, 9F, 98, 80]         │ 1F600     │ grinning                       │  │
│ 1  │ 😃 │ grinning face with big eyes    │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 83]         │ 1F603     │ smiley                         │  │
│ 2  │ 😄 │ grinning face with smiling     │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 84]         │ 1F604     │ smile                          │  │
│    │    │ eyes                           │      │                   │                          │           │                                │  │
│ 3  │ 😁 │ beaming face with smiling eyes │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 81]         │ 1F601     │ grin                           │  │
│ 4  │ 😆 │ grinning squinting face        │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 86]         │ 1F606     │ laughing, satisfied            │  │
│ 5  │ 😅 │ grinning face with sweat       │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 85]         │ 1F605     │ sweat_smile                    │  │
│ 6  │ 🤣 │ rolling on the floor laughing  │ 3.0  │ SmileysAndEmotion │ [F0, 9F, A4, A3]         │ 1F923     │ rofl                           │  │
│ 7  │ 😂 │ face with tears of joy         │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 82]         │ 1F602     │ joy                            │  │
│ 8  │ 🙂 │ slightly smiling face          │ 1.0  │ SmileysAndEmotion │ [F0, 9F, 99, 82]         │ 1F642     │ slightly_smiling_face          │  │
│ 9  │ 🙃 │ upside-down face               │ 1.0  │ SmileysAndEmotion │ [F0, 9F, 99, 83]         │ 1F643     │ upside_down_face               │  │
│ 10 │ 🫠 │ melting face                   │ 14.0 │ SmileysAndEmotion │ [F0, 9F, AB, A0]         │ 1FAE0     │ melting_face                   │  │
│ 11 │ 😉 │ winking face                   │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 89]         │ 1F609     │ wink                           │  │
│ 12 │ 😊 │ smiling face with smiling eyes │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 8A]         │ 1F60A     │ blush                          │  │
│ 13 │ 😇 │ smiling face with halo         │ 1.0  │ SmileysAndEmotion │ [F0, 9F, 98, 87]         │ 1F607     │ innocent                       │  │
│ 14 │ 🥰 │ smiling face with hearts       │ 11.0 │ SmileysAndEmotion │ [F0, 9F, A5, B0]         │ 1F970     │ smiling_face_with_three_hearts │  │
│ 15 │ 😍 │ smiling face with heart-eyes   │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 8D]         │ 1F60D     │ heart_eyes                     │  │
│ 16 │ 🤩 │ star-struck                    │ 5.0  │ SmileysAndEmotion │ [F0, 9F, A4, A9]         │ 1F929     │ star_struck                    │  │
│ 17 │ 😘 │ face blowing a kiss            │ 0.6  │ SmileysAndEmotion │ [F0, 9F, 98, 98]         │ 1F618     │ kissing_heart                  │  │
│ 18 │ 😗 │ kissing face                   │ 1.0  │ SmileysAndEmotion │ [F0, 9F, 98, 97]         │ 1F617     │ kissing                        │  │
│ 19 │ ☺️  │ smiling face                   │ 0.6  │ SmileysAndEmotion │ [E2, 98, BA, EF, B8, 8F] │ 263A FE0F │ relaxed                        │  │
╰─#──┴─emo┴──────────────name──────────────┴─unico┴───────group───────┴────────utf8_bytes────────┴─codepoints┴───────────shortcodes───────────┴─s╯
```
