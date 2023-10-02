---
layout: bare-bones
---
{% include head.html css_override="/css/resume.css" %}

# Sam Lijin

recruiting@sxlijin.com \| https://github.com/sxlijin \| https://linkedin.com/in/sxlijin \| he/him

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