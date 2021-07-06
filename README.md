# Oxidized-Soundboard
simple soundboard written in Rust
This is my first projekt written in Rust, I won't maintain it, unless I find an issue or bug that annoys me.
So please don't open any issues or PR's;
If you want to add to it you are free to fork it or use it as a starting off point.

It uses iced as it's GUI library, rodio for sound decoding and output and serde_yaml + home for the config files.

How to:
  - add files : 
    - either drag and drop them in and enter a name
    - or press add and enter the path by hand
  - play sounds as audio input:
    - install VB-cables or any other equivalent software and use it's virtual input as output
  - build :
    - just the usual cargo build --release 

