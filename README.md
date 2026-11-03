# Markdown line-break verification

Evidence for warpdotdev/warp-stanford issue #34 (upstream warpdotdev/warp#13732).

- `before.png`: fresh baseline build at f411fbc20e6283a2abcdd1ed64bf5b67df864f82 renders tags literally.
- `after.png`: the patched macOS arm64 app renders both paragraph and table-cell breaks.
- `after-edge-cases.png`: tag variants, formatting, repeated breaks, and literal tags.
- `after-selection.png`: selecting the word following a break in a bold table cell.
- `after-styled-blocks.png`: fixed debug build renders heading, bullet, numbered-list, and task-list breaks without assertions; continued task items are unchecked.
- The Markdown files are the exact manual fixtures.

Local validation: 682 parser/editor tests passed, including 26 new regression tests; targeted Clippy and the `warp-oss` GUI build passed. The main PR describes the checks and scope. The screenshots are unedited captures of the locally built app.
