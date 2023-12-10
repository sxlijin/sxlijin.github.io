---
title: Some Notes on LLMs
mathjax: yes
...

# Some Notes on LLMs

I had a conversation with a friend a few months ago talking about whether or
not either of us were going to use LLMs on a regular basis, and my answer at the
time was that I didn't think I was going to do much with it, since everything I
was seeing at the time suggested it wasn't sufficiently contextually aware to be
able to help in my everyday work.

In the months since, that answer has changed a fair bit: I reach for ChatGPT
every few days now. Sometimes it's every other day, sometimes it's just once a
week, but I've very much found a useful cadence out of it.

It did require a bit of a shift in mindset, though, and still does. My natural
instinct with a lot of problems is:

  1.  encounter a problem, generally a vague one
  2.  reach for Google to get some background on what options I have for solutions
  3.  choose a solution to implement
  4.  go back to Google to figure out how to implement the thing
  5.  implement the thing, and repeat the last step as many times I need to, to
      arrive at something that (hopefully) solves my problem

I've found that ChatGPT is pretty darn good at that second Google use case - if
I know the rough shape a solution should take, I can craft a sufficiently
precise prompt that will get ChatGPT to spit out a reasonably well-formed
starting point.

To get there, though, I had to switch my own, personal, default behavior from
"just Google it" to "toss it in ChatGPT first, and then fall back to Google".
Now that I've been through that cycle enough times, and am learning what ChatGPT
can do, I'm building a new pattern of my own. As I write this, though, I'm
realizing that maybe I need to repeat this behavior reset, and try using ChatGPT
for more things again.

---

There is one tricky part to all this though: just like you need to craft a query
to get information back from Google Search, you need to craft a prompt to get
answers back from ChatGPT. I'm not even talking about precision or quality or
anything of the sort here: I'm just saying that to get output from either of
them, you need to provide an input.

And crafting that input is, I think, still a very human activity. If I'm
reviewing my company's AWS infrastructure setup, for instance, and resolving a
vulnerability in a resource's configuration, there's no one-size-fits-all answer
for "how do I make this configuration safe" - sometimes there are questions like
"what do we use this resource for? do we actually need this specific resource for
that purpose?". And if you just delete the resource, well - then you don't have
a vulnerable resource anymore.

A lot of problems have solutions like that, where if you start expanding the
scope of the problem, a solution with a very different shape can work just as
well. This is where (2) from the above list came into play - learning about the
things I don't know that I don't know, to figure out what my options are.

That's a thing I haven't really used ChatGPT for yet. In part because I haven't
tried much, partly because it doesn't have the same knowledge discovery
potential as following forum links to related questions and answers, which at
first glance seem unrelated, and at a second glance can sometimes be
surprisingly relevant.

---

Here's the part that I really wanted to write about: "hallucinations". Or as I'm
going to call it, "guessing".

At the end of the day - at least, as I understand it - LLMs are just big black
boxes that hand back output that is a reasonably probable response to the input.
This "probable" criterion is not causally related to any metric of truth,
accuracy, or precision, beyond that human communication tends to be truthful,
accurate, precise, or some combination thereof - how could it, when LLMs are
just really big probability matrices?

And so, as a consequence, LLMs really shine in situations where there's good
correlation between "what is a likely answer to this question" and "what is a
correct answer to this question", but struggle when those two sets start to
diverge.

---

There's an anecdote from a conversation I had with a coworker once, which this
reminds me of: Google Drive Search has a reputation for being "terrible",
whereas Google Search comparatively seems to be a few orders of magnitude
better. And, as he explained to me, it's not because there's any distinction
between the technical qualities of the underlying technologies, but because
Google Search has many more opportunities to be correct than Google Drive
Search does.

If you're looking for something on the public internet, odds are
that there are hundreds or even thousands of things that Google Search can hand
back to you which will reasonably satisfy your query. All Google Search has to
do to make you happy with your query results is find a few of those items and
put them at the top.

By contrast, if you're looking for something in your company's Google Drive, or
just your own Google Drive, there's far fewer opportunities to be correct. Often
you're searching for some specific document, and maybe you don't even remember
enough details about it to actually find it (if you're even remembering the
correct details!)

With ChatGPT, it's a lot of the same (and incidentally, the same goes for things
like horoscopes and fortune cookies and whatnot) - give it a prompt for which
there are many potential correct answers, and there's a good chance you'll get
back one of them, or something close to a correct answer. But if your prompt is
one where it needs to be really specific, well, then it's going to struggle.

---

So here's anecdote #1 of LLM guessing: I found myself nerdsniped by a geometry
question I saw while just browsing around, and realized that I could break it
down into a few trigonometry problems. I figured that rather than spending 5
minutes re-learning triangles (and also to avoid getting off the couch and
grabbing pen and paper), I could just plop each question into ChatGPT,
double-check the work, and move on. Here was one of those sub-problems:

me:

> Given a triangle ABC where AB is 4, BC is 8, and A is 60 degrees, what's the
> length of AC?

ChatGPT 3.5:

