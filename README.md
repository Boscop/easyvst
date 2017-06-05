# easyvst
Helper crate for creating VST audio plugins easily

The purpose of this crate is to make it easy to get started writing VST plugins in Rust.
What does it do?
For now it helps with the complexities of plugin parameter management.

This crate provides an `EasyVst` trait that your plugin has to implement.
It's basically a middleman between the `Plugin` trait from the rust-vst2 crate and your plugin, requiring you to implement fewer methods and allowing you to organize your plugin state in a more convenient manner.

The examples demonstrate the usage:
- easygain: simple gain plugin without GUI
- conrodgain: gain plugin with simple GUI using conrod

Contributions and feature requests welcome!
