#!/usr/bin/env python
import pathlib
import subprocess
import invoke

import in_silico_cancer_cell.plot as insilico_plot
import matplotlib.pyplot as plt

RESULTS = pathlib.Path(__file__).resolve().parent / "figures" / "results"


@invoke.tasks.task()
def save_d3_plots(ctx: invoke.context.Context):
    subprocess.run(["yarn", "build"])
    for name in ("full-simulation-current", "voltage-protocol", "simulation-error"):
        subprocess.run(
            ["rsvg-convert", "-f", "pdf", "-o", f"figures/results/{name}.pdf", f"dist/plots/{name}/index.html"]
        )


@invoke.tasks.task()
def save_screenshot(ctx: invoke.context.Context):
    out = "figures/above-the-fold-screenshot.png"
    url = "http://localhost:4321/"
    size = "1800,1080"
    subprocess.run(f"chromium --headless --screenshot={out} --window-size={size} --hide-scrollbars {url}".split(" "))


@invoke.tasks.task()
def save_python_plots(ctx: invoke.context.Context):
    insilico_plot.set_results_folder(RESULTS)
    insilico_plot.plot_full_comparison()
    plt.show()
