---
title: Personal Pages, Powered by Pandoc
...

# Personal Pages, Powered by Pandoc

I just finished reimplementing this website using pandoc, and am feeling
reasonably good about the longevity of this implementation. Here's roughly how
it works:

  - `build.sh` is a home-rolled build system that downloads the `pandoc` and
    `dart-sass` CLIs on-demand

  - `pandoc` and `dart-sass`, in combination with a custom pandoc template and
    Lua filters, generate all the static assets

  - I use `nodemon` and `browser-sync` for hot reload

This feels an order of magnitude better than all the Jekyll shenanigans I was
wrangling in the previous setup.

## Requirements

Here's what I identified as requirements when re-implementing all this:

  - running it locally should be incredibly easy - Jekyll was abysmal at this
  - Boring Technology TM
    - If the next time I update this website is in 2028, I want to be able to
      run the website locally within a minute of cloning the repo.
  - mostly preserve the existing URLs: `sxlijin.github.io/`, `/blog`, `/resume`,
    and `/bookshelf` (I've since renamed `/bookshelf` to `/library`, just
    because I liked the latter more)
  - SCSS support - I'm never hand-writing straight CSS again if I can help it
  - some basic templating functionality (e.g. being able to include a header)
    - I could home-roll some kind of preprocessor, but if I'm using an existing
      framework, I shouldn't have to.
  - I'd like to stick with GH Pages, because I don't want to figure out a
    different hosting provider. I haven't even used my own domain for my
    website, even though I've had it for years and use it for email.

I realized some time ago that GitHub had, at some point, launched support for
building GitHub Pages using a custom GitHub workflow. This meant that I was no
longer tied to using Jekyll for my personal website, and I could abandon all the
Gemfile and bundler stuff.

Every single time I've tried to run my website locally since college - when I
first fleshed out the Jekyll implementation - I got stuck re-learning how to
set up all the Ruby dependencies and just gave up.

## Options

Static site generation tools were really the only things I considered:

  - Hugo
  - Zola
  - Gatsby
  - pandoc

If I hadn't gone with pandoc, it probably would've been Hugo:

  - Zola hasn't been around long enough for me to be confident in its longevity
    as a tool, although admittedly it does ship as a self-contained binary, so
    there's some guarantee of longevity that you get from that
  - Gatsby isn't available as a self-contained binary
  - Hugo is, and has been around for quite some time

But ultimately, thinking back to all the times I tried to re-learn how Jekyll
worked and how much time I had to spend browsing Jekyll documentation... I
decided that Hugo most likely has too many features for me to need.

## Retrospective

I didn't do the best job of identifying all the requirements up front - I didn't
quite appreciate how much templating functionality I was going to need for what
I wanted to do. Fortunately pandoc's template system is sufficiently powerful
for me (and it wouldn't have been _too_ terrible hand-rolling it, I think).

I don't love that I had to use Pandoc Lua filters for some of the business
logic, in addition to `build.sh`, but it's a more low-dependency setup than any
of the alternatives. If I hadn't been able to do that, I would've written
something in either Python or JS to manage it, since those filters - one for
generating the list of blog posts, and another for attaching the date to each
blog post - need to do YAML front matter parsing.

I guess that's another thing I didn't realize upfront - that every page needs
some kind of metadata annotation, for things like titles and whatnot, which you
can't embed in the filename - at least, not if you want the filenames to be easy
to work with. (For the dates on blog posts, I've embedded those into the
filenames.)

There's definitely a bunch of good though:

  - hot reload, both on the build side and the in-browser rendering, makes the
    ergonomics of iterating _so_ much better; you literally can't go back once
    you've experienced it

  - rebuilding is fast, O(seconds), and there's easy opportunities for
    optimization there by parallelizing page builds; I just don't consider it
    worth doing because it would make debugging trickier

  - the framework itself consists of 111 lines of code right now, across
    `build.sh`, `lib/index.lua`, and `lib/post.lua` - that's simple, super easy
    to read, and will make iterating/debugging issues easy in the future

  - the GitHub workflow that continuously deploys this site is super simple: it
    calls `./build.sh` and then uses some GitHub-provided GitHub Actions to
    upload the built artifacts to GitHub Pages hosting

Overall, I think, a success!
