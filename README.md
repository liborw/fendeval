# fendeval:

Fendeval is a proof of concept that utilizes the fend_core library to evaluate text files containing calculations. It is a step towards creating a plugin for neovim similar to Calca. I have already developed a neovim plugin called Equals, which can evaluate codeblocks within Markdown files. I have used Equals with Python for numerous projects to incorporate calculations into project documentation. However, Python's main drawback is its lack of built-in support for units, which hinders readability due to the need for conversions and formatting. This project aims to address this issue.


## Examples

Sample file caould look somethig like this
```
This is some text.

a = 1 V
a * 10 mA in Watts #=
```

And there is and output
```
This is some text.

a = 1 V
a * 10 mA in Watts  #= 0.01 watts
```

## Roadmap

 [x] Proof of concept
 [ ] Neovim plugin
 [ ] Highlighting

