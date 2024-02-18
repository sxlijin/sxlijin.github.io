---
title: "Lessons Learned: From Big Tech to Startup"
...

# Lessons Learned: From Big Tech to Startup

I started my career with a 4-year stint at Google, regularly dealing with orgs
of 60 to 100 people, and then spent the next 2.5 years at a [seed/series-A
startup](https://trunk.io). Here are some lessons I learned in the process.

(Disclaimer: this is very stream-of-consciousness. It's only been lightly edited
for grammar, and most of these lessons are highly dependent on the context in
which I learned them.)

## Time estimation is even more important.

In $bigco, my time estimation skills boiled down to "how long do I think it
should take? then double it" - which is fine in $bigco, where so much time is
lost to sheer organizational friction that this was OK. In a startup, time is
much more finite.

Also, if you minimize org friction (e.g.  code review time, stakeholder feedback
turnaround) in a fast-paced environment, then at least you can throw more time
at a problem to solve it. (That we _had_ organizational friction at such a small
company was... a different problem.)

## Iteration cycles should be as fast as possible.

The other two engineers on my team wasted way too much time figuring out how to
iterate on webhook handler implementations, and I should've taken a week to
shave that time down from O(hours) to O(seconds), given that this was a pain
point we dealt with on and off for months. (I'm exaggerating slightly, and we
had a better process when we started; I just didn't notice when that process
broke.)

Even spending a week to take webhook handler iteration from 1-minute-cycles to
5-second-cycles would've been a good use of my time (assuming that we would need
to touch handlers at least... once every other week?). There's a time threshold
at which, if I'm kept waiting for enough seconds, I will mentally context switch
away from what I'm working on, and then have to context switch back once the
cycle finishes.

Reminds me of the well-known 100ms search result threshold (quick search turns
up [this ex-Amazon source][glinden] and [this UX StackExchange answer][ux-se]).

[glinden]: https://glinden.blogspot.com/2006/11/marissa-mayer-at-web-20.html
[ux-se]: https://ux.stackexchange.com/questions/101908/live-search-response-time

