00
74
100

I have an inkbird samrt thermo-hygrometer
model ITH-13-B 
IC 303680ith13b 
fcc id 2ayzd-ith13b 
power 2aaa/ev
made in china 
fcc ce ic rohs 
R 001-a16261 

an inkbird bluetooth low energy project to set up a low power way to catch broadcasts from temperature humidity monitor and store it in a git repository 

my idea is everything in this project should live within podman / podman compose and be completely podman 6 ready whenever it becomes available 
we should not need anything from the host computer 
and anything we install should be within podman itself 
assume fedora 33 or later 34(?) on an acer swift go 14 with amd 8845hs 
but really the computer shouldn't matter
except now that I think about it maybe it does matter? 
assume the computer has bluetooth 
not sure how you would pass it to a container 
let me know if you need sudo 
but yes, do what is necessary 
the main goal is to add no more battery stress on the inkbird 
i believe it is already sending out bluetooth low energy 

it is critical for you to give me FULL files for all files that need to change
as well as the path for the file 
because sometimes the same file name exists in multiple places 

review `dump.txt`, if it exists, for full source code as it exists now

prefer compiled languages over scripting languages like python
but if it is absolutely a show stopper I guess you can use python 
regardless of what programming language you use 
always use the latest technology
always use latest packages 
always use latest versions of github actions 
always use latest version of containers 
speaking of containers, we use containerfile not docker file 
and definitely we don't have any file called docker compose 
in fact mentioning the word docker anywhere is a sin 
because our project must be vendor neutral 
also speaking of packages 
do not use any packages that are non-free 
or free only for non-commercial use 
our project will be AGPL-v3 licensed 
and we will not pull anything with nagware like moq 
I know you will likely not use dotnet csharp for this 
but I am mentioning this just in case 

I am thinking rust would be a first preference with the code 
and github actions should be as slim as possible
prefer to outsource most of the work to some kind of bash script 
for building for testing etc 
and the github actions simply calls these bash scripts 

implement best engineering practices 
solid principles and so on 
this repo will be public at https://github.com/kusl/myinkbird
so we want to make sure it is a good learning experience 
disclose in the readme that this is LLM assisted development
idk if this euphamism is enough 
err on the side of caution

also please make sure to document everything as we go along
not just this original prompt but also 
as we go along and add more changes 
we should update the readme and the documentation 
and architecture decision record as well 

from https://adr.github.io/ 

```
Motivation and Definitions

An Architectural Decision (AD) is a justified design choice that addresses a functional or non-functional requirement that is architecturally significant. An Architecturally Significant Requirement (ASR) is a requirement that has a measurable effect on the architecture and quality of a software and/or hardware system. An Architectural Decision Record (ADR) captures a single AD and its rationale; Put it simply, ADR can help you understand the reasons for a chosen architectural decision, along with its trade-offs and consequences. The collection of ADRs created and maintained in a project constitute its decision log. All these are within the topic of Architectural Knowledge Management (AKM), but ADR usage can be extended to design and other decisions (“any decision record”).

The aim of the GitHub adr organization is to:

    Motivate the need for and benefits of AD capturing and establish a common vocabulary.
    Strengthen the tooling around ADRs, in support of agile practices as well as iterative and incremental engineering processes.
    Provide pointers to public knowledge in the context of AKM and ADRs.

ADRs in the Media

    (in German) Architekturentscheidungen sichtbar und nachvollziehbar gestalten at JavaLand 2026 (2026-03-10).
    The Azure Well-Architected Framework features ADRs and this website (2024-11-10).
    Love Unrequited: The Story of Architecture, Agile, and How Architecture Decision Records Brought Them Together, Michael Keeling in the Pragmatic Designer column of IEEE Software Vol. 39 Issue 4 (2022) (PDF)
    Architectural decision capturing is positioned as one of the essential activities in Design Practice Reference, a LeanPub e-Book.
    Chapter 3 of “Patterns for API Design: Simplifying Integration with Loosely Coupled Message Exchanges” in the Addison Wesley Signature Series at Pearson features six narratives guiding through the conceptual level of API design: 29 recurring decisions with options and criteria. Learn more in this blog post.
    (in German) Gut dokumentiert: Architecture Decision Records by @obfischer published at heise online.

Background Information

The work in the adr organization is based on the guidelines and principles in Sustainable Architectural Decisions by Zdun et al., for instance the Y-statement format suggested in that article.

More general background information and ADR guidance is available:

    A comparison of seven templates can be found in “Architectural Decision Guidance Across Projects — Problem Space Modeling, Decision Backlog Management and Cloud Computing Knowledge”, a WICSA 2015 conference paper.
    Architectural Decisions — The Making Of provides a history on architecture decision recording since the late 1990, as well as examples and guidance for providing decision rationale.
    Documenting Architecture Decisions is the blog post from 2011 by Michael Nygard that popularized the concept.
    Architectural Decision Records (ADR): Open & Transparent Decision History is a practice in the Open Practice Library.
    An AWS Prescriptive Guidance recommends using architectural decision records to streamline technical decision-making for a software development project.
    Architecture Decision Records in Action by Michael Keeling (IBM Watson Group) and Joe Runde (IBM) [YouTube] is a presentation that includes empirical numbers.
    ADRs and Architecture Stories is part of a video series by Mark Richards explaining ADRs, starting from Nygard’s template.
    Additional pointers and resources can be found on the web page Architectural Knowledge Management (AKM).
```

add as many unit tests as possible 
lets make this the best project it can be 

oh and please also give me a concise "instruction" that I can tack at the end of the "instructions" for this project so you don't forget what we talked about in this conversation 

whenever you give me a new file, make sure to always give me the full relative path as well 
and always give me FULL files, never tell me to change line x in file y 
if there are a lot of files changed 
for example in this first prompt
give me a full zip to unzip on project root and tell me which files I would need to delete 
