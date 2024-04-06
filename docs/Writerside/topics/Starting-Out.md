# Starting Out 

This is a learning exersise for my self. Join along if you want, but not be surprised if this doesn't work out!

<warning>
Also warning, I suck at spelling. So if there are typos, incorrect grammar, or other oddities, think of them as the spice of life and not that I have become dependent on spell check to save my butt.
</warning>

Starting out we're going to learn how to do a small in-memory key-value database.  
We are going to start very small and grow to be larger and include many different features.

I have a basic understanding of Rust and I have done a small amount of C++ programming in college.
I started my programming career as a Java developer. Since then my stack has changed to Node.JS, Typescript, Python, and some frontend frameworks.
Though I've always like getting into the nitty gritty of how things work I have never actually implemented a  low-level system on my own using a _"low-level"_ language.

So this is my progress as a developer to figure out what's going on and out to continue through it.

I am strongly opinionated on the idea that in order to truly understand why something is done, you need to go through the basics and do some things wrong in order to understand why they're done right.
I am also of the mind that a great way to integrate knowledge is to attempt to teach it to someone else.

Thus, I am taking the stance, that we create a horrible first database, in the style of making a tutorial.

This tutorial is not for making a production ready database, _Use one that has been worked on for years and is developed by some of the smartest people I know_ . This will be a simple server and a horrible one at that.
We can't have multiple clients connecting to it, the command structure is done strictly through strings, there might not even be persistent connections,
authentication is not even an idea yet, and we are going to try to learn some more Rust on the way...
Later we might implement this, or I might get burnt out by the end of this project. Who knows? Lets find out!

Things we are going to go through in this section:
* Implement TCP socket communication
* Implement a basic command structure through human-readable strings
* Implement basic hash map database key value system
* Implement basic error handling

Things that won't be present for now:
* Multi-tenancy
* Threading
* Acid Compliance


## Before you start

List the prerequisites that are required or recommended.

Make sure that:
- You have Rust [installed](https://www.rust-lang.org/tools/install)
- You have at least gone through the [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) book
- You know what a database is, though if you are interested reading this, you probably already know this ;).


## Tools used

I am using RustRover for this project. <i>Yes I shilled out for suite of tools from Intellij. No I currently do not regret it.</i>
You can use VSCode or any other editor you want. I just will not be providing any other information about them.

Also for writing this document, I am using WriterSide, _again from Intellij_, so if you want to see the source it is found [here](https://github.com/Expore-it-all/KVMem).


## Part 1 - Init project, the Sisyphus of development

Starting out we to start out our project. 

1. Execute the following command in the terminal:

   ```bash
    run this --that
   ```

2. Step with a [link](https://www.jetbrains.com)

3. Final step in part 1.

## Part 2

This is the second part of the tutorial:

1. Step 1
2. Step 2
3. Step n

## What you've learned {id="what-learned"}

Summarize what the reader achieved by completing this tutorial.

<seealso>
<!--Give some related links to how-to articles-->
</seealso>
