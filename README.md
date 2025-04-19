Highlight Regular Expression Pattern
====================================

Specify one or more patterns, and highlight matching portions of text from input.
Also specify corresponding highlight colors as desired.


Example Usage
-------------

```
diff -U 3 old.c new.c | hrep -c red -e "^-.*" -c green -e "^\+.*" -c yellow -e "^@@.*@@"
```

Color Specification Strings
---------------------------

The argument to a `--color` / `-c` option is a *Color Specificiation String*.  Examples of
Color Specification Strings:

* `fg=red+bold,bg=blue` (specifying both goregound and background attributes)
* `green` (specifying just a color changes text foreground color)


