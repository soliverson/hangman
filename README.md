# Overview

This project is a Christmas Hangman Game that was developed to showcase and practice using the Rust programming language, especially when integrated with WebAssembly. My main goal was to create an interactive game where users are able to guess Christmas-themed words while adhering to Rust's principles of memory safety, ownership, and performance.

The purpose of developing this software was to deepen my understanding of Rust's syntax, tools, and its integration with front-end technologies. By working on this project, I learned more about using Rust for building web applications and I learned how to use Rust with other we web technologies like HTML, CSS, and JavaScript.

[Software Demo Video]https://www.loom.com/share/fd9f55ba21534950af4ccc6d614efa98?sid=3efe8fcf-338a-4335-a444-e85348a0f2fa

# Development Environment

- **Code Editor:** Visual Studio Code
- **Version Control:** Git and GitHub for publishing the project
- **WebAssembly Tooling:** `wasm-pack` for compiling Rust to WebAssembly
- **Programming Language:** Rust (edition 2021)
- **Libraries:**
  - `rand` for random word selection
  - `wasm-bindgen` for interacting between Rust and JavaScript
  - `web-sys` for manipulating the DOM

# Useful Websites

- [Rust Programming Language](https://www.rust-lang.org/) - Official Rust documentation
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/) - Guide for compiling Rust to WebAssembly
- [MDN Web Docs - DOM Manipulation](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model) - Reference for DOM-related concepts
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Examples for Rust programming
- [GitHub Pages Deployment](https://pages.github.com/) - Resource for deploying static web projects

# Future Work

- **Improve User Interface:** I could add more a hangman on the page that would draw itself everytime the user picked the wrong letter.
- **Mobile Screen:** I could make sure that the game is works on mobile devices.
- **Sound Effects:** I could add some sound effects for when the user makes correct and incorrect guesses to make the game more interactive.
