Highlight Regular Expression Pattern
====================================

Is invoked with one or more regex patterns and text styles as arguments.  Reads text from
standard input and writes highlighted text to STDOUT. Text is highlighted using terminal 
control codes.


Example Usage
-------------

The message will render letters with an underline and digits in bold with red foreground:

```
echo "'Bob' has 1000000 dollars" | hrep -i -e '[a-z]' -s underline -e '[0-9]' -s 'bold,f=red'
```

Text Styles
-----------

Styles are lists of attributes separated by commas. An attribute may be one of the following:

* A highlight: `bold`, `dimmed`, `intense`, `italic`, `strikethrough`, or `underline`.
* A background color e.g. `b=cyan`.
* A foreground color, e.g. `f=red`.

If an attribute does not match a highlight and also does not begin with `b=` or `f=`, it will
be assumed to be a foreground color.

Colors can be specified either by name, or with an ANSI color number (0-255).  Available color
names: `black`, `blue`, `green`, `red`, `cyan`, `magenta`, `yellow`, and `white`.

Example Text Styles
-------------------

* `bold,red` - produces text that is bold and the foreground color is red.
* `f=black,b=128` - produces text that has a black foreground color and background that is ANSI
  color code 128 (magenta/purple).
* `strikethrough,b=green` - produces text that has a green background color and the 
  strike-through highlight.

