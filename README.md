# Electrophysiological Cancer Cell Model

![PyPI - Version](https://img.shields.io/pypi/v/in-silico-cancer-cell)

Attempt to model an A549 cancer cell's ion channels using an HMM (Hidden Markov Model) and simulation of voltage + current development accross the membrane of the cell.

This software comes in three flavours:

- to run the `main.rs` simulation, do `cargo run`,
- to compile the Python module, do `maturin develop --features pyo3`,
- to precompile for the Astro dashboard, do `yarn run wasm-pack build frontend`.

## A visual to capture your interest:

![Ion Channels](https://journals.plos.org/ploscompbiol/article/figure/image?size=large&download=&id=10.1371/journal.pcbi.1009091.g002)
(Image source: [here](https://doi.org/10.1371/journal.pcbi.1009091.g002)).

This computational model is based on [Langthaler et al., 2021](https://journals.plos.org/ploscompbiol/article?id=10.1371/journal.pcbi.1009091): **A549 in-silico 1.0: A first computational model to simulate cell cycle dependent ion current modulation in the human lung adenocarcinoma**.
