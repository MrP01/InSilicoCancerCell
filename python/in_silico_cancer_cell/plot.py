import pathlib

import matplotlib.axes
import matplotlib.pyplot as plt
import numpy as np

from . import setup_logging
from .fit import load_problem, solve_with

RESULTS = pathlib.Path.cwd()
setup_logging()


def plot_measurement():
    fig = plt.figure()
    axes: matplotlib.axes.Axes = fig.add_subplot(1, 1, 1)
    axes.plot()
    axes.set_xlabel("")
    axes.set_ylabel("")
    axes.legend()
    fig.savefig(str(RESULTS / "plot.pdf"))


def plot_full_comparison(method="nnls", n=800):
    single_channels, data = load_problem(n)
    channel_counts = solve_with(single_channels, data, method)
    time = np.linspace(0, 9.901, single_channels.shape[0])
    fig = plt.figure(figsize=(8, 4))
    axes: matplotlib.axes.Axes = fig.add_subplot(1, 1, 1)
    axes.plot(time[: len(data)], data, label="Measurements")
    axes.plot(time, (single_channels * channel_counts).sum(axis=1), label="Simulation")
    axes.set_xlabel("Time $t$ / s")
    axes.set_ylabel("Current $I$ / pA")
    axes.legend()
    fig.savefig(str(RESULTS / "data-vs-simulation.pdf"))


def set_results_folder(path: pathlib.Path):
    global RESULTS
    RESULTS = path
