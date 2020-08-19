# RUST.ED

Rust.Ed is (will be) a lightweight, Rust based, terminal text editor.

## Objectives

+ Create a terminal text editor entirely in Rust
+ Keep program fooprint small and portable
+ Leverage Rust strengths of speed and safety
+ More 'Nano' than 'vim' (for now, at least)

## The TODO list

+ Add actual text editing functionality (only completely critical or whatever)
  - Add functionality for capturing session input
  - Support opening, modifying, and saving text files
  - Generate unamed file on boot without arguments, that can be immediately edited
  - (opt) Support reading/modifying files as raw
+ Add useful productivity features
  - Modify text in blocks with hot-keys
  - Configuration for setting hot keys; write, undo, exit, etc.
  - Line/document keyboard navigation
  - (opt) Mouse support
+ Crash recovery
+ Extensions (Syntax highlighting, etc)
