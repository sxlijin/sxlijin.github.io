# Notes on Designing Data-Intensive Applications

I just finished reading Kleppman's Designing Data-Intensive Applications, which
at this point seems to be something of a seminal text, and wanted to jot down
some thoughts I had about the book.

Some context on my background: I spent 3.5 years at Google, most of it working
on [Cloud Firestore](https://cloud.google.com/firestore) and in the guts of
[Bigtable], [Megastore], and [Spanner], so I know some things about
distributed data systems.

[Bigtable]: https://static.googleusercontent.com/media/research.google.com/en//archive/bigtable-osdi06.pdf
[Megastore]: https://storage.googleapis.com/gweb-research2023-media/pubtools/pdf/36971.pdf
[Spanner]: https://www.usenix.org/system/files/conference/osdi12/osdi12-final-16.pdf

Overall, I found the book to be really well done:

* It introduces the reader to a wide variety of design problems and the
  patterns used to solve them.
* It manages a good balance between theory and practice, by consistently
  providing real-life examples of the design patterns it discusses.
* It spends, appropriately, a lot of time discussing transactions and
  consistency semantics[^consistency-semantics].

[^consistency-semantics]: I do wish the book also warned users that even if a
    system claims to have such-and-such semantics, that such claims can also be
    misleading if not blatantly wrong. For example, [recent
    research][jepsen-mysql] has found that "MySQL Repeatable Read transactions
    cannot safely read a value and then write it".

[jepsen-mysql]: https://jepsen.io/analyses/mysql-8.0.34

## Criticisms

* I would've liked to see discussion of migration patterns, e.g.:
  * How do you do a live migration?
  * How do you do a migration with downtime?
  * How do you decide what acceptable downtime for a migration is?
  * Do you migrate all clients individually?
  * Do you migrate clients transparently using a middleware layer?

* I would've liked to see discussion of techniques for detecting and managing
  data consistency issues - specifically, patterns around cron jobs and
  automatic vs on-demand data repair.[^cron-jobs]

[^cron-jobs]: There's a brief discussion in Ch. 11, "Stream Processing",
    about "Keeping Systems in Sync". This felt way too short to me - this is an
    extremely common data design problem, and there's a lot of complexity here.
    For example, what do you do if your change-data-capture processor has a bug
    (which it likely will!) - and how would you even notice it?
  
* I do _not_ like its presentation of exactly-once semantics. You can only
  guarantee either at-most-once semantics or at-least-once semantics.

  Yes, you can combine at-least-once semantics with idempotent operations to
  give off the appearance of exactly-once semantics, but you cannot actually
  provide exactly-once semantics (i.e. you can't _guarantee_ that an operation
  will succeed).

* I wish it talked about tradeoffs at a very abstract (and somewhat
  reductionist) level, e.g. that OLTP vs OLAP is the difference between systems
  that supports mixed read-write workloads and systems that prioritize
  read-heavy workloads.

* I would've liked to see some case studies - but the book is already
  incredibly long, and _very_ dense as it is.

## There's always more

Some things that came to mind which the book did not discuss (appropriately
so, I think, since Kleppman had to draw the line somewhere for content to put
in the book):

### General distributed system patterns

* Caches are tricky, because cache invalidation is a hard problem. It sounds
  easy to do cache invalidation, but when everyone has a story about an outage
  they dealt with because a cache somewhere wasn't invalidated, well...

  Or, as I like to think of it: caches are an easy way to introduce eventual
  consistency into your system.

* "control plane and data plane" is super common - your data plane, which
  handles most user-facing work, needs to scale, but your control plane, which
  handles your partitioning/replication, usually doesn't (and often doesn't
  even have the same availability requirements!).

* When you start building distributed systems, you also rapidly run into new
  classes of problems: long tail latencies, queries of death, feedback loops,
  and so on. [You also have to design for these.][sre-book]

[sre-book]: https://sre.google/sre-book/addressing-cascading-failures/

### Operational Considerations

* How do you monitor a database? What telemetry is important? (e.g. lock
  contention as a proxy for user-facing errors, logging of per-query memory
  usage to identify OOM-causing queries)

* How do you monitor data consistency or replication lag?

* How do you handle a data replication bug in production? What custom telemetry
  do you need to debug it? Do you need tooling to, say, quarantine a replica or
  directly modify a replica's on-disk data?

* How do you release changes to the database? Schema updates, storage engine
  upgrades, replication topology changes, and so forth.

* Do you have backups? Do you know how you would restore from a backup? Do you
  know your RPO and RTO?

* What testing strategies should you employ to validate a major change? e.g.
  can you replay production traffic in a test environment? (Do you need to
  sanitize it, to respect users' privacy?)
