---
title: resume
header: <header></header>
css: /css/resume.css
...

# Sam Lijin

<p>
he/him
|
<a href="mailto:jobs@sxlijin">
<svg class="icon" viewBox="0 0 24 24"><path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4-8 5-8-5V6l8 5 8-5v2z"></path></svg>
jobs@sxlijin.com
</a>
|
<a href="https://sxlijin.github.io">
<svg class="icon" viewBox="0 0 24 24"><path d="M3.9 12c0-1.71 1.39-3.1 3.1-3.1h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1zM8 13h8v-2H8v2zm9-6h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1s-1.39 3.1-3.1 3.1h-4V17h4c2.76 0 5-2.24 5-5s-2.24-5-5-5z"></path></svg>
sxlijin.github.io
</a>
|
<a href="https://github.com/sxlijin">
<svg class="icon" viewBox="0 0 24 24"><path d="M12 1.27a11 11 0 00-3.48 21.46c.55.09.73-.28.73-.55v-1.84c-3.03.64-3.67-1.46-3.67-1.46-.55-1.29-1.28-1.65-1.28-1.65-.92-.65.1-.65.1-.65 1.1 0 1.73 1.1 1.73 1.1.92 1.65 2.57 1.2 3.21.92a2 2 0 01.64-1.47c-2.47-.27-5.04-1.19-5.04-5.5 0-1.1.46-2.1 1.2-2.84a3.76 3.76 0 010-2.93s.91-.28 3.11 1.1c1.8-.49 3.7-.49 5.5 0 2.1-1.38 3.02-1.1 3.02-1.1a3.76 3.76 0 010 2.93c.83.74 1.2 1.74 1.2 2.94 0 4.21-2.57 5.13-5.04 5.4.45.37.82.92.82 2.02v3.03c0 .27.1.64.73.55A11 11 0 0012 1.27"></path></svg>
sxlijin
</a>
|
<a href="https://linkedin.com/in/sxlijin">
<svg class="icon" viewBox="0 0 24 24"><path d="M19 3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14m-.5 15.5v-5.3a3.26 3.26 0 0 0-3.26-3.26c-.85 0-1.84.52-2.32 1.3v-1.11h-2.79v8.37h2.79v-4.93c0-.77.62-1.4 1.39-1.4a1.4 1.4 0 0 1 1.4 1.4v4.93h2.79M6.88 8.56a1.68 1.68 0 0 0 1.68-1.68c0-.93-.75-1.69-1.68-1.69a1.69 1.69 0 0 0-1.69 1.69c0 .93.76 1.68 1.69 1.68m1.39 9.94v-8.37H5.5v8.37h2.77z"></path></svg>
sxlijin
</a>
</p>

## Experience

### [Trunk](https://trunk.io) \| July 2021 - current

#### [Check](https://trunk.io/products/check) \| Senior Software Engineer \|  IC, July 2021 - Feb 2023 \| Services TL, Mar 2023 - current

* Proposed, designed, and led a team of 3 to build a [web experience for Check](https://docs.trunk.io/check/get-started) (both a web-only onboarding flow and SaaS offerings)
* Proposed and built vulnerability scanning into Check, enabling it to compete with security products such as Snyk
* Helped grow Check from <1K users to 90K+ users by focusing on product-led growth

### Google \| Sept 2017 - June 2021

#### User Identity SRE \| Senior Software Engineer \| IC, Mar 2021 - June 2021

* Designed an incremental key rotation system to limit the global outage risk to Google SSO
* Discovered and severed an undocumented Gmail serving dependency on Identity-internal systems

#### [Cloud Firestore](https://firebase.google.com/docs/firestore) \| Senior Software Engineer \| IC, Sept 2017 - July 2019 \| EngProd TL, Aug 2019 - Feb 2021

* Metadata TTL system: backlog of XX trillion records, sustained 1M ops/sec, peaking at 3M ops/sec
  * Designed and implemented a logging system with novel observability and privacy requirements
  * Designed and implemented Jepsen-style testing to validate correctness guarantees

* [Datastore Migration](https://cloud.google.com/datastore/docs/upgrade-to-firestore): zero downtime, xM RPS and xxPB of data over xM customers and 36 datacenters
  * Designed composite index migration, queue processing migration, progressive rollout, fast rollback, and disk stockout mitigations; implemented transaction log replay, state transitions, and dark launch process
  * Designed and implemented end-to-end correctness and performance testing

* Velocity improvements for 60-eng org
  * Proposed and implemented automated rollbacks: got us out of a 3-month release freeze and prevented 5 outages over the next 6 months
  * Proposed and implemented new development and release environments spanning 30+ microservices

* Incident response for API proxy rollback affecting every Google Cloud service

#### Google App Engine Memcache \| Software Engineer \| EngProd TL, Apr 2019 - July 2019

* Proposed and led execution of test coverage improvement strategy for a new control plane: reduced rollbacks and ensured strong consistency of a distributed cache serving xxM QPS
* Designed and implemented automated performance regression testing for two critical serving paths
  * Used to validate Google-wide rollout of AMD CPUs, by proving a 50p latency delta of <10µs
  * Implemented on shared Borg (i.e. vulnerable to noisy neighbors) with <12% variance

#### Miscellaneous \| Sept 2017 - June 2021

* Redesigned the Noogler training on Google-internal storage technologies & trained 2500+ Nooglers
* Landed multiple google3-wide refactorings, each spanning xxK files (e.g. SWIG to CLIF)

## Education

Vanderbilt University (Nashville, TN) \| May 2017 \| B.S. in Computer Science, Mathematics, and Political Science

Stuyvesant HS (New York, NY) \| 2013

## Skills

* C++, Java, Typescript, Javascript, Python, Bash; light experience with Rust, Golang, Scheme
* gRPC, Bazel, React, Linux
* Hobbies: climbing, skiing, photography
