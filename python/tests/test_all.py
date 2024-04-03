import in_silico_cancer_cell as insilico


def test_ramp_simulation():
    cell = insilico.A549CancerCell.new()
    error = cell.evaluate(insilico.PatchClampProtocol.Ramp, insilico.CellPhase.G0)
    assert error >= 0
