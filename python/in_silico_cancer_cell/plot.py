import pathlib

import matplotlib.axes
import matplotlib.pyplot as plt

from in_silico_cancer_cell import CellPhase, ChannelCountsProblem, PatchClampData, PatchClampProtocol, setup_logging
import numpy as np

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


def plot_full_comparison():
    measurements = PatchClampData.pyload(PatchClampProtocol.Activation, CellPhase.G0)
    data = np.array(measurements.to_list())
    problem = ChannelCountsProblem.new(measurements)
    problem.precompute_single_channel_currents()
    single_channels = np.array(problem.get_current_basis())

    fig = plt.figure()
    axes: matplotlib.axes.Axes = fig.add_subplot(1, 1, 1)
    axes.plot(data)
    axes.plot((single_channels * [22, 78, 5, 1350, 40, 77, 19, 200, 17, 12, 13]).sum(axis=1))
    axes.set_xlabel("Time $t$ / s")
    axes.set_ylabel("Current $I$ / nA")
    fig.savefig(str(RESULTS / "data-vs-simulation.pdf"))


def set_results_folder(path: pathlib.Path):
    global RESULTS
    RESULTS = path
