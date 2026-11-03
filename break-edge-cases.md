# Break regression checks

Uppercase: first<BR/>second<br />third.

**Bold first<br>bold second**, and `<br>` stays literal.

Trailing break.<br>
Next source line, without an extra blank line.

| Case | Result |
| --- | --- |
| Bold | **first<br>second** |
| Repeated | first<br><br>third |
| Leading/trailing | <br>middle<br> |
| Literal | &lt;br&gt; and `<br>` |

```html
<br> inside fenced code remains literal
```
