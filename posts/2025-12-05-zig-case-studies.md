# Projects that use Zig

In much the same vein as my Rust post, I intend to keep a running log of Zig case studies here.

## Bun
Source: [Unsafe Rust vs. Zig (2023)](https://zackoverflow.dev/writing/unsafe-rust-vs-zig/)

Bun is an alternative JS interpreter, known for substantial performance gains over V8.

> After spending a lot of time practicing the dark arts in Rust, I was excited to leave unsafe Rust and learn Zig and start rewriting the project in it.
>
> Apart from not having crazy UB like in unsafe Rust, Zig is a language that understands that you are going to be doing memory-unsafe things, so its designed and optimized around making that experience much better and less error prone. These were some key things that helped:

# TigerBeetle
Source: [Zig and Rust (2023)](https://matklad.github.io/2023/03/26/zig-and-rust.html)

TigerBeetle is a database. Per their docs, it is "the financial transactions database designed for mission critical safety and performance to power the next 30 years of OLTP."

> Zig _is_ a much smaller language than Rust. Although you’ll _have_ to be able to keep the entirety of the program in your head, to control heaven and earth to not mess up resource management, doing that could be easier.
>
> It’s not true that rewriting a Rust program in Zig would make it simpler. On the contrary, I expect the result to be significantly more complex (and segfaulty). I noticed that a lot of Zig code written in “let’s replace [RAII](https://doc.rust-lang.org/rust-by-example/scope/raii.html) with [defer](https://ziglang.org/documentation/master/#defer)” style has resource-management bugs.

# LightPanda
Source: [Why We Built Lightpanda in Zig (2025)](https://lightpanda.io/blog/posts/why-we-built-lightpanda-in-zig)

LightPanda is a web browser.

> To be honest, when I began working on Lightpanda, I chose Zig because I’m not smart enough to build a big project in C++ or Rust.
>
> I like simple languages. I like Zig for the same reasons I like Go, C, and the KISS principle. Not just because I believe in this philosophy, but because I’m not capable of handling complicated abstractions at scale.

## Ghostty
Source: [We ain't afraid of no Ghostty! (2024)](https://changelog.com/podcast/622)

Ghostty is a performant terminal emulator.

> The way I’d describe it philosophically, and as a technical achievement, I have absolutely nothing but respect, and I’m impressed by Rust. I think it’s very impressive. But as a personal basis, it’s very superficial. When I write and read Rust, I’m not having fun. And I want to have fun, and part of the joy is writing the code… And it’s very much a stylistic choice.
> 
> I hate to put it in that perspective, because I think engineers want some sort of concrete, objective reason of why one versus another is better… It’s really a vanilla versus strawberry ice cream flavor sort of thing for me. They’re both great, they both are edible… But I choose one over the other. That’s really what it came down to for me.

# Thoughtful posts about Zig

[Thoughts on Go vs. Rust vs. Zig (2025)](https://sinclairtarget.com/blog/2025/08/thoughts-on-go-vs.-rust-vs.-zig/)
> I’ve collected here my impressions of the three languages I’ve experimented with lately: Go, Rust, and Zig. I’ve tried to synthesize my experience with each language into a sweeping verdict on what that language values and how well it executes on those values. This might be reductive, but, like, crystallizing a set of reductive prejudices is sort of what I’m trying to do here.
