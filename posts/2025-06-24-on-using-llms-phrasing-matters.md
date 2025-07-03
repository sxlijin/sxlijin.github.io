# On Using LLMs: Phrasing Matters

While talking with a friend recently, it occurred to me that most people using LLMs may not necessarily think about how their **choice of phrasing** in a prompt might influence the quality or detail of the response. Consider the following three versions of the same prompt:

1. What happens when I use a credit card to buy something?
2. What underlying processes are invoked when I pay with a credit card?
3. Describe the sequence of actions triggered during a credit card purchase, including authorization, clearing, and settlement phases, and what happens if any individual action fails.

Go on, try it out! I'll wait.

(NB: keep in mind that LLM providers are now personalizing their consumer-facing LLM products, and that will also materially influence the kind of response you get back.)

You should notice two things:

1. The responses to each of version 1, version 2, and version 3 are mutually interchangeable; that is, the response to version 3 could easily be used to respond to version 1.
2. The response to each version reflects how in-depth the response is expected to be, as implied by the phrasing of the prompt.

So how does an LLM - a deterministic-ish black box - decide how in-depth its response should be? You can think about this largely as a person would, when receiving these prompts:

- A person asking version 1 is not likely to know what the difference between the clearing and settlement phases are. They may not even know what a chargeback is.
- By contrast, a person asking version 3 clearly knows what clearing and settlement phases are. They likely know that a chargeback is a way for a consumer to get money back. They probably don't know how chargebacks work with respect to clearing/settlement.

The other thing you should think about is that **in the absence of context signals, the LLM still needs to make assumptions** about what kind of response the user expects from it.

More precisely, the people building the LLM need to make decisions about what level of detail an LLM should respond with by default[^level-of-detail] (the LLM is not making these decisions on the fly, at runtime; these decisions are implicitly encoded into the model as a consequence of the training data and system prompt).

[^level-of-detail]: It's not just the level of detail either: they also need to think about whether the LLM should be matter-of-fact and curt, whether it should [respond with affirmation](https://openai.com/index/expanding-on-sycophancy/), and whether it should suggest further conversation! This is why ChatGPT will sometimes respond with "Which response do you prefer better?", because they're trying to collect feedback on what their users prefer.

This idea, that the choice of phrasing matters, is not a new one. See for example [WIRED's "5 Levels" series](https://www.youtube.com/playlist?list=PLibNZv5Zd0dyCoQ6f4pdXUFnpAIlKgm3N), where experts explain complex subjects with 5 different levels of complexity, and [Randall Munroe's _Thing Explainer_](https://xkcd.com/thing-explainer/), which uses only the 1,000 most common English words to explain helicopters, tectonic plates, etc.

It is, though, easy to forget how many decisions we're implicitly making - how many decisions we don't think about - when we choose the phrases we use in a prompt.