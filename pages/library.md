---
title: library
...

# Library

An incomplete list of media I've consumed. Most are formative, some are included for completeness, and occasionally there's something that's still on my todo list.

## Technical

### Blogs

* Avery Penwarr / apenwarr - https://apenwarr.ca/log/
  * go/vfa and [What do executives do, anyways? (2019)](https://apenwarr.ca/log/20190926)
  * [Systems design explains the world (2020)](https://apenwarr.ca/log/20201227) and [Systems design 2: what we hope we know (2023)](https://apenwarr.ca/log/20230415)
* Patrick McKenzie / patio11 - https://www.kalzumeus.com/ and https://www.bitsaboutmoney.com/
  * [Falsehoods Programmers Believe About Names (2010)](https://www.kalzumeus.com/2010/06/17/falsehoods-programmers-believe-about-names/)
  * [Salary Negotiation (2012)](https://www.kalzumeus.com/2012/01/23/salary-negotiation/)
* Julia Evans / b0rk - https://jvns.ca/
* Dan Luu / danluu - https://danluu.com/
* rachelbythebay - https://rachelbythebay.com/
* Amos Wenger / fasterthanlime - https://fasterthanli.me/
* Bartosz Ciechanowski - https://ciechanow.ski/ - a master class in interactive graphics
* Hillel Wayne - https://www.hillelwayne.com/
* Google / C++ Tip of the Week - https://abseil.io/tips/
* Google / Testing on the Toilet - https://testing.googleblog.com/
* High Scalability - https://highscalability.com/
* Will Larson / lethain - https://lethain.com/ and https://staffeng.com/
* Murat Buffalo - https://muratbuffalo.blogspot.com/
* Cindy Sridharan - https://copyconstruct.medium.com/
* Memos, collected by Sriram Krishnan - https://sriramk.com/memos/
* Daniel Lemire - https://lemire.me/blog/
* Raymond Chen - https://devblogs.microsoft.com/oldnewthing/

### Articles

* [Why you need a "WTF Notebook" (2021)](https://www.simplermachines.com/why-you-need-a-wtf-notebook/)

  > There's a very specific reputation I want to have on a team: "Nat helps me solve my problems. Nat get things I care about done."

* [What Every Systems Programmer Should Know About Concurrency (2020)](https://assets.bitbashing.io/papers/concurrency-primer.pdf)

* [When a Microsecond is an Eternity (2017)](https://github.com/CppCon/CppCon2017/blob/master/Presentations/When%20a%20Microsecond%20Is%20an%20Eternity/When%20a%20Microsecond%20Is%20an%20Eternity%20-%20Carl%20Cook%20-%20CppCon%202017.pdf)

* [What Color is Your Function? (2015)](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/)

* [Async Rust Is A Bad Language (2023)](https://bitbashing.io/async-rust.html)
  
  > Used pervasively, Arc gives you the world’s worst garbage collector.

* [People lie on surveys and focus groups, often unwittingly (2004)](https://devblogs.microsoft.com/oldnewthing/20041012-00/?p=37593)

  > The majority of consumers who buy computers claim that personal finance management is one of the top three reasons they are purchasing a PC. They’ve been claiming this for more than a decade. But only somewhere around 2% of consumers end up using a personal finance manager.

* [How Figma’s databases team lived to tell the scale (2024)](https://www.figma.com/blog/how-figmas-databases-team-lived-to-tell-the-scale/) and [The growing pains of database architecture (2023)](https://www.figma.com/blog/how-figma-scaled-to-multiple-databases/)

  > Limit potential availability impact to <1 minute

* [Every infrastructure decision I endorse or regret
    (2024)](https://cep.dev/posts/every-infrastructure-decision-i-endorse-or-regret-after-4-years-running-infrastructure-at-a-startup/)

* [ABP Privacy Infra, Long Range Investments (Facebook internal, 2021)](https://www.documentcloud.org/documents/21716382-facebook-data-lineage-internal-document)

* [Latency Numbers Every Programmer Should Know](https://colin-scott.github.io/personal_website/research/interactive_latency.html)

  AFAIK this originates from [a talk that Jeff Dean gave in
  2010][latency-numbers-jeff-dean]; the numbers have changed slightly in the
  decade+ since, but it largely still holds. You can also find it on [High
  Scalability][latency-numbers-high-scalability] and
  [sre.google][latency-numbers-sre-google].

[latency-numbers-jeff-dean]: https://youtu.be/modXC5IWTJI?t=3554
[latency-numbers-high-scalability]: http://highscalability.com/blog/2011/1/26/google-pro-tip-use-back-of-the-envelope-calculations-to-choo.html
[latency-numbers-sre-google]: https://static.googleusercontent.com/media/sre.google/en//static/pdf/rule-of-thumb-latency-numbers-letter.pdf

* [No Vehicles in the Park](https://novehiclesinthepark.com/)

  > some people think that there could be simple rules for Internet content that are easy to apply

* [StackOverflow Update: 560M Pageviews A Month, 25 Servers, And It's All About Performance (2014)](http://highscalability.com/blog/2014/7/21/stackoverflow-update-560m-pageviews-a-month-25-servers-and-i.html)

* [How we migrated our PostgreSQL database with 11 seconds downtime (2024)](https://gds.blog.gov.uk/2024/01/17/how-we-migrated-our-postgresql-database-with-11-seconds-downtime/) - Gov.UK's story of migrating from one RDS Postgres DB to another.

* [MySQL 8.0.34 (2023)](https://jepsen.io/analyses/mysql-8.0.34.pdf)

  > The core problem is that MySQL claims to implement Repeatable Read but
  > actually provides something much weaker.

* [PostgreSQL's fsync() surprise (2018)](https://lwn.net/Articles/752063/)

  > If that happens to a PostgreSQL server, the result can be silent corruption
  > of the database

* [A Few Billion Lines of Code Later (2010)](https://web.stanford.edu/~engler/BLOC-coverity.pdf)

  > Parsing is considered a solved problem. Unfortunately, this view is naïve,
  > rooted in the widely believed myth that programming languages exist.


* [No Silver Bullet (1986)](http://worrydream.com/refs/Brooks-NoSilverBullet.pdf) - a discussion of the fundamental challenges involved in creating software.

* [Supercomputing's Monster in the Closet (2016)](https://spectrum.ieee.org/computing/hardware/how-to-kill-a-supercomputer-dirty-power-cosmic-rays-and-bad-solder) - found via [this Twitter thread](https://twitter.com/whitequark/status/980522328151834624?s=19)

  > Jaguar had 360 terabytes of main memory [and] was logging ECC errors at a
  > rate of 350 per minute.

* [The Hardest Program I've Ever Written (2015)](http://journal.stuffwithstuff.com/2015/09/08/the-hardest-program-ive-ever-written/) - the story of `dartfmt`

  > The hardest program I’ve ever written, once you strip out the whitespace,
  > is 3,835 lines long. That handful of code took me almost a year to write.
  > [...] I deleted 20,704 lines of code over that time.

* [We can't send mail more than 500 miles](https://web.mit.edu/jemorris/humor/500-miles)

* [Caches, Modes, and Unstable Systems (2021)](https://brooker.co.za/blog/2021/08/27/caches.html)

  > So our system has two stable loops. One's a happy loop where the cache is
  > full [...] The other is a sad loop, where the cache is empty, and stays
  > empty

* [We Have No Moat, and Neither Does OpenAI (Google, 2023)](https://www.semianalysis.com/p/google-we-have-no-moat-and-neither)

* [The Tyranny of Structurelessness (1972)](https://www.jofreeman.com/joreen/tyranny.htm)

  > As long as the structure of the group is informal, the rules of how
  > decisions are made are known only to a few and awareness of power is
  > limited to those who know the rules.

* [Things You Should Never Do, Part 1 (2000)](https://www.joelonsoftware.com/2000/04/06/things-you-should-never-do-part-i/) - Joel Spolsky talking about the Netscape rewrite

* [Debugging an evil Go runtime bug (2017)](https://marcan.st/2017/12/debugging-an-evil-go-runtime-bug/)

* [The Elves Leave Middle Earth - Sodas Are No Longer Free (2009)](https://steveblank.com/2009/12/21/the-elves-leave-middle-earth-%E2%80%93-soda%E2%80%99s-are-no-longer-free/)

* [An Oral History of Bank Python (2021)](https://calpaterson.com/bank-python.html)

* [Better Context Menus with Safe Triangles (2023)](https://www.smashingmagazine.com/2023/08/better-context-menus-safe-triangles/), found via [this Tweet](https://twitter.com/claviska/status/1710098277767729479?t=9M1Mv15eiCqWxx7q8ghztg)

### Talks

* [Jeff Dean - Building Software Systems at Google and Lessons Learned (2010)](https://youtu.be/modXC5IWTJI)

* [Robert Kennedy - Life in the Trenches of healthcare.gov (2014)](https://www.dotconferences.com/2014/05/robert-kennedy-life-in-the-trenches-of-healthcare-gov)

* [Mike Monteiro - Fuck You, Pay Me (2012)](https://youtu.be/jVkLVRt6c1U)

### Books

* Tanya Reilly - The Staff Engineer's Path
* Martin Kleppman - Designing Data-Intensive Applications

### Google

I originally put this together back when I was at Google, trying to collate the publicly available resources I knew about so that I had something to point to when people asked me about life at Google. I don't update this much anymore.

* [Call Me Ishmael (2012)](https://mike-bland.com/2012/08/10/call-me-ishmael.html) - Mike Bland reflects on the challenges involved in encouraging Google engineers to test their code (Mike's blog also talks about internal infra like TAP, Forge, etc., and even for Googlers the stuff about the history of the Testing Grouplet is a great read).
* [Why Google Stores Billions of Lines of Code in a Single Repository (2016)](https://dl.acm.org/doi/pdf/10.1145/2854146) - a primer on google3's scale, both technical and organizational, and the solutions that Google has built around it
* [Site Reliability Engineering (2016)](https://landing.google.com/sre/sre-book/toc/index.html) and its followup, [The Site Reliability Workbook (2018)](https://landing.google.com/sre/workbook/toc/), which discuss the art of maintaining production systems.
* [Modern Code Review: A Case Study at Google (2018)](https://sback.it/publications/icse2018seip.pdf) - qualitative analysis of code review practices at Google (there's also [CodeFlow: Improving the Code Review Process at Microsoft (2018)](https://queue.acm.org/detail.cfm?id=3292420), which is another mostly qualitative discussion, although they do have some very interesting stats).
* [The Datacenter as a Computer (2018)](https://www.morganclaypool.com/doi/pdf/10.2200/S00874ED3V01Y201809CAC046) - I'm not sure if I got anything *specifically* useful from this, but in general I found it to be a nice overview of the challenges involved in operating a modern datacenter.
* [Software Engineering at Google (2020)](https://abseil.io/resources/swe-book) - still haven't read this
* [25 Years of Warehouse-Scale Computing (2024)](https://www.computer.org/csdl/magazine/mi/5555/01/10551740/1XyKBf0Y6uA)
* Code Search, but for OSS: https://cs.opensource.google/

## Personal

### Fiction

* N. K. Jemisin's "The Broken Earth" trilogy
* pirateaba's [The Wandering Inn](https://wanderinginn.com/)

### Niches

Industries (non-software)

* https://abovethelaw.com/
* https://semiengineering.com/
* https://www.eetimes.com/
* https://ncatlab.org

Outdoors

* gear: REI, Moosejaw, Backcountry, Expertvoice
* navigation: Gaia, CalTopo, OnX, Fatmap
* https://www.vdiffclimbing.com/
* https://www.redbull.com/int-en/shows/reel-rock-1

Other

* Hardware: McMaster-Carr, Grainger, OnlineMetals
* Electronics: Digikey, Mouser
* Stationery: JetPens, MUJI, others that I haven't used
* https://thecentercolumn.com/
* WiFi: https://www.wiisfi.com/

### Assorted

* [Veritasium - Why Machines That Bend Are Better (2019)](https://youtu.be/97t7Xj_iBv0)
* [Paul Campos - How I Won My Lawsuit Against the University of Colorado (2024)][campos-pt-1], [pt. 2][campos-pt-2], and [pt. 3][campos-pt-3]

[campos-pt-1]: https://www.lawyersgunsmoneyblog.com/2024/02/how-i-won-my-lawsuit-against-the-university-of-colorado
[campos-pt-2]: https://www.lawyersgunsmoneyblog.com/2024/02/how-i-won-my-lawsuit-against-the-university-of-colorado-part-ii
[campos-pt-3]: https://www.lawyersgunsmoneyblog.com/2024/02/how-i-won-my-lawsuit-against-the-university-of-colorado-part-iii
