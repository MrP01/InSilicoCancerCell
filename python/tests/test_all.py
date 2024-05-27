from in_silico_cancer_cell import (
    A549CancerCell,
    CellPhase,
    InSilicoMethod,
    PatchClampProtocol,
    PatchClampData,
    find_best_fit_for,
    setup_logging,
)


def test_ramp_simulation():
    cell = A549CancerCell.new()
    error = cell.evaluate(PatchClampProtocol.Ramp, CellPhase.G0)
    assert error >= 0


def test_projection_solver():
    measurements = PatchClampData.pyload(PatchClampProtocol.Activation, CellPhase.G0)
    setup_logging()
    solution = find_best_fit_for(measurements, InSilicoMethod.Projection)
    print("Found solution", solution)
