# easyvst
Helper crate for creating VST audio plugins easily

The purpose of this crate is to make it easy to get started writing VST plugins in Rust.

### What does it do?

It provides a higher-level way of writing VSTs that abstracts away the complexities of plugin parameter management, notifying the host etc.

In my experience, a lot of (C++) VSTs get parameter management wrong, and this leads to problems with automation, e.g. parameters aren't automatable even though they should be.
With `easyvst`, all your plugin parameters are already automatable (and your plugin will get notified when they change) so you don't have to implement this manually.

This crate provides an `EasyVst` trait that your plugin has to implement.
It's basically a middleman between the `Plugin` trait from the [vst](https://crates.io/crates/vst) crate and your plugin, requiring you to implement fewer methods and allowing you to organize your plugin state in a more convenient manner.

The examples demonstrate the usage:
- easygain: simple gain plugin without GUI
- conrodgain: gain plugin with simple GUI using conrod

Contributions and feature requests welcome!
