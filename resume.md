---
layout: bare-bones
---
{% include head.html css_override="/css/resume.css" %}

# Sam Lijin

he/him \| 
<svg class="icon" viewBox="0 0 24 24"><path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4-8 5-8-5V6l8 5 8-5v2z"></path></svg>
[jobs@sxlijin.com](mailto:jobs@sxlijin.com) \| 
<svg class="icon" viewBox="0 0 24 24"><path d="M12 1.27a11 11 0 00-3.48 21.46c.55.09.73-.28.73-.55v-1.84c-3.03.64-3.67-1.46-3.67-1.46-.55-1.29-1.28-1.65-1.28-1.65-.92-.65.1-.65.1-.65 1.1 0 1.73 1.1 1.73 1.1.92 1.65 2.57 1.2 3.21.92a2 2 0 01.64-1.47c-2.47-.27-5.04-1.19-5.04-5.5 0-1.1.46-2.1 1.2-2.84a3.76 3.76 0 010-2.93s.91-.28 3.11 1.1c1.8-.49 3.7-.49 5.5 0 2.1-1.38 3.02-1.1 3.02-1.1a3.76 3.76 0 010 2.93c.83.74 1.2 1.74 1.2 2.94 0 4.21-2.57 5.13-5.04 5.4.45.37.82.92.82 2.02v3.03c0 .27.1.64.73.55A11 11 0 0012 1.27"></path></svg>
[sxlijin](https://github.com/sxlijin) \| 
<svg class="icon" viewBox="0 0 24 24"><path d="M19 3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14m-.5 15.5v-5.3a3.26 3.26 0 0 0-3.26-3.26c-.85 0-1.84.52-2.32 1.3v-1.11h-2.79v8.37h2.79v-4.93c0-.77.62-1.4 1.39-1.4a1.4 1.4 0 0 1 1.4 1.4v4.93h2.79M6.88 8.56a1.68 1.68 0 0 0 1.68-1.68c0-.93-.75-1.69-1.68-1.69a1.69 1.69 0 0 0-1.69 1.69c0 .93.76 1.68 1.69 1.68m1.39 9.94v-8.37H5.5v8.37h2.77z"></path></svg>
[sxlijin](https://linkedin.com/in/sxlijin)

## Experience

### trunk.io \| July 2021 - current

#### Check \| Services TL, Mar 2023 - current

Proposed, scoped, designed, and led a team of 3 to:

* Build out Trunk Check's GitHub integrations, to scan PRs and repos for lint issues, formatting issues, and security vulnerabilities
* Build a web-only onboarding experience (prior to this, onboarding meant either installing the CLI or our VSCode extension)
* Build out integrations with security tools, to provide comprehensive security coverage of PRs and repos

#### Check \| IC, July 2021 - Feb 2023

* Helped close our first 100+ engineer customer, by working with them to identify and close feature gaps
* Designed and implemented system for tracking Check billing usage
* Proposed, designed, and implemented various key Check features:
  * a plugin system, which enables users to contribute linter integrations to Check,
  * linter integration framework features (non-file targets, output post-processing, linter integration debugging),
  * usability improvements (e.g. surfacing filename and line numbers of invalid config entries)
* Provide customer service to enterprise and community users

### Google \| Sept 2017 - June 2021

#### User Identity SRE \| IC, Mar 2021 - June 2021

* Identified and severed a Gmail dependency on internal Identity storage systems, preempting the need to staff a team to do so
* Analyzed global outage potential for three internal systems (OIDC, autoconfig, TidyDB) and designed solutions to limit the blast radius of production changes to said systems

#### Cloud Firestore \| EngProd TL, Aug 2019 - Feb 2021

* Ran OKR planning and set a vision for the EngProd team, while also transitioning through 4 managers over 6 months
* Ramped up every new hire for the team: two new grad ICs, three senior ICs, and one managers
* Participated in org-level reviews of design docs and postmortems for Cloud EngProd and Databases
* Set up automated release proctoring, allowing us to stop manually proctoring releases
* Modernized release automation for GAE nightly releases, allowing Firestore to retain a production-like environment for ad-hoc testing
* Designed and implemented tooling to run a hermetic instance of Firestore
* Proposed and implemented a standardized onboarding curriculum for new Firestore engineers ("breadth talks") 

#### GAE Memcache \| EngProd TL, Apr 2019 - July 2019

* Proposed and kicked off a plan to close critical test coverage gaps and improve iteration speed, based on a review of high-impact outages and the feature roadmap for the next year
* Implemented automated performance regression detection for the backend and GAE Flex serving path 
* Consulted on how to run end-to-end tests that exercised GAE Memcache integrations with other subsystems

#### Cloud Firestore \| IC, Sept 2017 - July 2019

* Participated in design of the migration from an eventually consistent storage layer to strongly consistent storage layer (Megamover for Firestore)
* Designed and implemented test infrastructure for the aforementioned migration
* Designed and implemented invariant testing of a new metadata garbage collection system (EG cleanup)
* Participated in incident response for high-severity outages (omg/12642, omg/12277, omg/12344)

#### Miscellaneous \| Sept 2017 - June 2021

* Redesigned the Noogler training curriculum on Google-internal storage/database technologies
* Proposed and landed features in various systems owned by other teams (e.g. grimoire docs-panel, readtome.gwsq, canaries_max)
* Landed multiple large-scale changes to refactor APIs across all of google3 (e.g. adding Python RPC stub types, deleting old C++ APIs, removing SWIG dependencies)

## Education

Vanderbilt University (Nashville, TN) \| May 2017 \| B.S. in Computer Science; B.A. in Mathematics and Political Science \|  3.73 / 4.00

Stuyvesant HS (New York, NY) \| 2013

## Skills

* C++, Java, Typescript, Javascript, Python; light experience with Rust, Golang, Scheme
* Bazel, gRPC, React
* Hobbies: climbing, skiing, photography