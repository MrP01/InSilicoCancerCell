import pathlib

import in_silico_cancer_cell.plot as insilico_plot

RESULTS = pathlib.Path(__file__).resolve().parent / "figures" / "results"


if __name__ == "__main__":
    insilico_plot.set_results_folder(RESULTS)
    insilico_plot.plot_measurement()
