---
layout: post
title: Was My CS Degree Worth It?
---

I don't know. My financial aid made it more than worth it for me, because I
didn't pay anywhere near the six figure sticker price - if I had, I don't know
if I would be able to justify that cost.

I certainly gained a lot from it. For what it's worth, the core requirements
(i.e., the CS-specific ones) of my CS degree were as follows:

* Programming and Problem Solving,
* Digital Logic,
* Program Design and Data Structures,
* Discrete Structures,
* Computer Organization,
* Algorithms,
* Intermediate Software Design,
* Programming Languages,
* Principles of Operating Systems, and
* at least three CS electives, one of which must be a "designated project
  course".

It certainly gave me a decent grounding in theory. At the very least, the
curriculum (in theory) enables the student to understand:

* programming concepts (variables, control flow, data structures, OOP, design
  patterns),
* how very basic CPUs, memory, and programs work,
* how all the bells and whistles in a basic computer are managed and used by 
  software (i.e. what an operating system does),
* how to reason about computer code from an abstract perspective, and
* the origins of and decisions behind (some) tools used today.

I have deliberately omitted items such as "how to write code" or "how to design
software" from this list. They don't belong there.

---

To a certain extent, the former ("how to write code") does not belong there
because I don't think that's something you can quite teach - rather, it must be
learned and gained through experience - experience beyond the scope of an
undergraduate education.  I am, admittedly, speaking of "writing code" in a
poetic sense - specifically, how to write good code: comprehensible, elegant,
maintainable, succinct, to name just a few of myriad criteria, many of which are
often at odds with each other.

None of this, however, is ever really taught explicitly. That, I think, is
probably one of the bigger problems: students are assessed point deductions for
violating DRY, perhaps, or because they abused global state and side effects to
get their homework to pass the test suites, but there's no class that teaches
this. These principles are taught in the offhand comments a professor makes in
lecture, in a rambling afterthought inspired by an answer to a novel question.

The latter ("how to design software") suffers from the same challenges, with one
more: the decisions are much more difficult. When writing code, you can make
decisions much more easily: you can compare different approaches based on, say,
performance and memory usage. When designing software - by which I mean the
high-level design decisions, such as the choice of programming language,
monolith vs. microservices, how dependencies are split, whereas I refer to the
low-level design decisions as "writing code" - the metrics are much more opaque,
if quantifiable/qualifiable in any meaningful way at all.

One potential solution that might mitigate these somehow is a course devoted
entirely to case studies and code review of open source projects. Do deep dives
into codebases, studying the various design decisions that were made, and what
might have been if a decision had gone another way. Potential topics include

* design principles such as DRY, SOLID, Liskov substitution, YAGNI, KISS (and
  more importantly, demonstrations of how easy it is to violate these),
* decisions made about splitting code between libraries (Angular and React come
  to mind, e.g. why React Router isn't in React core),
* dependency design and management (e.g. `#include` in C/C++, with `make` and
  `cmake`; JS modules and their precursors, CommonJS and AMD, as well as the
  Yarn vs. NPM mess; Go's acknowledgment of repositories at the language level;
  Python's namespaces, `pip`, and `conda`),
* advantages and disadvantages of high-level abstractions (e.g. LLVM IR makes
  implementing C++17 easier for the `clang` folks, whereas the people behind
  `g++` have to struggle to incorporate new language features, but as a result
  LLVM developers also have to understand both the front-end and back-end),
  and so on.

---

Beyond the above, there are a lot of things that simply aren't covered in any
course in the undergraduate curriculum. I'm not talking about highly technical
topics (e.g. feedback systems, NLP, or crypto), but rather, general skills and
topics that are never explicitly covered:

* Understand what you're learning, and *why* you're learning it.

  Seriously. All of this stuff fits together: Computer Organization teaches you
  why the L1 cache matters; Algorithms teaches you how to reason about expensive
  operations; and so on.

* How to read. Read the documentation. RTFM. Read the error message.

  This may seem obvious. But surprise, surprise: in practice people always mess
  this up, especially when they're not in the habit of doing so. I answered
  countless questions during office hours by pulling up the documentation on
  their screen and Ctrl+F-ing to the answer. Sometimes I'd have to highlight a
  type annotation to get my point across. The same goes for error messages:
  there were plenty of questions that people asked that they could have answered
  on their own by reading the compiler traceback just a little more closely.

* Debugging.

  Copying and pasting from StackOverflow and then randomly choosing from the
  autocomplete menu is not a recipe for success. Reading your code, tracing the
  issue with breakpoints, and identifying the difference between what you expect
  to happen and what is actually happening is.

* Trying stuff out.

  If pointers don't make sense to you, then write code that uses pointers. Use
  the `final` and `static` keyword, see what works and doesn't work, and
  reconcile what happens with the documentation. You can't understand how to use
  something without actually using it, just like how you can't learn to ride a
  bike without actually getting on one.

From a technical standpoint, the list balloons massively:

* how strings work (encoding, decoding, `char` vs. `wchar`, the history of
  Unicode, code points vs. code units vs. glyphs vs. graphemes),
* floating-point advantages and disadvantages (and the purposes of
  double-precision and decimal types),
* optimizing for hardware (e.g. SIMD instruction sets, vectorization),
* real-life optimization concerns (e.g. compute-bound vs. I/O-bound tasks, cache
  misses),
* security concepts and issues, such as
  * best practices re password management (e.g. hashing and salting),
  * common attacks and issues (e.g. timing attacks and null-byte injection),
  * cryptographic protocols (e.g. public key exchange),
* modern Internet technologies (HTTP/2, WebSocket, TLS, HSTS/HPKP, WebAssembly),
* using Linux, particularly the command line,
* shell scripting (ideally by the end of one's undergraduate education they
  would have some notion of proper *defensive* shell scripting techniques - i.e.
  knowing that `set -e` is wildly insufficient - but most students don't even
  know what the hashbang is for),
* how to choose a technology (e.g. Postgres vs. MySQL, React vs. Angular), and
* how to and why choose a platform/tool (e.g. Kubernetes, EC2, S3, Heroku).

Those last two notes are a good segue into another list. In addition to the
aforementioned hard technical skills, there are plenty of important softer
skills that anyone going into software development should train:

* project management: how are bug reports and issues triaged, how are new
  features planned, how is the release cycle managed (Linux and its subsystems,
  for example, or how Rust plans new features),
* basic technical writing: how to report an issue (what level of detail to go
  into, what to mention and what not to mention), and
* interviewing and negotiating: how to interview, manage recruitment timelines,
  timelines, assess job offers (e.g. equity isn't always good - in fact,
  exercising options can be prohibitively expensive), and negotiate.

These are not exhaustive lists, of course. Some topics were deliberately omitted
(because I know there are separate courses that cover them, e.g. compiler
optimizations) and there are undoubtedly plenty more that I didn't think of, to
say nothing of those that I myself haven't learned of yet.

---

The basic point, however, I hope is clear: there are plenty of things that you
will not get from an undergraduate degree - or, more specifically, there are
plenty of things that I did not get from *my* undergraduate education.

That's not to say that my degree is worthless; I do indeed consider my degree
valuable. I gained a lot from my coursework that I wouldn't have in a less
structured environment. The fact of the matter, though, is that going to class
and completing my coursework was, hands-down, not enough. I would be nowhere
near where I am today were it not for the sheer breadth and depth of the
learning I did on my own, *outside* my classes.
