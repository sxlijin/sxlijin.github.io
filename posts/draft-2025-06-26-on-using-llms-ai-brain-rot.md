a colleague asked me my thoughts on how to stay competent in the age of AI, particularly when it's possible to offload reasoning tasks to it. (inspired by mit's recent study suggesting cognitive decline when using llms: https://arxiv.org/pdf/2506.08872v1)

to the societal problem, i have no answer.

personally, my answer is to continue to engage in active learning and to proactively explore a topic at different levels of depth.

interjection: my experience with ethan, SAT studying, and watching him continue to blindly trust an LLM for sat english answers even after showing him an example where it was _demonstrably wrong_ (he took a screenshot of the essay answers, and got a wrong answer, and then i told him to type it out and he got the correct answer) - also watching ethan not know how to ask it questions that he needed to be asking, because he didn't understand how to ask them with a higher level of introspection

aside: chelsea troy's piece on ai in her classroom https://chelseatroy.com/2025/05/14/the-homework-is-the-cheat-code-genai-policy-in-my-computer-science-graduate-classroom/

> A thing you’ve likely deduced is that I don’t forbid the use of the tools in my class. Why not: first of all I can’t, and second of all I don’t want to.  
>   
> **I can’t:** Branding the use of generative AI as cheating and then attempting to go after every student who uses it is an amount of work I don’t have the bandwidth for. No matter how hard I tried, I couldn’t possibly close the net on this.
> 
> **I don’t want to:** First of all, I do believe there is some truth to the idea that banning these tools in the classroom represents an outdated view of the environment students will walk into professionally. If I tell them “don’t use this” and the zeitgeist is telling them “you are so screwed if you don’t use this,” it’s my word against, like, Zuck’s and Melon’s. Who are they going to believe, do you think? Furthermore, suppose I tell students “don’t use this,” knowing full well that in the vast majority of cases I could not build a solid case for discerning whether they used it. Then I ask them in surveys how they used it. What’s a rational actor going to say? They’ll say they didn’t use it. Then I get no data about _how_ they used it. Again: these tools are new. We do not know how they integrate with an academic context yet. I have no interest in incentivizing my students to withhold that information from me.

things that are in the back of my mind as i use LLMs:
- what failure modes do i need to guard against? in particular, am i already an expert in the topic i am prompting about or do i need to build expertise in?
- how up-to-date is the knowledge corpus?

my personal pov on llms is that it allows me to shift cognition to *valuable* tasks. i know how to read `man` pages, and i know what `ln` and `tar` do, but i will never remember what "source file" and "target file" mean in the `ln` man page nor what the current `tar` invocation to list files in a `.tar.gz` is, and i certainly do not consider this knowledge critical to my engineering skills.

by contrast, if i'm going to swap out the implementation of TLS that the library we switch uses, i will absolutely use an LLM to help me explore the research area, but i will not rely solely on it, in the same way that i will never ask a colleague an engineering question and take their answer at face value without repeating the train of thought myself.