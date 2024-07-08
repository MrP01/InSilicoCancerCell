#!/usr/bin/env python
import pathlib
import subprocess
import time

import invoke
import matplotlib.pyplot as plt
from in_silico_cancer_cell.fit import load_problem, solve_with
from in_silico_cancer_cell.plot import plot_full_comparison, set_results_folder
import numpy as np

RESULTS = pathlib.Path(__file__).resolve().parent / "figures" / "results"


@invoke.tasks.task()
def save_d3_plots(ctx: invoke.context.Context):
    subprocess.run(["yarn", "build"])
    for name in ("full-simulation-current", "voltage-protocol", "simulation-error", "dt-plot", "delta-tolerance"):
        subprocess.run(
            ["rsvg-convert", "-f", "pdf", "-o", f"figures/results/{name}.pdf", f"dist/plots/{name}/index.html"]
        )


@invoke.tasks.task()
def save_screenshot(ctx: invoke.context.Context):
    out = "figures/above-the-fold-screenshot.png"
    url = "http://localhost:4321/"
    size = "1920,1080"
    subprocess.run(
        f"chromium --headless --screenshot={out} --window-size={size} --hide-scrollbars --timeout=1100 {url}".split(" ")
    )


@invoke.tasks.task()
def save_python_plots(ctx: invoke.context.Context, method="nnls"):
    set_results_folder(RESULTS)
    plot_full_comparison(method)
    plt.show()


@invoke.tasks.task()
def compare_solvers(ctx: invoke.context.Context, n=800):
    single_channels, data = load_problem(n)
    single_channels_raw, data_raw = load_problem(1)
    rmses = {}
    runtimes = {}
    for method in ("lstsq", "nnls", "qp", "langthaler"):
        start = time.monotonic()
        channel_counts = solve_with(single_channels, data, method)
        if not all(c >= 0 for c in channel_counts):
            print(f"{method} returned invalid result", channel_counts)
            continue
        runtimes[method] = time.monotonic() - start
        diff = (single_channels_raw * channel_counts).sum(axis=1) - data_raw
        rmses[method] = np.sqrt((diff**2).sum() / len(diff))

    print(60 * "-")
    for method in runtimes.keys():
        print(f"{method:12} error: {rmses[method]:.2f}\t runtime: {runtimes[method] * 1000:.2f} ms")
    print(60 * "-")
