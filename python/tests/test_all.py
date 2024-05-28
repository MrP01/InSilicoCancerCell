from in_silico_cancer_cell import (
    A549CancerCell,
    CellPhase,
    InSilicoMethod,
    ChannelCountsProblem,
    PatchClampProtocol,
    PatchClampData,
    find_best_fit_for,
    setup_logging,
)
import numpy as np

setup_logging()


def test_ramp_simulation():
    cell = A549CancerCell.new()
    error = cell.evaluate(PatchClampProtocol.Ramp, CellPhase.G0)
    assert error >= 0


def test_projection_solver():
    measurements = PatchClampData.pyload(PatchClampProtocol.Activation, CellPhase.G0)
    solution = find_best_fit_for(measurements, InSilicoMethod.Projection)
    print("Found solution", solution)


def test_current_basis_matrix():
    measurements = PatchClampData.pyload(PatchClampProtocol.Activation, CellPhase.G0)
    problem = ChannelCountsProblem.new(measurements)
    problem.precompute_single_channel_currents()
    print("Matrix", np.array(problem.get_current_basis()))