> To find the length of side AC in triangle ABC, you can use the law of cosines. The law of cosines states:
>
> $$c^2 = a^2 + b^2 - 2ab \cdot \cos(C)$$
>
> where:
> - $c$ is the length of the side opposite the angle you know (side AC in this case),
> - $a$ and $b$ are the lengths of the other two sides, and
> - $C$ is the angle opposite side $c$.
>
> In your case:
> - $a = AB = 4$
> - $b = BC = 8$
> - $C = \angle A = 60^\circ$
>
> Now, plug these values into the law of cosines [...]

No, ChatGPT, $\angle C$ and $\angle A$ are definitely different angles.

(Incidentally, the rest of the math in that response, after plugging in $a$,
$b$, and $C$, was all correct. I wonder if that says anything about how
frequently math problems look like "evaluate a polynomial for specific integer
values".)

[The rest of the session follows a similar
pattern](https://chat.openai.com/share/b9cefe04-8d20-4638-b24f-cbc0c8f669fa):

  * ChatGPT consistently produces answers that look correct if you don't
    double-check its reasoning or calculations, and
  * it gets a fair amount of the integer math correct, but
  * it makes key mistakes in its "reasoning" along the way:
    * it claims that $AC^2 - 4 \cdot AC - 48 = 0$ implies $(AC - 8)(AC + 6) =
      0$;
    * it says "$8 + 8 > 16$ is not greater than $4$"; and
    * it tells me that "the sine of 120 degrees is $\frac{\sqrt{3}}{2}$ or
      approximately -0.866".

But these are the cherry-picked incorrect steps in ChatGPT's reasoning - the
vast majority of them are actually correct, and if this were your
run-of-the-mill American high school trigonometry class, ChatGPT would do pretty
dang well. It honestly looks a lot like a high school student guessing their way
towards an answer, who knows who to get it, but is just consistently making a
mistake in a key part of their reasoning.

(I read a piece a while back saying that if you start objectively describing
human intelligence as a black box of inputs and outputs, where most of the time
it's correct but every so often it's incorrect, well - that's a description that
sounds remarkably like ChatGPT. And it made a very good case that for a lot of
definitions of human intelligence, it demonstrably does very well at placing
above the 50th percentile. I wish I could find it again - that was one of the
pieces I read that actually shifted my thinking about LLMs.)

---

Here's anecdote #2 of LLM guessing: I was trying to figure out how to write Lua
filters for pandoc, and decided to ask ChatGPT for help since (a) I don't know
Lua and (b) I'm definitely not familiar with the Pandoc Lua filter APIs. In one
of its responses, it suggested a Lua snippet that included this:

```lua
  local entries_list = pandoc.BulletList{}
  for _, entry in ipairs(entries) do
    entries_list:insert(pandoc.Plain(entry))
  end
```

To my untrained eye - keep in mind that I don't know Lua - this looks pretty
reasonable. So I tossed it into a Lua filter, got an "attempt to call a nil value
(method 'insert')" error, and went back to ChatGPT with it:

me:

> The Lua interpreter complains about "attempt to call a nil value (method
> 'insert')" on the entries_list line

ChatGPT 3.5:

> I apologize for the oversight. It seems like there is an issue with the pandoc
> module. The `BulletList` and `Plain` classes may not be available directly in
> a Lua script outside of a Pandoc filter environment. [...]

Well, that was... wrong. The filter was getting past the `pandoc.BulletList{}`
statement, and I was running it in a Pandoc filter environment.

I figured out a bit later, after going back and forth between "Programming in
Lua" and the Pandoc Lua filter API documentation, that `entries_list:insert`
makes no sense, and that what I wanted to write was
`table.insert(entries_list.content, ...)`.

(I'm not super clear on why it's not `entries_list.content:insert(...)` - I have
a plausible-sounding explanation in my head, which has to do with (1) what `a.b`
actually means in Lua when `a` is a table, and (2) how method resolution works
in Lua - but I don't want to go into that much of a tangent here.)

Ignoring that ChatGPT's response didn't work when copy-pasting it into a Lua
filter, though, this was still a remarkably good response:

  * `pandoc` is certainly a popular piece of software, but I'm pretty sure there
    are far fewer people using `pandoc` than there are people who are, say,
    writing Node.js code to read from a SQL database;
  * the population of `pandoc` users writing Lua filters is even smaller;
  * the response does a bunch of things correctly:
    * it uses `pandoc.Plain(STRING)` to construct a pandoc AST element,
    * it uses braces to initialize a Lua object `pandoc.BulletList{}`, and
    * it iterates through `entries` with the right syntax

And besides, not only does `object:method` look like correct Lua code to an
untrained eye, but it is indeed how instance methods are referenced in Lua (or,
that is, Lua's version of them).

---

In both of the LLM guessing anecdotes, if you score the answers using the
criterion "is this a reasonably probable answer for the specified prompt", well,
they're all answers I can see a human giving me. It still continues to blow my
mind how much intersection there is between the set of "reasonably probable
answers" and "useful answers" though, particularly in situations like that of
anecdote #2 where I'm dealing with more esoteric APIs.
