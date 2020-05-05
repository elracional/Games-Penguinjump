# Games 
# Penguin Jumps 

## Language

The programming language used for this project is **Rust***.

**Rust** is used to compile to a binary `.wasm`, which is finally technology **WebAssembly**. This means that is a Web Application.

As secondary tools, **HTML** and **Bootstrap $** were used to give it design and **JavaScript** to add music and sound effects. thus showing on scrren the movements that the character of the game is making.

## Installation

In order to carry out the execution of this project, some points must be taken into consideration:

#### 1. Rust Language

[**Rust Lang**](https://www.rust-lang.org/tools/install "Instalación") must be downloaded and installed, whose isntallation will depend on the OperatingSystem.
*The tests were done in a Linux environment and its use is recommended for ease of testing*.

#### 2. WASM Compiler

In order to compile our Rust code to code ** WebAssembly ** we need to install that compilation option, for this we will use `rustup` by executing the following commands in the terminal:

	rustup install nightly
	rustup target add wasm32-unknown-unknown
	rustup update
	cargo +nightly install wasm-bindgen-cli
	cargo install -f cargo-web

#### 3. Testing

To verify its correct installation you can run the command in the terminal:

	cargo web --version

#### 4. Start the Server

To verify that there are no problems running an application of these, go to the project folder and run the following command:

	cargo web start --target=wasm32-unknown-unknown


A local server will be created where you can run the project in `localhost:8000` or the port that you dial in the terminal. You can easily open the link from the terminal.

The first time it runs it will automatically download all the necessary packages and it may take a while.

## Penguin Jumps

The project is about a penguin that is on screen and must jump through them to collect fish that appear in the upper right corner. To collect them it is only necessary to touch them and the score will auto-increase.

Initially it has two lives, which are marked in the upper left of the screen, but throughout the game they can appear more, although to collect them it is necessary to stand on them and press the button down. A maximum of 4 lives can be accumulated.

Fish, lives and platforms (both width and position) will appear randomly on the screen.

As fish are collected, the speed of the platforms will increase as follows:

* 10 Fish, speed is doubled
* 50 Fish, speed is doubled
* 100 Fish, speed is doubled
* 175 Fish, speed is doubled

The goal is to collect the most fish.

### Controls

| Tecla | Función |
|-|-|
Up, W | Jump
Down, S | Crouch
Left, A | Left
Right, D | Right
SPACE | Start/Restart
S + W, Down + Up | Go Down

