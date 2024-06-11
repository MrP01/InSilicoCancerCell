import clarabel
import numpy as np
from scipy import sparse
import scipy

from . import CellPhase, ChannelCountsProblem, PatchClampData, PatchClampProtocol
from .utils import moving_average


def load_problem(n):
    measurements = PatchClampData.pyload(PatchClampProtocol.Activation, CellPhase.G0)
    data = moving_average(np.array(measurements.to_list()), n)
    # data = np.array(measurements.to_list())[::12]
    problem = ChannelCountsProblem.new(measurements)
    problem.precompute_single_channel_currents()
    single_channels = moving_average(np.array(problem.get_current_basis()), n)
    # single_channels = np.array(problem.get_current_basis())[:, (3,)]
    # single_channels = np.concatenate([single_channels, np.ones((single_channels.shape[0], 1))], axis=1)
    return single_channels[: data.shape[0]], data


def solve_as_quadratic_problem(single_channels, data):
    # LSQ formulation min 1/2 ||Rx - d||^2
    R = single_channels
    d = data

    # converted into QP formulation
    P = sparse.csc_matrix(R.T @ R)
    q = -R.T @ d
    A = -sparse.identity(11).tocsc()
    b = np.zeros(11)
    cones = [clarabel.NonnegativeConeT(11)]

    # solve
    settings = clarabel.DefaultSettings()
    solver = clarabel.DefaultSolver(P, q, A, b, cones, settings)
    solution = solver.solve()
    return np.array(solution.x)


def solve_with(single_channels, data, method):
    if method == "lstsq":
        channel_counts, res, rank, s = np.linalg.lstsq(single_channels[: len(data), :], data, rcond=None)
    elif method == "nnls":
        channel_counts, rnorm = scipy.optimize.nnls(single_channels[: len(data), :], data)
    elif method == "qp":
        channel_counts = solve_as_quadratic_problem(single_channels, data)
    elif method == "langthaler":
        channel_counts = np.array([22, 78, 5, 1350, 40, 77, 19, 200, 17, 12, 13])
    channel_counts = channel_counts.round().astype(int)
    print(f"Best fit: {channel_counts}")
    return channel_counts
