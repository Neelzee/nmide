---
title: Everyone is better at development than me
subtitle: Creating a modular IDE
author: Nils Michael Fitjar
date: 2025-01-30
---

<!--toc:start-->
- [Introduction](#introduction)
  - [Introduction](#introduction)
- [Creating an IDE](#creating-an-ide)
  - [What is an IDE?](#what-is-an-ide)
  - [Integrated Development Environment](#integrated-development-environment)
  - [Magnolia](#magnolia)
  - [The Current IDE](#the-current-ide)
  - [Why create a new IDE](#why-create-a-new-ide)
  - [Why modular](#why-modular)
- [Features](#features)
  - [Features](#features)
    - [Goals](#goals)
  - [The Everything App](#the-everything-app)
  - [What is a module?](#what-is-a-module)
  - [How To Model A Module?](#how-to-model-a-module)
  - [Module Architecture](#module-architecture)
  - [Haskell "Trivial"-Example](#haskell-trivial-example)
  - [Haskell "Counter"-Example](#haskell-counter-example)
  - [Haskell "Counter"-Example `evtHandler`](#haskell-counter-example-evthandler)
  - [Tech stack](#tech-stack)
- [Challenges](#challenges)
  - [I Need Super Computer Time For My Featureless App](#i-need-super-computer-time-for-my-featureless-app)
  - [I Need Super Computer Time For My Featureless App](#i-need-super-computer-time-for-my-featureless-app)
  - [Blazing Fast Memory Leakage](#blazing-fast-memory-leakage)
  - [Core Flow diagram](#core-flow-diagram)
  - [Backend setup](#backend-setup)
  - [Refactoring](#refactoring)
  - [Granularity](#granularity)
  - [Module Families](#module-families)
- [Conclusion](#conclusion)
  - [Conclusion](#conclusion)
<!--toc:end-->

# Introduction

## Introduction


- Creating an IDE
    - Why create a new IDE
    - Why modular
- Features
    - Modularization
    - Tech stack
- Challenges
    - Refactoring
        - Granularity
        - Module Families
- Conclusion

# Creating an IDE

## What is an IDE?
\begin{quote}
  "It works on my machine." \textemdash Intern
\end{quote}

- The Terminal, The Text Editor and The Compiler
- Missing incomplete
    - Libraries
    - Environment Variables
    - Configurations
    - Scripts
- Make another program
    - CMake
    - Gradle
- Why not _integrate_ everything?

## Integrated Development Environment
- Easier to onboard new developers
- Other quality of life improvements
    - File explorer
    - Project manager
    - Version Control System integration
    - Syntax Highlighting
    - Integrated debugging
    - \dots


## Magnolia
- A research programming language being developed by
  Bergen Language Design Laboratory at the University of Bergen
- Uses something called `concepts`
- Similar to a Java interface.
- A concept declares
    - Types
    - Operations on those Types
    - Axioms that specify the behavior of the Operations
- A concept can use other concepts, and rename the Types and Operations
  in the concept, this is called renaming
- It is useful for a Magnolia Developer to be able to see the different
  renaming's of a concept


## The Current IDE
- It uses an old version of Eclipse
- Uses deprecated plugins
- Installation process is complex
- In INF220, two weeks is set aside for installation

## Why create a new IDE
- Current IDEs cannot have good support for experimental programming
languages
- VS Code or similar IDEs could deprecate needed functionality
- The installation process would then be complex
- Deep understanding of the used IDE is needed

## Why modular
- Magnolia is still in development
- The Magnolia tool chain is being developed in parallel
- Nothing to create an integrated development environment with, yet.
- Modularity allows for future discoveries to be quickly adopted into
the IDE
- Lowers the onboarding time for future maintainers

# Features

## Features

- Easy to extend with modules
    - Low bar to create a module
    - Easy to reason about how modules work together
- Prototype modules should include these features:
    - Can open, edit, delete files
    - Has LSP support
    - Can execute a program

### Goals
- Should be better than the current IDE
    - Easy installation
    - _Lasts long_
- Easy for the next ~~sucker~~ developer to change core functionality
- Have a good plugin developer experience

## The Everything App
- The Developer, the main userbase
    - Need modules to have _any_ experience
- The Module Developer, the secondary userbase
    - Language Agnostic Module Architecture
    - Good documentation and examples
    - Should prioritize the Module Developer Experience
- Maintainer Experience
    - Good documentation
    - Good testing
    - CI\/CD

## What is a module?

- Third Party Code that calls/is called by the application
    - Tailor made Scripting Language
    - An already existing programming language

## How To Model A Module?

- A module needs to:
    - Initialize some state
    - Update the state based on events
    - Render the view based on the state
- Module architecture is inspired by Elm and MVC
- A Module exposes a singular function:
    - Init -- to set the state
- Like Elm, events like, _onClick_, are sent as Events
- init $\to$ update $\to$ view $\to$ update $\to$ \dots cycle


## Module Architecture
\centering
\begin{tikzpicture}
\node (p) [rectangle, draw, minimum height=1cm, minimum width=2cm] at (0, 0) {Module};
\node (i) [rectangle, draw, minimum height=1cm, minimum width=2cm] at (4, 0) {Core};
\draw[->] (p.north) to[out=60, in=120] node[midway, above] {HTML/State} (i.north);
\draw[->] (i.south) to[out=-120, in=-60] node[midway, above] {Msg/State} (p.south);
\end{tikzpicture}

## Haskell "Trivial"-Example
```haskell
init :: CoreModification
init = emptyCoreModification
```

## Haskell "Counter"-Example
```haskell
init :: CoreModification
init = emptyCoreModification
  { uiModification = [
      CreateBtn
        { text = "Click"
        , onClick = throwEvent "ButtonClick"
        , id = "button-id"
        }
      ]
  , eventHandler = [("ButtonClick", evtHandler)]
  , stateModification = [
    AddField
      { field = "counter"
      , value = 0
      }
    ]
  }
```

## Haskell "Counter"-Example `evtHandler`
```haskell
evtHandler :: Event -> Core -> CoreModification
evtHandler _ c = case lookup "counter" of
  Just n -> emptyCoreModification
    { uiModification = [
      ModifyUi
        { id = "button-id"
        , text = "Click" ++ show (n + 1)
        }
      ]
    , stateModification = [
      UpdateField
        { field = "counter"
        , value = n + 1
        }
      ]
    }
  Nothing -> emptyCoreModification
```

## Tech stack
- Rust, because it is a low level system language
    - Compiler knows when a value is unused
    - Automatically _dropped_
    - No dangling pointers/null references
- Tauri, UI components can be created using JavaScript
- Splits the core application into two parts, loosely coupled parts
    - Frontend (JavaScript)
    - Backend (Rust)
- Communication is JSON-RPC, which, effectively, is the same as a
  client-server
- Allows for modules in two different languages, with little effort.
  I hoped.


# Challenges
## I Need Super Computer Time For My Featureless App
- JavaScript is more _unsafe_ than Rust, due of a lack of typing
- Need to decode the output from the modules, and catch any exceptions
- When implementing the `init` $\to$ update $\to$ view - cycle, I tested with
  a _basic_ module, which should only display "Hello, World!"
    - The module initialized the state
    - It rendered the view
    - Somehow triggered an update
    - Which triggered a re-render
    - Which triggered an update
    - Which triggered a re-render
    - \dots

## I Need Super Computer Time For My Featureless App
\begin{figure}
\centering
\includegraphics[width=0.8\textwidth]{./pics/memory-allocation-zoomed.png}
\end{figure}
It's just $140.6594 Terabytes$, which is a meager $15$ \% of the total memory of NASA's
supercomputer

## Blazing Fast Memory Leakage
- The Rust ABI is not protected by their SemVer notation
- This means that even a patch to the Rust compiler can break a
  Rust Module
- Can be fixed by using a Rust Library: `abi_stable`
- Had to use `ManuallyDrop` for more complex types, which disables
  the automatic drop
- Fixed by having Rust modules only reference the state, meaning
  after update and view, the module can be safely dropped

## Core Flow diagram
\centering
\includegraphics[width=0.95\textwidth]{./pics/mini-core-cycle.png}

## Backend setup
\centering
\includegraphics[width=0.95\textwidth]{./pics/backend-setup.png}

## State Cycle
\centering
\includegraphics[width=0.9\textwidth]{./pics/state-cycle.png}

## Refactoring
## Granularity
## Module Families
# Conclusion

## Conclusion
