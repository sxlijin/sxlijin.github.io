# Next.js and Coordinated Disclosure

Everything old is new again.

# What am I talking about?

Next.js, one of the most dominant frameworks for greenfield full-stack applications, recently merged a fix for [CVE-2025-29927 Authorization Bypass in Next.js Middleware](https://github.com/advisories/GHSA-f82v-jwr5-mffw). Everything about how this vulnerability was communicated, though, was incredibly problematic.

This blog post is going to be about what went wrong, what should have happened, and is a plea and call to action for everyone doing security things who hasn't done them before.

(If you're trying to figure out what you need to do, tl;dr [read the Next.js CVE post](https://nextjs.org/blog/cve-2025-29927). If you're looking for details about the vuln itself, please refer to [the writeup from zhero and inzo](https://zhero-web-sec.github.io/research-and-things/nextjs-and-the-corrupt-middleware) who discovered and reported this.)

# Background

For the purposes of this blog post, here's the technology context you need to know:

- Next.js had a bug that allowed bypassing authorization logic.
- Next.js is an **open-source** framework for full-stack applications. It has become the go-to choice of framework for everyone from indie hackers and early-stage startups to mature companies. React itself even now lists it as the **first recommended choice** of full-stack framework: https://react.dev/learn/creating-a-react-app.
- Vercel is the company that maintains Next.js, and also offers their own Next.js hosting solution. This naturally makes Vercel the default solution for hosting a Next.js app.
- However, for Next.js to be a truly open framework, and not just a proprietary tech stack locking you into Vercel, then people should be able to host their Next.js app anywhere! That's where the [OpenNext](https://opennext.js.org/) project comes in, which aims to make it easy for users to host their Next.js app elsewhere, whether it be AWS (using SST), Cloudflare, or Netlify.

Also, in the interest of personal disclosure: I work at a seed-stage startup that uses Next.js and Vercel. We're happy customers and will continue using them. But I have absolutely lost trust in Vercel because of this.

# Timeline
From [zhero and inzo's writeup](https://zhero-web-sec.github.io/research-and-things/nextjs-and-the-corrupt-middleware):

- **02/27/2025**: vulnerability reported to the maintainers (_specifying that only versions between 12.0.0 and 12.0.7 were vulnerable, which was our understanding at the time_)
- **03/01/2025**: second email sent explaining that **all versions were ultimately vulnerable**, including the latest stable releases
- **03/05/2025**: initial response received from the Vercel team explaining that versions 12.x were no longer supported/maintained (_probably hadn’t read the second email/security advisory template indicating that all were vulnerable_)
- **03/05/2025**: another email sent so that the team could quickly take a look at the second email/security advisory template
- **03/11/2025**: another email sent to find out whether or not the new information had been taken into account
- **03/17/2025**: email received from the Vercel team confirming that the information had been taken into account
- **03/18/2025**: email received from the Vercel team: the report had been accepted, and the patch was implemented. Version 15.2.3 was released a few hours later, containing the fix (_+backports_)
- **03/21/2025**: publication of the security advisory

And here's where Vercel fucked up, hard:

- Vercel started spinning this as a marketing opportunity for their hosting platform.
- No OpenNext maintainer was ever notified before CVE publication that they needed to possibly ship fixes.
- Vercel has not - as far as I know - done any explicit user notification of whether they are impacted or not (which they absolutely have the ability to do).

# Security Incident Response

Before I explain what I expected of Vercel here, I should first explain what incident response looks like, from the perspective of someone who runs an application serving users:

- I open Hacker News, Slack, or Twitter, and hear about CVE-2025-xxxx affecting FizzBuzz Technology.
- The first thing I do is **triage**: are we vulnerable? Do we need to learn more about CVE-2025-xxxx?
	- If I do not use FizzBuzz Technology, cool, I can ignore it.
	- If I do use FizzBuzz Technology, well, now I have to go and learn more about CVE-2025-xxxx to figure out if I'm vulnerable.
	- Before I do any research, I may also just immediately **page oncall**: maybe I'm a dad taking care of a crying newborn, or I'm just checking Hacker News while I'm on the ski lift (yes I do this sometimes), or maybe I work at a Big Company and I am one of 10 teams using FizzBuzz Technology, and CVE-2025-xxxx sounds really bad.
	- If it turns out we're not vulnerable, oncallers get to go back to their life.
- If we are vulnerable, we need to do three things, in no specific order:
	- Determine if the vulnerability is being actively exploited.
	- Cut off the attack vector, which usually means shipping a hotfix/patch.
	- Page everyone that needs to be involved in active response.
- Then we need to do **forensic analysis**: was the vulnerability ever exploited, and if so, how and what data was impacted? What do I need to tell my users to do?
	- This is why it makes headlines when [Microsoft loses security logs for two weeks](https://www.bleepingcomputer.com/news/security/microsoft-warns-it-lost-some-customers-security-logs-for-a-month/) - if you, as the user, do not have forensic logs, you cannot tell your users that their data is safe. You can't tell your users "no one stole your SSN" or "no one accessed your medical records" because you do not have the proof you need to make that assertion.
- If your forensic analysis shows evidence of exploitation, then you need to **notify your users** and specifically tell them what they need to do.
	- Do they need to rotate their passwords? Do they need to rotate their API keys?
	- Or, [in the case of Experian leaking credit reports](https://krebsonsecurity.com/2023/01/experian-glitch-exposing-credit-files-lasted-47-days/), just tell all your users "hey, someone might steal your identity for the rest of your life, kthxbye".

(For a full-fledged version of this, please refer to PagerDuty's [security incident response playbook](https://response.pagerduty.com/during/security_incident_response/).)

For a lot of Vercel's users, most of these steps don't matter! If you're building something where everyone can read everyone's data, or a social network with 10 users who just make shitposts, then you get to skip these steps.

But for a startup processing medical records for a hospital, or streamlining compliance research for legal firms, or providing banking services, **none of this is optional**.

# Security Incident Response as an Infrastructure Provider
If you're an IaaS or PaaS provider, your job is to streamline the process for your users, so that they can run the above for their end users. In other words, Vercel, and OpenNext maintainers - i.e. SST, Cloudflare, and Netlify - all need to provide their users with **guidance** about what to do.

In 2025, it is table stakes to:

- notify your user whether or not they need to take action in response to CVE-2025-xxxx
	- if they need to take action, what action they need to take
	- if they were vulnerable, how they can determine if they were ever exploited
	- if you can't do the above, provide guidance for how the user can figure this out themselves
- try to fix the problem in the platform layer
	- ideally, make it so that the user does not need to hotfix/patch any of their applications
	- alternatively, provide a setting that users can flip which will automatically protect them

Both of these have inherent tradeoffs:
- you want to avoid causing user panic
- you don't want to notify users unnecessarily
- if you fix it in your platform, do you run the risk of breaking any users?

Most importantly, all of this takes time to prepare.

# Coordinated Disclosure
This is where coordinated disclosure comes in: if a vulnerability affects an ecosystem, then you want to try to involve as many infrastructure providers in the disclosure process as possible, so that they can each run their own security incident response playbook.

This is not a new problem.

We saw this in 2018, when the [Meltdown and Spectre speculative execution vulnerabilities](https://meltdownattack.com/) were announced on the heels of **6 months of coordinated disclosure work** (see [security.stackexchange](https://security.stackexchange.com/questions/177147/why-and-where-was-meltdown-made-public-before-schedule-for-the-first-time),  [TechTarget](https://www.techtarget.com/searchsecurity/news/450432720/Huge-coordinated-vulnerability-disclosure-needed-for-Meltdown)) involving but not limited to Google, Amazon, Microsoft, Apple, Intel, ARM, AMD, and [many more](https://meltdownattack.com/#faq-advisory).

We saw this in 2014, as news about the [Heartbleed vulnerability in OpenSSL](https://heartbleed.com/) made its way through security backchannels [over the course of 2.5 weeks](https://www.smh.com.au/technology/heartbleed-disclosure-timeline-who-knew-what-and-when-20140414-zqurk.html): Google, Cloudflare, Akamai, OpenSSL maintainers themselves, various Linux distributions (Red Hat, SuSE, Debian, FreeBSD).

And these have just been the big ones: there have been countless more, all involving software providers who have had to work with each other to collectively serve their users.

# OK, so what did Vercel do wrong?

Well, as far as I can tell, there was no coordinated disclosure. At all.

- Mar 21 03:17AM PDT: [the CVE and GHSA were published](https://github.com/advisories/GHSA-f82v-jwr5-mffw).
- Mar 22 00:34AM PDT: @pilcrow, of [Lucia](https://lucia-auth.com/) and [Oslo](https://oslojs.dev/), [notices on Twitter](https://x.com/pilcrowonpaper/status/1903349591409303894).
- Mar 22 11:59AM PDT: @nextjs [shares their blog post](https://x.com/nextjs/status/1903522002431857063).

What follows is OpenNext providers (Cloudflare, Netlify) and the rest of the ecosystem scrambling to react.

## Cloudflare tries to roll out a hotfix

Mar 22 04:42AM PDT: [Matt Silverlock of Cloudflare](https://x.com/elithrar/status/1903411980070797691) providing remedial guidance and announcing that they're going to try to fix this for their users automatically:

> We ([@cloudflare](https://x.com/Cloudflare)) are going to deploy an automatic WAF rule that blocks requests w/ that can bypass Next.js auth middleware, including unpatched versions.
> 
> You can also create a WAF rule right now on *any* plan by heading to `<website> > Security > WAF > Create rule in the dash`:
> 
> <img width="100%" src="https://pbs.twimg.com/media/GmpG1X3X0AA9Ckf?format=jpg&name=4096x4096" />

Mar 22 08:47AM PDT: [rollout is finished](https://x.com/elithrar/status/1903473589820489828): 

> Rule is rolled out: https://developers.cloudflare.com/changelog/2025-03-22-next-js-vulnerability-waf/ No false positive reports yet (but if you are affected, please let us know). We are seeing a significantly higher % of blocks from VPS provider ASNs, which is typically a sign of malicious traffic/scanning.

Mar 22 11:09AM PDT: Matt announces that they [have to undo their rollout](https://x.com/elithrar/status/1903509257380938144) and create an opt-in setting because the fix could break user apps (c.f. [@\_DavidCodes](https://x.com/_DavidCodes/status/1903718973985771744), [@polar_sh](https://x.com/polar_sh/status/1903497785439047961), [@SamyPesse](https://x.com/SamyPesse/status/1903492686042009896)):

> We're (unfortunately) moving this rule to opt-in only as we saw folks using middleware from hosted auth providers fail. I have a PR up here that will go live very shortly with instructions on how to disable/enable: https://github.com/cloudflare/cloudflare-docs/pull/21062

## Netlify is pissed off, rightly so

Mar 22 09:14AM PDT: [Sean Roberts of Netlify](https://x.com/JavaSquip/status/1903480443158298994) :

> yo, [@vercel](https://x.com/vercel) [@rauchg](https://x.com/rauchg) This is awful - not the vulnerability (those happen) but your handling of it with the next.js community.
> 
> Glad major platforms have mitigations in place and rolled out. While Netlify itself was not vulnerable to this, ALL platforms have had to do a fire drill late/early today to validate this, push changes, and ensure this. Any platforms that were impacted now have to spend lots of time with customers ensuring their sites were not abused through this vulnerability - including Vercel's customers. Even Cloudflare's team [@elithrar](https://x.com/elithrar) [@dok2001](https://x.com/dok2001) had to work on putting rules into place today instead of when Vercel/next knew about it?!
> 
> From my perspective here, the timeline looks like the Vercel team knew about it for _at least_ 5 days and quietly pushed changes in that time to the latest versions of Next. This morning we see this advisory and it's the first time anyone from next/vercel reached out to anyone on the matter.
> 
> This is a major security issue with an open source framework that you maintain and where was Vercel here?
> 
>- The PRs for fixing the bugs look like maintenance not security issues
>- YOU HAVE DONE ZERO OUTREACH via [@vercel](https://x.com/vercel) [@nextjs](https://x.com/nextjs) even now to customers to advocate for updating and validating their systems.
>- You privately hit up other platforms early this morning several hours AFTER the CVE was announced
>- you did put in a changelog today that talks about your firewall but this is covering your bases, not supporting the community.
>
>Even on your platform, were customers ever impacted or was there a timeline that they were? As far as I can tell - as someone with Next.js sites deployed to Vercel - no outreach has been done to confirm any of this. Maybe at the end of the month it will show up in the product emails. You've treated this thus far the same as a spelling error that you needed to fix.
>
>So this means, Vercel knew about this for 5 days and did not work with the open community about it until they had to. This hurts trust with the open source contributors to next and customers using next at all. It also increased the amount of time customers were vulnerable and the amount of auditing and research to protect customers.
>
>We need something here to believe that you're properly informing and advocating for security practices and safety for those who use this framework regardless of if they user Vercel.

Mar 22 09:26AM PDT: [Eduardo Bouças of Netlify](https://x.com/eduardoboucas/status/1903483488847720615) :

> The timeline of CVE-2025-29927, a critical security vulnerability in [@nextjs](https://x.com/nextjs):
> 
>    - March 17: Fixes were committed to Next.js by Vercel staff
>    - March 21: CVE is published (…)
>    - March 22: [@vercel](https://x.com/vercel) published entry in changelog announcing that "Vercel Firewall proactively protects against vulnerability with Middleware"
>    - March 22: Vercel reaches out to Netlify in OpenNext Discord offering to help with a patch, linking to the changelog announcement
> 
> In the changelog announcement, Vercel managed to spin this as a strength: their firewall has protected your sites. But they're not very transparent about how many of their customers were ever vulnerable and for how long.
> 
> And they also don't explain why it took them 5 (five!) days to disclose the vulnerability to other providers so they could protect their own customers as quickly as possible. In the announcement, they say that the vulnerability «was responsibly disclosed». Not by them.
>
> Netlify customers were never affected by this vulnerability, but I can say with certainty that if we were on the other side of this, we would've been better citizens of the web. If you deploy Next.js to anywhere other than Vercel, Netlify or Cloudflare, please patch now! (…)
>
> Reference links:
> - https://vercel.com/changelog/vercel-firewall-proactively-protects-against-vulnerability-with-middleware
> - https://github.com/advisories/GHSA-f82v-jwr5-mffw
> - https://github.com/vercel/next.js/commit/52a078da3884efe6501613c7834a3d02a91676d2
> - https://github.com/vercel/next.js/commit/5fd3ae8f8542677c6294f32d18022731eab6fe48

## Clerk

Clerk is an authn provider that offers React components for `<SignIn/>`, `<SignUp/>`, `<UserButton/>`, and `<UserProfile/>`, as well as backends to power SSO, 2FA via SMS/email, and more. They offer [Next.js integration](https://www.npmjs.com/package/@clerk/nextjs) as a first-class feature and were similarly caught unaware.

Mar 22 10:20AM PDT: Clerk's [first official response](https://x.com/ClerkDev/status/1903497002828120426):

>  We are in touch with the Next.js team and can report that developers using Clerk are not impacted by the vulnerability disclosed today. Clerk uses a cryptographic signature to ensure middleware has run. If it’s bypassed by an attacker, Clerk will terminate the request with an error. As always, we still recommend upgrading to the latest version as soon as possible.

Mar 23 02:58AM PDT: [Konsti Wohlwend pointed out](https://x.com/n2d4wastaken/status/1903748178874360024) that this was blatantly wrong and Clerk is indeed affected:

> This is a blatant lie [@ClerkDev](https://x.com/ClerkDev) — you absolutely ARE vulnerable to the Next.js security vulnerability, and it's important that you inform your users about this.
> 
> It's quite trivial to reproduce. Just create a new Clerk project (on Next.js 15.2.2) and follow the exact instructions on the Getting Started page. That means adding a new protected route in the middleware, and supposedly the page should be protected, right?
> 
> Except it's not! The same malicious header that can be used to bypass Next.js middleware also works on Clerk. Yes, an error will show — the <ClerkProvider /> does that — but it doesn't render until the entire page has already been sent to the browser, and the Network Tab exposes the secret contents (see screenshot below).
> 
> Not only that, route.ts handlers are completely exposed to everyone with the new middleware bypass. Since API route handlers don't render the <ClerkProvider /> in layout.tsx, there won't even be an error.
> 
> I really don't know what to say here. It's not Clerk's fault that this whole thing happened in the first place, but pretending like everything is fine is absolutely not the way to go. It's important — the issue is out there now, and being exploited. This is not the time for partial information!

Mar 23 11:09AM PDT, Clerk [acknowledges the mistake](https://x.com/ClerkDev/status/1903871638245986311):

> Important update: the tweet below was in error, and there are two scenarios where your application may be impacted. The details are available here: https://clerk.com/blog/cve-2025-29927. Potentially impacted customers have been notified by email. The tweet below was a significant error. We apologize, and will be reflecting on and improving our procedures for zero-day vulnerabilities to ensure it does not happen again. Going forward, we are pleased that the Next.js team has committed to giving Clerk advance notice on vulnerabilities. We will be seeking similar relationships with other framework authors.

Mar 23 12:47PM PDT: yours truly gets the aforementioned email from Clerk.

<img width="100%" src="/assets/clerk-cve-2025-29927-impact.png" />

## Vercel running the cover-up playbook

Mar 22 05:07PM PDT: for the icing on the cake, [Vercel has taken down the firewall marketing post, and is currently pretending that it never happened](https://x.com/eduardoboucas/status/1903599428356780118):

> Yes, [@vercel](https://x.com/vercel) completely changed the changelog I linked to in my thread leaving no traces to the firewall marketing spin. Yes, I knew it would happen so I took a screenshot. Here's the version of reality you'd be presented had they not been called out.
>
> <img width="100%" src="https://pbs.twimg.com/media/GmrwU38WIAAIf_O?format=jpg&name=4096x4096" />


## Vercel's CEO

Mar 22 04:33PM PDT: meanwhile, [here's Vercel's CEO, @rauchg](https://x.com/rauchg/status/1903590962498326771):

> Cloudflare is responsible for one of the worst security disasters in internet history[1]
> 
> We tried to use your product and had non-stop incidents and had to move off.
> 
> We mitigate DDoS attacks you proxy to us daily (you’re slow).
> 
> Despite your constant cheap shots, we’re engaging with your team for better collaboration in the future. We want a secure web.
> 
> \[1] https://en.wikipedia.org/wiki/Cloudbleed

Except, gee, you know what happened in Cloudbleed? **Coordinated disclosure.**

Security incidents happen. Security disasters happen. What I care about, as a user, is **how you respond** when it happens, and what it says about how serious you are about your product, your ecosystem, and your users.

# What do we need to learn from this?

We're in the middle of another wave of startups and technology paradigms. People are now able to [vibe code entire apps into existence](https://x.com/karpathy/status/1903671737780498883), even [non-technical folks](https://x.com/leojr94_/status/1900767509621674109). 

That means it's increasingly important that if you're a software infrastructure provider, and you screw up, you **own it**. You don't play games with marketing or get into petty squabbles with your competitors. You definitely don't run the coverup playbook.

Here's a timeline I would have been OK with (not thrilled, per se, but this would have been acceptable):

  - day 1: researchers report initial issue
  - days 1-2: researchers and Next.js maintainers clarify scope of impact
  - days 3-5: Vercel prepares patches, rolls out fixes across the Vercel platform
  - day 4: Vercel sets announcement date to day 7 and reaches out to OpenNext maintainers at other hosting providers, namely Cloudflare and Netlify
  - day 4 or 5: other Next.js auth providers, e.g. Clerk, get looped in
  - days 4-6: Vercel, Cloudflare, Netlify, etc., analyze logs and share intel on the threat vector
  - day 7: everyone puts out their own tweets, blog posts, and runs their customer comm playbooks
  - day 7: people gossip on Twitter, but no one freaks out and no one's app breaks unexpectedly

Learn from the lessons of our predecessors: **none of these problems are new**. The only thing that's different is that you haven't dealt with them before.

To Vercel's credit, both Vercel's [CEO @rauchg](https://x.com/rauchg) and their [chief evangelist @leerob](https://x.com/leerob/status/1903551061752693183) are at least acknowledging that they screwed up:

> Vercel stands for a better, more secure web. We missed the mark on how we communicated about this CVE, esp with industry partners. We’ll iterate, our coordination & disclosure processes will strengthen as a result. I truly appreciate the outpouring of feedback from the community

> We missed the mark here. There's a number of things to improve: how quickly we can triage disclosures, how we work with partners, and our overall CVE comms. Working on this ASAP 🙏We'll also be publishing LTS (long-term support) versioning guidance.
