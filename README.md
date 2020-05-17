# game-of-life-wasm
Implement Conway's Game of Life using rust and WebAssembly

![AppVeyor](https://img.shields.io/appveyor/build/haochunchang/game-of-life-wasm?logo=AppVeyor)
![Travis (.org)](https://img.shields.io/travis/haochunchang/game-of-life-wasm?logo=Travis)
![GitHub repo size](https://img.shields.io/github/repo-size/haochunchang/game-of-life-wasm?logo=GitHub)

# Usage

* To build WebAssembly binary and Javascript API, run this command inside the project directory:
```bash
wasm-pack build
```

* To spin up a local http server at [http://localhost:8080/](http://localhost:8080/), run this command inside ```www/``` directory:
```bash
npm run start
```

* This repo was mainly adopted from the tutorial of [Rust and WebAssembly](https://rustwasm.github.io/docs/book/introduction.html).
