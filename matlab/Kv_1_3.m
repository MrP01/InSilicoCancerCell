%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM Kv1.3
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space] = Kv_1_3()
  %% parameter definitions
  % rate constants
  a = 0.488;
  b = 0.043;
  c = 0.003;
  d = 0.00008;
  const_a = 280.035;
  const_b = 1.648;
  m = 27.530;
  n = 17.528;
  p = 174.961;
  q = 1016.330;
  %single channel conductance
  g = 15 * 10^-12;

  %% transition rates
  alpha_rate = @(V)a * exp(V * 1e3 / m);
  beta_rate = @(V)b * exp(-V * 1e3 / n);
  gamma_rate = @(V)c * exp(V * 1e3 / p);
  phi_rate = @(V)d * exp(-V * 1e3 / q);

  % transition probabilities = rate constants * ms
  alpha = @(V, dt) alpha_rate(V) * dt * 1e3;
  beta = @(V, dt) beta_rate(V) * dt * 1e3;
  gamma = @(V, dt) gamma_rate(V) * dt * 1e3;
  phi = @(V, dt) phi_rate(V) * dt * 1e3;
  A = @(dt) const_a * dt * 1e3;
  B = @(dt) const_b * dt * 1e3;

  %% define state space model:
  P_KV_1_3 = @(V, dt) [1 - 4 * alpha(V, dt), beta(V, dt), 0, 0, 0, 0, 0;
                       4 * alpha(V, dt), 1 - 3 * alpha(V, dt) - beta(V, dt), 2 * beta(V, dt), 0, 0, 0, 0;
                       0, 3 * alpha(V, dt), 1 - 2 * alpha(V, dt) - 2 * beta(V, dt), 3 * beta(V, dt), 0, 0, 0;
                       0, 0, 2 * alpha(V, dt), 1 - alpha(V, dt) - 3 * beta(V, dt), 4 * beta(V, dt), 0, 0;
                       0, 0, 0, alpha(V, dt), 1 - 4 * beta(V, dt) - A(dt), B(dt), 0;
                       0, 0, 0, 0, A(dt), 1 - gamma(V, dt) - B(dt), phi(V, dt);
                       0, 0, 0, 0, 0, gamma(V, dt), 1 - phi(V, dt)];

  state_space.A = P_KV_1_3;
  state_space.b = zeros(7, 1);
  state_space.c = [0, 0, 0, 0, 0, 1, 0];
  state_space.g_k = g;
end
