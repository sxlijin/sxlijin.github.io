---
title: Joining Boundary
...

# Joining Boundary

I've been in the industry for about 6 years now: the first 4 at Google
and the past 2 at [Trunk], a series-A DevEx startup that I joined pre-A.

[Trunk]: https://trunk.io/

I realized a few months ago that I wasn't excited in my role anymore, and and
asked myself what I wanted to do. After a bit of looking around, I found that
the answer was that I wanted to transition to AI/ML[^why-ai-ml] - so that's
where I started.

[^why-ai-ml]: Specifically, that we've reached a point where this technology is
    accessible for the average developer, and that we're seeing steady progress
    (sometimes in huge leaps) with LLMs and particularly because we've somehow
    gotten to a point where, between the transformer architecture and Moore's
    law, we can just throw obscene amounts of compute at problems to solve them.

I ended up with two offers: one from [Boundary] and one from OpenAI.

I chose to join [Boundary].

[Boundary]: https://www.boundaryml.com/.

## Why?

The short answer: I was more excited about joining Boundary. I believe in
Vaibhav and Aaron, and I think we have a real chance to do something big.

The long answer: it took weeks to make this decision. Joining OpenAI is, as
Vaibhav put it, the objectively sane decision to make in this situation.

Here's the list of criteria I went through, in roughly the order in which I
thought about them[^criteria-order]:

  * career opportunity
  * product opportunity (both absolute and relative[^absolute-and-relative])
  * timing (both product-in-industry timing and personal timing)
  * culture (people and values)
  * learning opportunity[^learning-opportunity]
  * money
  * location
  * doing something else

[^criteria-order]: When I say the "order in which I thought about them", I do
    mean that - this is not necessarily the order of importance for me. It's
    close to the order of importance, certainly, but I doubt I could actually
    stack rank these criteria against each other.
