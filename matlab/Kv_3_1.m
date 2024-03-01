%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM Kv3.1
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space, steady_state] = Kv_3_1()
  %% parameter definitions
  %rate constants
  a = 0.925973;
  b = 35.893170;
  c = 0.194083;
  d = 279.735292;
  alpha_rate = 40.558628;
  beta_rate = 389.270005;

  a = 1.027813;
  b = 35.984422;
  c = 0.212136;
  d = 187.126824;
  alpha_rate = 44.050331;
  beta_rate = 368.999741;

  %single channel conductance
  g = 40 * 10^-12;

  %% transition rates
  a_rate = @(V) a * exp(V * 1e3 / b);
  c_rate = @(V) c * exp(-V * 1e3 / d);

  % transition probability = rate constants * dt
  a_prob = @(V, dt) a_rate(V) * dt * 1e3;
  c_prob = @(V, dt)c_rate(V) * dt * 1e3;
  alpha = @(dt) alpha_rate * dt * 1e3;
  beta = @(dt) beta_rate * dt * 1e3;

  %% define state space model:
  P_KV_3_1 = @(V, dt) [1 - 4 * a_prob(V, dt), c_prob(V, dt), 0, 0, 0, 0;
                       4 * a_prob(V, dt), 1 - 3 * a_prob(V, dt) - c_prob(V, dt), 2 * c_prob(V, dt), 0, 0, 0;
                       0, 3 * a_prob(V, dt), 1 - 2 * a_prob(V, dt) - 2 * c_prob(V, dt), 3 * c_prob(V, dt), 0, 0;
                       0, 0, 2 * a_prob(V, dt), 1 - a_prob(V, dt) - 3 * c_prob(V, dt), 4 * c_prob(V, dt), 0;
                       0, 0, 0, a_prob(V, dt), 1 - 4 * c_prob(V, dt) - alpha(dt), beta(dt);
                       0, 0, 0, 0, alpha(dt), 1 - beta(dt)];

  state_space.A = P_KV_3_1;
  state_space.b = zeros(6, 1);
  state_space.c = [0, 0, 0, 0, 0, 1];
  state_space.g_k = g;
