import matplotlib.axes
import matplotlib.pyplot as plt
import pathlib

RESULTS = pathlib.Path.cwd()


def plot_measurement():
    fig = plt.figure()
    axes: matplotlib.axes.Axes = fig.add_subplot(1, 1, 1)
    axes.plot()
    axes.set_xlabel("")
    axes.set_ylabel("")
    axes.legend()
    fig.savefig(str(RESULTS / "plot.pdf"))


def set_results_folder(path: pathlib.Path):
    global RESULTS
    RESULTS = path
