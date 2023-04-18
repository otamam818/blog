# Personal Finance Tracker - Introduction
I want to make the tracking of personal finances a breeze. In some contemporary finance trackers, I feel like there
are still some *unimplemented features* that I would have liked having in a personal finance tracker app, and upon thinking about it, I decided I wanted
to implement my own personal finance tracker.

What are these *unimplemented features*? As I don't want to come off as oversmart beyond my understanding of what is avaiable, I am not ready to
disclose that just yet. Despite that, I do want to talk about a core focus of mine which I hope affects future features and experiences: ergonomics.

## Why ergononmics?
As a recent fan of the **Axum** framework of Rust, I found new value in making things ergonomically efficient. I realized that, while other
web servers may favor explicitness or implementation speeds, I haven't encountered a framework like Axum that balances both whilst providing
type-safety guarantees.

It made me think: What if I can make a personal finance tracker with this as a core value? I know for sure that I would find it meaningful to use, and
if I make it freely available to future users, they might find value in using it too.

Despite the ambition, I found myself blocked in my implementation few days ago. This blog post aims to explain why I got blocked, and why a
restructuring of the core structures is necessary.
