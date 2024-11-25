# README

러스트 기본 입문 책 : https://doc.rust-lang.org/book/ch01-02-hello-world.html 
한국어 버전 : https://doc.rust-kr.org/


예제로 배우는 러스트 : https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html

https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

https://github.com/emilk/egui/issues/996

serial port libaray : https://crates.io/crates/serialport  
gui library : egui
 - egui demo : https://www.egui.rs/#Demo
 - docs : https://docs.rs/egui/latest/egui/
 - git : https://github.com/emilk/egui


## WSL Serail connect commands
``` powershell
> usbipd list
> usbipd bind --busid $portName
> usbipd attach --wsl --busid $portName
 ```


## Getting started

St art by clicking "Use this template" at https://github.com/emilk/eframe_template/ or follow [these instructions](https://docs.github.com/en/free-pro-team@latest/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template).

Change the name of the crate: Choose a good name for your project, and change the name to it in:
* `Cargo.toml`
    * Change the `package.name` from `eframe_template` to `your_crate`.
    * Change the `package.authors`
* `main.rs`
    * Change `eframe_template::TemplateApp` to `your_crate::TemplateApp`
* `index.html`
    * Change the `<title>eframe template</title>` to `<title>your_crate</title>`. optional.
* `assets/sw.js`
  * Change the `'./eframe_template.js'` to `./your_crate.js` (in `filesToCache` array)
  * Change the `'./eframe_template_bg.wasm'` to `./your_crate_bg.wasm` (in `filesToCache` array)

Alternatively, you can run `fill_template.sh` which will ask for the needed names and email and perform the above patches for you. This is particularly useful if you clone this repository outside GitHub and hence cannot make use of its
templating function.

### Learning about egui

`src/app.rs` contains a simple example app. This is just to give some inspiration - most of it can be removed if you like.

The official egui docs are at <https://docs.rs/egui>. If you prefer watching a video introduction, check out <https://www.youtube.com/watch?v=NtUkr_z7l84>. For inspiration, check out the [the egui web demo](https://emilk.github.io/egui/index.html) and follow the links in it to its source code.

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Deploy
1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
3. Upload the `dist` directory to any of the numerous free hosting websites including [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).
4. we already provide a workflow that auto-deploys our app to GitHub pages if you enable it.
> To enable Github Pages, you need to go to Repository -> Settings -> Pages -> Source -> set to `gh-pages` branch and `/` (root).
>
> If `gh-pages` is not available in `Source`, just create and push a branch called `gh-pages` and it should be available.
>
> If you renamed the `main` branch to something else (say you re-initialized the repository with `master` as the initial branch), be sure to edit the github workflows `.github/workflows/pages.yml` file to reflect the change
> ```yml
> on:
>   push:
>     branches:
>       - <branch name>
> ```

You can test the template app at <https://emilk.github.io/eframe_template/>.

## Updating egui

As of 2023, egui is in active development with frequent releases with breaking changes. [eframe_template](https://github.com/emilk/eframe_template/) will be updated in lock-step to always use the latest version of egui.

When updating `egui` and `eframe` it is recommended you do so one version at the time, and read about the changes in [the egui changelog](https://github.com/emilk/egui/blob/master/CHANGELOG.md) and [eframe changelog](https://github.com/emilk/egui/blob/master/crates/eframe/CHANGELOG.md).




시리얼 예제

use std::time::Duration;

fn serialExample() {
    let ports = serialport::available_ports().expect("No ports found!");

    for p  in ports {
        println!("{}", p.port_name);
    }


    let mut serial = serialport::new("COM15", 9600)
    .timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");

    let output = "This is output".as_bytes();
    serial.write(output).expect("Writed Failed");


    let mut serial_buf : Vec<u8> = vec ! [0;32];
    serial.read(serial_buf.as_mut_slice()).expect("Found no data!");
}


fn main() {
    println!("Hello, world!");
    serialExample();
}