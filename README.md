# TimeSpent

<img src="https://i.imgur.com/6YpjZ0K.png" height="650">

**A simple GUI rust application that keeps track of how much time you spend on each application.**

## Installation
[Click here](https://github.com/i-program-stuff/TimeSpent/releases/download/1.2/TimeSpent.Setup.v1.2.exe) to download the Setup.
Extract the file and then Double click on it.
<br>
Follow the setup and that should be it. TimeSpent should be in your Start Menu.

Binary for linux coming soon...

## Usage
The Daemon binary tracks the amount of time you used an application. Make sure it's running in the background.

In the Gui, You can right click the names of applications to open a Context menu.
In the context menu, you have options to Rename, Hide/Unhide, and Delete Items.

You can find a Graph of how much time you spent on an application when you click on its name or by right clicking on its name and going to "More Info".

xdotool is needed on Linux for this program to work.

## Building from Source

Install [Rust](https://www.rust-lang.org/tools/install) if you don't have it already

``` bash
git clone https://github.com/i-program-stuff/TimeSpent.git
cd TimeSpent/
cargo build --bin daemon --release
cargo build --bin gui --release
```

You should find you executables in the /target/release/ directory

Rename the daemon binary to "TimeSpentDaemon" and make it a start-up application.

## TODO

- Upload to AUR.
- Add proper Settings and Config