[^absolute-and-relative]: The way I always describe the $bigco vs. $startup
    decision is that at $bigco, you'll have the opportunity to have huge
    absolute impact, but small relative impact (e.g. the Windows Task Manager is
    an entire team; Chrome Sync is an entire team) whereas at $startup, you'll
    have small absolute impact, but huge relative impact (i.e. maybe you'll get
    to 100K users, but all of those users will know what you've built).
[^learning-opportunity]: No, I don't like putting the third "opportunity"
    further down the list. But this is a more true order for this list.

## Career Opportunity

With Google, my resume's pretty solid. With OpenAI[^openai-role], it would be
bulletproof - especially for future AI/ML opportunities.

[^openai-role]: I would've joined a new team being spun up to build new training
    infrastructure. In other words: as close to the AI/ML research as I can get
    without getting a PhD.

With Boundary, it's a very different career opportunity:

  * It's going to mean defining what the AI/ML developer experience looks like, 5
    years from now.

  * It's going to mean tackling a lot of Hard Problems™, from building a compiler
    to defining our infrastructure patterns to building our brand identity.

  * I know that I can learn a lot from Vaibhav and Aaron, in everything from
    technical instincts to product thinking to leadership skills.

  * It's _not_ going to have household-level brand recognition - plus, there's
    a very real chance that we won't make it.

which leads me to:

## Product Opportunity

Boundary's vision is to be **the** ML development experience that people want:

  * We want to give people the right abstractions to **build** on top of their
    ML models: everything from inline comments that get stripped from your LLM
    prompts to support for Python and Typescript and making it easy to switch
    between ML service providers.

  * We want to enable people to **test** the ML features and products that
    they're building, which is especially important when you're dealing with
    probabilistic systems and defining correctness is harder than enumerating
    edge cases!

  * We want it to be easy to **deploy** changes to your ML features:
    you should be able to both self-host everything that calls an OpenAI API and
    ask us to handle that for you, function-as-a-service style.

  * We want our users to be able to **monitor** their ML usage and ask
    questions about the precision and recall of their deployed model, about
    the costs of the current deployment, and about the reliability of the
    current deployment.

  * We want it to be straightforward to **refine** your ML usage, whether that
    means LLM prompt tuning, fine-tuning an existing open-source model, or
    training a special-purpose model from scratch.

And we think that the right way to do all this is to start with:

  * a freely available, open-source schema language for your ML APIs,
  * code generation for your LLM interactions, and
  * robust, fast, easy-to-use tooling to support every step of the process.

Importantly, this approach has a number of advantages compared to competitors in
the space (I may or may not have put together a spreadsheet at 2am at one point
to assess this):

  * We can offer our users a flexible, end-to-end platform. No one likes stitching
      together 10 products to build their workflow.

  * We don't have lock-in: our schema language, compiler, and IDE integrations
      are all freely available and open-source, so if users want to use just
      those, they're more than welcome to.

  * We can build our platform and ecosystem incrementally. Every platform
      suffers from the critical mass challenge - that you have to build out an
      entire platform for using it to be attractive, and then get enough
      adoption to accrue network effects - but everything that we want to build
      will be independently useful, so we'll be able to respond much more
      quickly to our users as we build out.

  * We're not tied to LLMs: if the winds shift and the industry discovers new
      model architectures, hosting patterns, or whatnot, we'll be
      well-positioned to respond, because our value proposition is giving
      you the right abstractions for your ML APIs. We have a lot of special
      support for working with LLMs, because the existing general-purpose LLMs
      are wildly useful. But there's definitely some insanity to the fact that
      API calls in the LLM world can and do take multiple seconds.

That's the pitch.

It's long - we're still working on it. But with Boundary, I get to be part of
that journey, whereas at OpenAI, they have answers to a lot of these questions
around what to build, how to build them, and an established leadership/executive
team to iterate on those answers. (And I do believe that OpenAI now has
fundamental execution risk around the [innovator's dilemma].)

[innovator's dilemma]: https://en.wikipedia.org/wiki/The_Innovator%27s_Dilemma

### Aside: on other product opportunities

Most other startups are just blatantly uninteresting to me - or if they are
interesting, they're interesting in the academic sense of it, i.e. "yeah, I can
see how this is a cool thing to do".

## Timing

I'm very confident that the products, companies, organizations, and technologies
that will be dominant in 5 years are getting started now. As confident (or as
arrogant!) as someone in their late 20s can be, I guess.

Looking at the past waves that made it big - cloud, mobile, sharing
economies, IoT[^iot], social, PCs, internet - and all those that didn't - VR,
blockchain[^blockchain], no-code, 3D printing - there's very clear value
creation that LLMs specifically have enabled, and the AI spring that we're in
right now feels here to stay.

[^iot]: IoT feels weird in this list. It's unquestionably been huge - not just
    Google Home and Alexa, but Square/Clover point-of-sale terminals, LG smart
    fridges, Samsung laundry machines, Nest thermostats, Ring doorbells - but it
    feels an order of magnitude smaller than, say, mobile. People in developing
    countries who don't have running water have phones - that's how big mobile
    is. IoT, or at least the way I think of it, internet-connected embedded
    devices, is not that big.
[^blockchain]: Oh, yes, digital assets are a thing. But I can't point to a
    single thing in the blockchain space that has created real, substantive
    value beyond novel financial currencies/instruments. And not novel in the
    way that the creation of credit cards and checking accounts enabled safer,
    easier consumer fiance and more efficient capital utilization, but novel in
    the way that mortgage-backed securities were another means for finance
    professionals to allocate capital and generate returns.

The top challenge right now feels mostly like implementation - namely, the fact
that ML operational costs (and, to a similar extent, capex as well) are growing
exponentially as capabilities improve. Every wave that became big was able to to
do so off the back of diminishing costs, either in the form of Moore's law or
network effects.

With OpenAI, the timing factor is that I would be joining as a cog in the
machine. A large cog, yes, because I have no doubt that they're going to
continue growing at a truly insane pace, but still one of many.

### Personal timing

It also helps that I don't have a partner, I don't have kids, and I don't have a
house right now. If any of those change, and I need something more stable and
less risky in the future - well, I don't know if I'll have the risk tolerance
for something like this at that point.

## Culture: people and values

Vaibhav and Aaron are incredibly smart people - I've known them for years now,
and if anyone can pull it off, it's them. I don't say that lightly: I've worked
with a lot of really smart people, and throughout my (still short!) life and
career, I've prioritized surrounding myself with friends and colleagues that
make me a better person, a more capable person, a more thoughtful person.

Regardless of whether or not we succeed, I'll be excited to spend the next
few years working alongside them, learning from them, and trying to change the
world.

We're all clear that this journey is only worth it if we have fun along the way,
if we build an environment where we enjoy each others' company, that we
consider that vital to our success, and that this will require very conscious
and deliberate effort as we grow.

OpenAI is now big enough that they're concerned about leaks.

## Learning Opportunity

One of my favorite things about Google was the sheer amount of learning
opportunities I was surrounded by. I spent time in the guts of Bigtable,
Megastore, Spanner, MapReduce, Flume, Borg, Chubby, GFEs, and more; I loved
being able to go "I wonder how this thing works" and then actually _reading the
design documents that people wrote as they were building the thing_.

This was not a thing that most people did. It's part of the appeal of being at
Google, I think - but most people never take advantage of it. I realized that
about a year in, that one of the reasons I had wanted to join was because of the
learning opportunities. That was when I started sponging up knowledge - about
how YouTube transcoding works, about how our monitoring system dealt with fan-in
problems, about how search was unified across multiple Google products.

I have no doubt I would get this at OpenAI, where I'd be able to avail myself of
world-class experts and being on the inside of impressive technologies at scale.

But at Boundary, I expect my learning to come from, well, shipping. I do have
specific goals for how I want to focus that learning, and if our execution path
ends up at odds with that learning, well, we'll cross that bridge when we get to
it.

## Money

I'm fortunate to do something professionally where I will never have to worry
about making rent, keeping the lights and heat on, or buying groceries. I have
earned more than the median _household_ income in the US since I finished my
education.

Yes, there are luxuries that I could afford and financial security that I'd have
if I took [OpenAI][openai-levels-fyi], but I have plenty enough of it right now
that I'm willing to forego the ludicrous levels of compensation I'd be able to
obtain elsewhere. I know enough people who've optimized for money at the expense
of their time, energy, and youth, that I don't care to follow in their
footsteps.

[openai-levels-fyi]: https://www.levels.fyi/?compare=OpenAI&track=Software%20Engineer

## Location

I'm also super excited to live in Seattle - everyone I know considers it to be
an amazing balance between work and play, where you can surround yourself with
people who care about their careers, but the Cascades are just an hour away, and
Squamish at four hours is close enough for a weekend trip.

Picking up outdoor sports and activities during the pandemic saved my life, and
I deeply value mountain proximity now.

I literally took a detour to San Francisco, when I was driving back to Boulder
from Seattle, as I was deciding between Boundary and OpenAI, to see if I could
find it in me to be excited about being back there - and I couldn't.

This isn't to say San Francisco is a bad place to live - it's a great city, and
I'm sure I could be happy if I moved back there. But I don't know if it'll be a
forever place for me.

(Fortunately, this decision wasn't too hard - Seattle and San Francisco are both
great options. And admittedly, I also asked myself whether OpenAI allowing
remote would change my decision, and it didn't, but for a different pair of
locations and opportunities, the decision calculus probably would've been very
different.)

## Why not be a founder myself?

I don't know if it's a thing I want to do.

Founding means putting 200% of yourself into the business, 24/7, for years - I
wouldn't want to do it without dedicating at least 100%, and that's just not
a balance of priorities that I'm willing to accept for myself at this stage of
my life. Possibly ever.

(There's also a little bit of an activation energy / escape velocity / critical
mass problem, where I'd need an idea that I'm passionate about, a cofounder who
I'm willing to spend years working alongside, and stability in the other
aspects of my life - and I think you only get there if you just dive in
head-first, sink-or-swim style.)

### Or just something else?

I would love to do something like the [Recurse Center] at some point. Or take
more time off. I could, in theory, quit for a year and just gallivant around the
country or the world. But I don't think I would get the same fulfillment out of
any of those as getting back on the wagon again.

[Recurse Center]: https://www.recurse.com/

## Making the decision

It took me weeks to make this decision (and Vaibhav, Aaron, I really appreciate
how long you gave me to make it). I talked to almost everyone I know about it,
tried the trick of "flip a coin and see if you're happy or not".

Ultimately, the thing that finally ended up convincing me was just
semi-committing to one option, and telling friends that I had decided to join
OpenAI. And I just couldn't help but feel like I was going to miss out on
something amazing.

Once I started telling people that I was going to join Boundary - I felt a lot
better about that decision. And if Boundary fails, and we don't make it - as all
the startup statistics suggest - well, at least I'll have tried to do something
big.

## Aside: self-reflection

Throughout this entire process - thinking about what I wanted to do next,
whether I wanted to do _anything_ next, what kinds of things I was qualified to
do - there was also a lot of reflection about myself:

  * what I've achieved
  * what I want to achieve
  * where in my career progression I am
  * what direction I want my progression to take
  * what I want to do with my life

And it was a really valuable experience. Highly recommend, 10/10.
