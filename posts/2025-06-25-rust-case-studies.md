# Rust Case Studies

Audience: Rust-curious technical decision-makers.

You're here because you've heard some good things about Rust, but it's all assorted anecdotes, and you'd like something a bit more comprehensive, in-depth, and ideally _neutral_, about the reasons that people choose to use Rust, and what context informs those reasons.

Here are some case studies, describing experiences from teams at both small and large companies, that will hopefully be useful to you:

- [Migrating to Rust](#migrating-to-rust): projects that rewrote large existing codebases in Rust
- [Adopting Rust](#adopting-rust): orgs that regularly use Rust for new projects
- [Decided Against Rust](#decided-against-rust): teams that have tried and rejected Rust

Please [let me know](mailto:me@sxlijin.com) if there is an article, blog, or other case study I should add.

# Migrating to Rust

## Statsig: Node/Python/Elixir/Java -> Rust
Source: [Escaping SDK maintenance hell with a core Rust engine (2025)](https://statsig.com/blog/escaping-sdk-maintenance-hell/)

What
- Statsig is a feature flag / experimentation SaaS platform, and provides users with SDKs in whatever language their app is built in.
> At Statsig, we have over 24 SDKs our customers use to log events and run experiments—and a team of just 7 devs to maintain them.

Why
> Three years ago, we only supported two server languages: Node and Go. Today, we support _ten_. Before this Rust core project, each of our server SDKs had to duplicate necessary evaluation logic in their own language [...] With this new approach—we’re calling it Server Core—the goal is to only need to write evaluation logic once in Rust. We’re rewriting language-specific SDKs to simply be bindings that pass data into that centralized Rust core.

## OpenAI, Codex: Typescript → Rust
Source: [Codex CLI is going native (2025)](https://github.com/openai/codex/discussions/1174)

(Note: this migration is ongoing at the time of this article.)

Why
> - **Zero-dependency Install** — currently Node v22+ is required, which is frustrating or a blocker for some users
> - **Native Security Bindings** — surprise! we already ship a Rust for linux sandboxing since the bindings were available
> - **Optimized Performance** — no runtime garbage collection, resulting in lower memory consumption
> - **Extensible Protocol** — we've been working on a "wire protocol" for Codex CLI to allow developers to extend the agent in different languages (including Type/JavaScript, Python, etc) and MCPs ([already supported in Rust](https://github.com/openai/codex/tree/main/codex-rs#mcp_servers))

## AWS, Aurora DSQL: C → Rust, Java/Kotlin → Rust
Source: [Just make it scale: An Aurora DSQL story (2025)](https://www.allthingsdistributed.com/2025/05/just-make-it-scale-an-aurora-dsql-story.html)

What
- Aurora DSQL is a new Postgres-compatible cloud database service from AWS
- DSQL data plane: writes database rows to disk and reads them back
- DSQL control plane: puts customer A's data in one place and customer B's data in another place; also changes server configuration on the fly (autoscaling, cluster topology)
- Postgres extensions: DSQL integrations with the Postgres query engine

Why, DSQL data plane
> To validate our concerns [about GC pauses], we ran simulation testing of the system [...] instead of achieving the expected million TPS in the crossbar simulation, we were only hitting about 6,000 TPS [...] This wasn’t just an edge case - it was fundamental to our architecture.

> We assigned two engineers to the project. They had never written C, C++, or Rust before. [But after a few weeks, the Rust version] was 10x faster than our carefully tuned Kotlin implementation – despite no attempt to make it faster. To put this in perspective, we had spent years incrementally improving the Kotlin version from 2,000 to 3,000 transactions per second (TPS). The Rust version, written by Java developers who were new to the language, clocked 30,000 TPS.

Why, DSQL control plane
> Kotlin seemed like the obvious choice [...] The benefits we saw with Rust in the data plane – throughput, latency, memory safety – weren’t as critical here. [...] It also turned out to be the wrong one [because] the control plane has to share some amount of logic with the data plane

Why, Postgres extensions
> Initially, the team felt C was a better choice. We already had to read and understand C to work with Postgres, and it would offer a lower impedance mismatch. As the work progressed though, we realized a critical flaw in this thinking. The Postgres C code is reliable: it’s been thoroughly battled tested over the years. But our extensions were freshly written, and every new line of C code was a chance to add some kind of memory safety bug, like a use-after-free or buffer overrun.

Also from Amazon:
- Firecracker, the AWS Lambda hypervisor ([FAQ](https://github.com/firecracker-microvm/firecracker/blob/main/FAQ.md))
- [Rebuilding Prime Video UI with Rust and WebAssembly (2025)](https://www.infoq.com/presentations/prime-video-rust/)

## Microsoft, many projects: C++ → Rust
Source: [Microsoft - Victor Ciura, Principal Engineer (2025)](https://corrode.dev/podcast/s04e01-microsoft/)

What
- [Pluton](https://learn.microsoft.com/en-us/windows/security/hardware-security/pluton/microsoft-pluton-security-processor), [Mu](https://microsoft.github.io/mu/), [Caliptra](https://github.com/chipsalliance/Caliptra): secure firmware primitives for Windows
- HSM firmware, [Hyperlight](https://opensource.microsoft.com/blog/2024/11/07/introducing-hyperlight-virtual-machine-based-security-for-functions-at-scale/): lightweight Virtual Machine Manager
- [DirectWrite](https://learn.microsoft.com/en-us/windows/win32/directwrite/dwritecore-overview): Windows text rendering
- "There's been a bunch of incubation projects across the company in all organizations, not just Azure. In Windows, in Office, in Azure, in M365, all across the company, really."

Why
> We're doing these tactical Rust migrations or Rust rewrites, where we're identifying pieces of code or components that have been traditionally targeted for vulnerabilities over the years [...] rewriting them in Rust [...] to be able to prove their security, as in not just "oh, it's Rust, it must be secure" - rewriting them in Rust gives us ways to reason about proving their safety.

> Memory safety dominates all CVEs across Microsoft, it's definitely over 70 percent [...] we're constantly developing newer and newer technologies for C++ to address this. But with Rust, you get most of these things out of the box, right?

Lessons
> People coming from C++ don't complain about compile times [...] only people coming from C# and .NET complain about compile times.

> Some teams care a lot about high fidelity language interop, right? And some teams care a lot about sort of the glue code and the automation of generating the glue code for the interop. [...] it's clunky for Rust to provide any kind of ABI stability and resilience.

> Success very much depends on if we can [make it] easy to cross from C++ or from C# in our case. [Another] top problem here is fitting cargo in existing build systems, right - how do you fit cargo in, let's say, a cmake project [...]

## Vercel, Turborepo: Go → Rust
Sources:
- [Finishing Turborepo's migration from Go to Rust (2024)](https://vercel.com/blog/finishing-turborepos-migration-from-go-to-rust)
- [How Turborepo is porting from Go to Rust (2023)](https://vercel.com/blog/how-turborepo-is-porting-from-go-to-rust)
- [Why Turborepo is migrating from Go to Rust (2023)](https://vercel.com/blog/turborepo-migration-go-rust)

What
> We managed to port about 70k lines of Go to Rust in 15 months with minimal disruption to users. Our porting strategies such as the Rust shim and the Go sandwich allowed us to incrementally port code for as long as possible.

Why
> Go's strength is network computing in data centers [...] The Rust language and community has prioritized correctness over API abstraction—a tradeoff that we care a lot about [...]

> [For example:] Go lets users set a Unix-style file permission code [but also allows] us to set a file permission code on Windows, even when doing so will have no effect. [...] If you want to set a file permission code in Rust, you have to explicitly annotate the code as Unix-only. If you don't, the code won't even compile on Windows.

> great perk: our team **wants** to write Rust. It's a language that solves what we care about and brings us joy. The fact that we enjoy writing Rust is valuable, by itself [...] Happier developers deliver better software.

Also from Vercel: [Vercel Functions are now faster—and powered by Rust (2024)](https://vercel.com/blog/vercel-functions-are-now-faster-and-powered-by-rust)

## Shopify, YJIT: C → Rust
Source: [Our Experience Porting the YJIT Ruby Compiler to Rust (2022)](https://shopify.engineering/porting-yjit-ruby-compiler-to-rust)

What
> The YJIT team has six developers, with four people taking an active part in the Rust porting effort. I was one of the founding members and have been acting as team lead for 18 months. I’ve been programming for 24 years and got started by learning C++ in 1998 [...] I’ve only been programming in Rust for four months.

> YJIT is a relatively simple JIT compiler that totals about 11,000 lines of C code. [...] It took us three months to complete the port of YJIT from C to Rust, and we feel very satisfied with the result. [The end product](https://github.com/ruby/ruby/tree/master/yjit) is, in my opinion, much more maintainable and pleasant to work with than the original C codebase.

Why
> C doesn’t provide many tools to manage complexity. [For example:] There are no classes or interface types to cleanly encapsulate functionality, and there are no standard container types. We implemented our own dynamic array type, which we had to manipulate through awkward preprocessor macros with no type checking. [...]

> One of the best features of the Rust language is the ML-inspired pattern-matching syntax. It’s simple, powerful, and meshes well with Rust’s enum (tagged union) and struct types.
>
> The Rust macro system is a huge improvement over C preprocessor macros, in terms of both safety and ergonomics. The macros nicely reuse the Rust pattern-matching syntax so that you can define how code is generated in a way that feels fairly intuitive and natural.

> The [build system] for conditional compilation works really well; it's a lot better than having a large number of preprocessor ifdefs in a C codebase. The ability to embed tests into the source code is also very nice.

## 1Password, Core: Go → Rust
Sources: 
- [Behind the scenes of 1Password for Linux (2021)](https://dteare.medium.com/behind-the-scenes-of-1password-for-linux-d59b19143a23)
- [1Password 8: The Story So Far (2021)](https://blog.1password.com/1password-8-the-story-so-far/)

What
> The goal was to replace those four separate technology stacks [(macOS, iOS, Windows, Android)] — each with their own idiosyncrasies, differences, and frankly, bugs — with something that allowed us to move faster, together. With a couple false starts and technology changes under our belts we finally caught our stride at the beginning of last year. A small team, using existing pieces of various apps and projects, put together a proof of concept of a brand new 1Password app running on top of what we now call the 1Password Core.

Why
> The team was using Go lang to create [the] shared backend [...] with [GopherJS](https://github.com/gopherjs/gopherjs) to compile to JavaScript [but] we outgrew GopherJS. When we started experimenting with WASM we discovered that Rust compiles to WASM incredibly well and has a solid toolchain we had to try it out.

> It also ticked all the boxes for the platforms to which we were planning to deploy: macOS, iOS, Windows, Android, Linux, our browser extension, and our web app.

## Discord, Read States service: Go → Rust
Source: [Why Discord is switching from Go to Rust (2020)](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)

What
> The service we switched from Go to Rust is the “Read States” service. Its sole purpose is to keep track of which channels and messages you have read. Read States is accessed every time you connect to Discord, every time a message is sent and every time a message is read. In short, Read States is in the hot path. We want to make sure Discord feels super snappy all the time, so we need to make sure Read States is quick.

Why
> It was fast most of the time, but every few minutes we saw large latency spikes that were bad for user experience. After investigating, we determined the spikes were due to core Go features: its memory model and garbage collector (GC).

> Go will force a garbage collection run every 2 minutes at minimum. [...]  in the Rust version of the Read States service, when a user’s Read State is evicted from the LRU cache it is immediately freed from memory. [...]
>
> At the time this service was reimplemented, Rust stable did not have a very good story for asynchronous Rust. [...] As an engineering team, we decided it was worth using nightly Rust and we committed to running on nightly until async was fully supported on stable. [...]
> 
> The actual rewrite was fairly straight forward. It started as a rough translation, then we slimmed it down where it made sense. For instance, Rust has a great type system with extensive support for generics, so we could throw out Go code that existed simply due to lack of generics. Also, Rust’s memory model is able to reason about memory safety across threads, so we were able to throw away some of the manual cross-goroutine memory protection that was required in Go.

## Figma, Multiplayer Service: TS → Rust
Source: [Rust in production at Figma (2018)](https://www.figma.com/blog/rust-in-production-at-figma/)

What
> Our multiplayer syncing engine [lets everyone] see each change made to a Figma document in real time.

Why
> The main problem with the old server was the unpredictable latency spikes during syncing. The server was written in TypeScript and, being single-threaded, couldn’t process operations in parallel. [...] And we couldn’t just create a separate node.js process for every document because the memory overhead of the JavaScript VM would have been too high.

> ![](https://cdn.sanity.io/images/599r6htc/regionalized/aef8d9f50f52218911be208167b18dac8966d646-800x458.png?w=804&q=75&fit=max&auto=format&dpr=2)

> Pros: low memory usage, awesome performance, solid toolchain, friendly error messages
> Cons: lifetimes are confusing, errors are hard to debug, many libraries are still early, asynchronous Rust is difficult

## Dropbox, Magic Pocket: Go → Rust
Sources:
- [The Epic Story of Dropbox's Exodus From the Amazon Cloud Empire (2016)](https://www.wired.com/2016/03/epic-story-dropboxs-exodus-amazon-cloud-empire/)
- [jamwt on Hacker News (2016)](https://news.ycombinator.com/item?id=11283688)

What
> Over the last two-and-a-half years, Dropbox [...] built their own Amazon S3.

Why
> One of the games in this project is optimizing how little memory and compute you can use to manage 1GB (or 1PB) of data. [...] It's much easier to do these particular kinds of optimizations using C++ or Rust [instead of in Go].

> The advantages of Rust are many. Really powerful abstractions, no null, no segfaults, no leaks, yet C-like performance and control over memory and you can use that whole C/C++ bag of optimization tricks.

## Mozilla, Firefox: C++ → Rust
Sources:
- [Implications of Rewriting a Browser Component in Rust (2019)](https://hacks.mozilla.org/2019/02/rewriting-a-browser-component-in-rust/)
- [Shipping Rust in Firefox (2016)](https://hacks.mozilla.org/2016/07/shipping-rust-in-firefox/)

Mozilla deserves a special mention as the organization that first sponsored and drove a major production use case for Rust.


What: CSS Engine
> The style component is the part of a browser that applies CSS rules to a page. [...] By 2017, Mozilla had made two previous attempts to parallelize the style system using C++. Both had failed.

Why
> With Rust, you can statically verify that you don’t have data races. This means you avoid tricky-to-debug bugs by just not letting them into your code in the first place. The compiler won’t let you do it.

# Adopting Rust

## Google, many projects
Sources:
- [Beyond Safety and Speed: How Rust Fuels Team Productivity (2024)](https://www.youtube.com/watch?v=QrrH2lcl9ew)
- [Eliminating Memory Safety Vulnerabilities at the Source (2024)](https://security.googleblog.com/2024/09/eliminating-memory-safety-vulnerabilities-Android.html)
- [Supporting the Use of Rust in the Chromium Project (2023)](https://security.googleblog.com/2023/01/supporting-use-of-rust-in-chromium.html)
- [Rust fact vs. fiction: 5 Insights from Google's Rust journey in 2022](https://opensource.googleblog.com/2023/06/rust-fact-vs-fiction-5-insights-from-googles-rust-journey-2022.html)

To my knowledge, Google has not yet discussed the _migration_ of any brownfield project to Rust; its publicly stated priority for Rust in Android is "not to convert existing C/C++ to Rust, but rather to shift development of new code to memory safe languages over time" - hence Google is classified under *adoption*.

Productivity
> Rust teams at Google are as productive as ones using Go, and more than twice as productive as teams using C++. [...] We see reduced memory usage in the services that we've moved from Go [...] and a decreased defect rate over time in the services that've been rewritten in Rust.

>1/3 of respondents become as productive using Rust as other languages in 2 months or less. More than half of respondents say that Rust code is easier to review than other languages.

Android
>Memory safety issues, which accounted for 76% of Android vulnerabilities in 2019, and are currently 24% in 2024, well below the 70% industry norm, and continuing to drop.

Chromium
>Our goal in bringing Rust into Chromium is to **provide a simpler** (no IPC) and **safer** (less complex C++ overall, no memory safety bugs in a sandbox either) **way to satisfy [the rule of two](https://chromium.googlesource.com/chromium/src/+/master/docs/security/rule-of-2.md), in order to speed up development** (less code to write, less design docs, less security review) **and improve the security** (increasing the number of lines of code without memory safety bugs, decreasing the bug density of code) **of Chrome**. And we believe that we can use third-party Rust libraries to work toward this goal.

## Cloudflare, many projects 
Sources:
- [How we built Pingora, the proxy that connects Cloudflare to the Internet (2022)](https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/)
- [r/rust: How much Rust work is actually going on at Cloudflare? (2023)](https://www.reddit.com/r/rust/comments/10c0re0/how_much_rust_work_is_actually_going_on_at/)
- [Oxy is Cloudflare's Rust-based next generation proxy framework (2023)](https://blog.cloudflare.com/introducing-oxy/)
- [Introducing Foundations - our open source Rust service foundation library (2024)](https://blog.cloudflare.com/introducing-foundations-our-open-source-rust-service-foundation-library/)

>There is a lot of Rust at Cloudflare. All of our DDoS detection is done with Rust. I also believe that most/all of our DNS stack is Rust.
>
>Anecdotally, it seems like Rust has become the de-facto choice for new/greenfield software over the last three-ish years. Five years ago when I joined, the de-facto choice was Go. We still have lots of Go, but it seems like nobody is actively choosing Go anymore.

# Decided Against Rust
## Prisma ORM
Sources: [The Complete Rust-to-Typescript Migration Journey (2025)](https://www.prisma.io/blog/series/prisma-orm-the-complete-rust-to-typescript-migration-journey)

What
>Prisma ORM is a Node.js and TypeScript ORM with an intuitive data model, automated migrations, type-safety, and auto-completion.

>We detail the engineering rationale for switching to a TypeScript/WASM core, deliver 3.4x faster query benchmarks, and show how this change (with a 90% smaller bundle) made Prisma ORM production-ready for Serverless/Edge deployments.

Why
>When we started [...] we had a pretty clear vision: we wanted to build ORMs for as many languages as possible—TypeScript, Go, Python, Scala, Rust, and others.
>
>[...] the ability to support multiple clients is no longer our focus. Prisma ORM is a TypeScript project [...] we won't be developing [our community clients] in-house.

>While having a powerful Rust engine helped us deliver great performance quickly, we’ve since discovered that it creates some notable challenges:
>
> - **Skillset barriers**: Contributing to the query engine requires a combination of Rust and TypeScript proficiency, reducing the opportunity for community involvement.
>
> - **Deployment complexity**: Each operating system and OpenSSL library version needs its own binary, complicating deployments and slowing down development.
>
> - **Compatibility issues**: Modern JavaScript runtimes, serverless, and edge environments aren’t always compatible with large Rust binaries, limiting how and where Prisma can be deployed.
>

## Microsoft, TypeScript compiler: TS → Go
Sources: [Why Go? (2025)](https://github.com/microsoft/typescript-go/discussions/411), [TypeScript is being ported to Go (2025)](https://www.youtube.com/watch?v=10qowKUW82U&t=768s)

What
>We’ve begun work on a native port of the TypeScript compiler and tools. The native implementation will drastically improve editor startup, reduce most build times by 10x, and substantially reduce memory usage.

Why
>Languages that allow for a structurally similar codebase offer a significant boon for anyone making code changes because we can easily port changes between the two codebases. In contrast, languages that require fundamental rethinking of memory management [etc. do not]. Idiomatic Go strongly resembles the existing coding patterns of the TypeScript codebase, which makes this porting effort much more tractable.

>When you have a product that has been in use for more than a decade [...] you are going to be faced with the longest tail of incompatibilities you could imagine. So, from the get-go, we knew that the only way this was going to be meaningful was if we ported the existing code base. The existing code base makes certain assumptions -- specifically, it assumes that there is automatic garbage collection -- and that pretty much limited our choices. That heavily ruled out Rust. [...] In particular, [the borrow checker] effectively outlaws cyclic data structures, and all of our data structures are heavily cyclic.

## LogLog Games
Source: [Leaving Rust gamedev after 3 years (2024)](https://loglog.games/blog/leaving-rust-gamedev/)

>It's my personal opinion after trying to make Rust gamedev work for us, a small indie developer (2 people), trying to make enough money to fund our development with it. [...] We're well over 100k lines of Rust code written over 3+ years.

>The most fundamental issue is that the borrow checker _forces_ a refactor at the most inconvenient times. [...] Rust is also a language that will force the user to refactor much more often than other languages. [...] What would be 3 lines of code in C# suddenly becomes 30 lines of Rust split into two places. 

## PropelAuth (seed-stage, YC W22), hypothetical
Source: [I love building a startup in Rust. I wouldn't pick it again. (2023)](https://www.propelauth.com/post/i-love-building-a-startup-in-rust-i-wouldnt-pick-it-again)

>This advice is primarily for early startups (pre-product/pre-seed/seed) [...]
>
>What it really boils down to is that Rust is a great language for building performant production systems. If you are prototyping, iterating with customers, or just unsure in the product direction - you are probably going to waste valuable time with Rust.
>
>Once you have that clarity, and you’ve shifted to “It’s time to scale” mode or “Perf actually is really important here” mode or even “We have a solid direction/roadmap and we don’t want too much tech debt” mode, Rust is a fantastic choice.

# Appendix
## Other success stories
- Volvo, ECUs for EX90 and Polestar3: https://corrode.dev/podcast/s03e08-volvo/, https://www.youtube.com/watch?v=2JIFUk4f0iE
- Astral, `uv` and `ruff`
- Linkerd: Go → Rust  https://linkerd.io/2020/07/23/under-the-hood-of-linkerds-state-of-the-art-rust-proxy-linkerd2-proxy/
- Okta Workflows were built with Tokio pre-1.0 and then migrated to use `async` and `await`: https://www.okta.com/blog/2024/11/migrating-off-legacy-tokio-at-scale/

## Other lists
There are a few other lists, but none of them really provide any kind of meta-analysis or discuss the reasons that people _don't_ choose Rust.

- https://github.com/ImplFerris/rust-in-production
- https://github.com/omarabid/rust-companies