Our usage of [Storybook](https://storybook.js.org/) for being able to iterate
on React components outside of the web app, by contrast, was great.

### Use technology the way it was designed to be used.

If you leave the happy path, you should be damn sure that this is something
worth spending your time on, because it will be hard.

For us, using Bazel with Typescript was a mistake. Using Bazel, Typescript, and
protos? [An even bigger mistake.][bazel-ts-protos]

[bazel-ts-protos]: /2023-10-07-protocol-buffers-grpc-and-js-ts-a-rant

To give an example of the problems this caused, it took 3 senior+ engineers a
full week to figure out how to repackage our webapp for deployment in AWS Lambda
because we had to fight our way through Bazel packaging shenanigans which `tsc`
was never designed to support.

More generally, we spent - by my estimate - about one eng-month every quarter
fighting Bazel to make it do what we wanted it to do, because we kept putting
everything in Bazel.

Bazel for our C++ setup was fine - CMake files are hell on earth, and C++
compilation and linking are very core to Bazel's design and implementation. We
should've stopped there: nothing else that we did belonged in Bazel.

Even for things that Bazel in theory makes easier, e.g. "identify the tests that
this PR modified and we should run", we still had our own layer on top of that
of "if path foo changed run foo integration test, if path bar changed run bar
integration test".

### Corollary: the release cycle should be fast.

As in: once you merge the code, why is it not instantly deployed to prod? If you
want to debounce releases, why not every 5 minutes?

We only did backend releases twice a week. CLI releases maybe every other week?
Both cadences were far too slow.

There was a huge mental disconnect between "eng working on feature" and
"customer experiencing feature" in no small part due to the slow release
cadence.

### Corollary: the review cycle should be fast.

It should also be easy! If you change your webapp, for example, you should be
able to preview those changes on a dev version of your webapp that's spun up for
your PR. This is especially critical for your reviewer.

Incidentally, we also hand-rolled this at the start, whereas Vercel provides
this out of the box. (I'll get to this later.)

### Corollary: updating documentation must be easy.

AKA: your docs should come with a WYSIWYG editor. Reviews should be highly
optional.

Gitbooks is terrible. Docusaurus was OK - it makes sense for a 100+ eng org or
company, maybe, but not a pre-PMF startup.

I should be able to fix docs using a 10 year old smartphone.

Going through Git is unnecessary; sure, version control is nice and dandy, but
you do not need Git for that.

If you do put your docs in the monolith, then you should at least have CD for
your docs.

## Using AWS - probably a good decision.

We stuck to what felt like reasonably run-of-the-mill infra decisions, e.g. use
EKS, RDS, SQS, SNS, Secretsmanager.

Control Tower was the right call for provisioning new accoutns.

We should've only had one `production` AWS account, one `staging`, one `dev`.
Instead we had two each and it was a constant source of friction and confusion.

Scared about people being able to change the DNS record for
yourstartupwebsite.com? That's part of the tradeoff you make if you want people
to be able to push to prod. You could handle this problem, maybe, by putting
sensitive resources like this in another AWS account, but IMO, not worth
cross-AWS-account complexity. Maybe you can handle this problem by defining a
new IAM role that doesn't have Route53 permissions and having this be the
default role given to all engineers?

If something does go wrong? Handle the "who did it" problem using a blameless
postmortem culture and having audit trails of all AWS ops (again, Control Tower!).

### Using CDK - honestly not sure.

It felt like a good decision, esp. when we needed to wire up stuff in ways where
if we had written actual CFN, it would have been a nightmare to keep it all in
sync, but it definitely caused a lot of friction at times when you had tricky
cross-stack dependency cycles.

Maybe it was because we had too many stacks - when I was leaving, we were
starting to shift towards "one construct per microservice" instead of "one CFN
stack per microservice".

I've seen advice since then that's suggested using `cdk deploy --no-rollbacks`
to help ameliorate the effects of a dependency cycle. Not sure how I feel about
that, but I can definitely see how that would help.

Another possibility would've been to use a combination of CDK, CFN, and
home-rolled shell scripts that strung togehter idempotent `aws` CLI commands.
This shape of solution - building a composite solution out of different
individual solutions on the spectrum of tradeoffs - is generally the right one.

## Go-to-market followup is vital

We never did the marketing/sales work you need for a B2B product, and - this
is what it felt like to me - kinda just crossed our fingers that product-led
growth would happen. (We had enough VSCode extension growth that I guess we
were able to lie to ourselves about this - 1K to 100K installs - but I suspect
most of those users just uninstalled the extension after trying it. It
certainly never translated into paid SaaS usage; and I also don't know what the
7DAU or 30DAU numbers for the extension were.)

In hindsight, I should've spoken up more about this at the start - but I was
still trying to adjust to startup rhythm from Google.

### Meet your users where they are

Every new flow you require a user to learn is an opportunity for you to lose
that user. The easier you make it for your users to use your product, the more
breadcrumbs they have to your product from their existing flows, the happier
they will be.

tldr: integrations are king.

## Premature cost optimization is bad.

> Premature optimization is the root of all evil.
>
> \- Don Knuth

One example: we didn't give out [LaunchDarkly](https://launchdarkly.com/)
accounts to everyone, so if someone was working on something gated by a feature
flag and wanted to turn it on in staging or prod, they had to ask a TL to turn
it on for them. We had one guy build an internal-only UI feature so that people
could actually toggle their own feature flags!

Maybe you don't give away admin permissions for your GitHub org. But other than
that? Give people the farm.

## Build custom tooling at every layer of abstraction.

You need your own tooling wrappers to make things easy. You also need tools at
every layer of abstraction - you should have shell scripts _and_ actual
type-checked TS/Python scripts.

One or two complex `jq` filters? A shell script is fine. Chain enough of them
together, though, and you should _probably_ start moving the monstrosity into
TS, just so that you can do `object.filter(...)[0].attr1.attr2`.

The two internal tools I wrote that I came back to regularly were `infractl rds
connect` to connect directly to a staging/prod DB, and `infractl gh GET
/repos/customer/hello-world` to make GH API calls using our prod credentials.

Having release shell scripts that we could run, and re-run, was important.

Having tools that configured everyone's AWS profiles and their k8s credentials
was also super important.

I found myself reaching under the hood of some of our release tools super
frequently, though, because I needed to iterate faster. Hence why I say you need
tools at every layer of abstraction.

## Strategy, culture, and values

With four founders, decision-making just took too long. The rest of us were
never exposed to the inputs that went into those decisions. I didn't learn until
2 years in that we had a revenue goal - and even then I only learned it through
backchannelling!

Transparency also takes work. Sure, as leads there's an instinctive desire to
control messaging and all that - but we didn't do nearly enough work,
to proactively share information.

It felt a bit like this triggered something of a negative feedback loop, where
because info A wasn't shared, decision B was harder to explain, so the reasoning
for decision B wasn't discussed, and so forth.

### Slow down occasionally

Retrospectives are important.

Performance reviews are important.

When I was getting ready to leave and was looking back on what I'd done, I
realized that there were entire months that were just spent on... nothing. Well,
not nothing - upgrading a dependency here, fixing a workflow there, etc - but a
lot of it on stuff that didn't feel like it moved the needle on our success as a
company.

If I'd asked myself that question every 6 months - if we'd had performance
reviews every 6 months that asked that question - I can't help but wonder if
I would've been able to address that way earlier.
