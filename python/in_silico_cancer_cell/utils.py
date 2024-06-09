import numpy as np


def moving_average(a: np.ndarray, n=3) -> np.ndarray:
    """Generate a moving average

    Args:
        a (np.array): original datum
        n (int, optional): Average stencil order. Defaults to 3.

    Returns:
        np.ndarray: averaged array
    """
    ret = np.cumsum(a, dtype=float, axis=0)
    ret[n:] = ret[n:] - ret[:-n]
    return ret[n - 1 :] / n
