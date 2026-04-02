# nano-templater
A really simple templater in Rust

## Idea

As it was said that is a dead simple template engine (if it could be called that way).
It has two "stages":
+ compilation
+ render (but it is called format in the library itself)

At the compilation stage template file is being splitted into chunks, up to the substituted parts, and after that the resulting string is being constructed from the chunks and substituted parts. 
