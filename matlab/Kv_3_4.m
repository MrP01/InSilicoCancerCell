%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM Kv3.4
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space] = Kv_3_4(F, R, T)
  %% parameter definitions
  %rate constants
  a = 3352;
  za = 0.06;
  b = 3230;
  zb = -0.8;
  eps = 434;
  zeps = 0.52;
  phi = 70;
  zphi = -0.37;
  k1 = 55;
  l1 = 0.8;
  %single channel conductance
  g = 14 * 10^-12;

  % transition rates
  a_rate = @(V)a * exp((za * V * F) / (R * T));
  b_rate = @(V)b * exp((zb * V * F) / (R * T)); %-
  eps_rate = @(V)eps * exp((zeps * V * F) / (R * T));
  phi_rate = @(V)phi * exp((zphi * V * F) / (R * T)); %-

  % transition probability = rate constants * ms
  a_prob = @(V, dt) a_rate(V) * dt;
  b_prob = @(V, dt) b_rate(V) * dt;
  eps_prob = @(V, dt) eps_rate(V) * dt;
  phi_prob = @(V, dt) phi_rate(V) * dt;
  k1_prob = @(dt) k1 * dt;
  l1_prob = @(dt) l1 * dt;

  %% define state space model:
  P_KV_3_4 = @(V, dt)[1 - 4 * a_prob(V, dt), b_prob(V, dt), 0, 0, 0, 0, 0;
                      4 * a_prob(V, dt), 1 - b_prob(V, dt) - 3 * a_prob(V, dt), 2 * b_prob(V, dt), 0, 0, 0, 0;
                      0, 3 * a_prob(V, dt), 1 - 2 * b_prob(V, dt) - 2 * a_prob(V, dt), 3 * b_prob(V, dt), 0, 0, 0;
                      0, 0, 2 * a_prob(V, dt), 1 - 3 * b_prob(V, dt) - a_prob(V, dt), 4 * b_prob(V, dt), 0, 0;
                      0, 0, 0, a_prob(V, dt), 1 - 4 * b_prob(V, dt) - eps_prob(V, dt), phi_prob(V, dt), 0;
                      0, 0, 0, 0, eps_prob(V, dt), 1 - phi_prob(V, dt) - k1_prob(dt), l1_prob(dt);
                      0, 0, 0, 0, 0, k1_prob(dt), 1 - l1_prob(dt)];
  state_space.A = P_KV_3_4;
  state_space.b = zeros(7, 1);
  state_space.c = [0, 0, 0, 0, 0, 1, 0];
  state_space.g_k = g;
end
