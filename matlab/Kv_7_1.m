%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM Kv7.1
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space] = Kv_7_1(F, R, T)
  %% definition of constants
  % transition rates
  a_rate = @(V) 4.6 * exp((0.47 * V * F) / (R * T));
  b_rate = @(V) 33 * exp((-0.35 * V * F) / (R * T));
  a2_rate = @(V) 24 * exp((0.006 * V * F) / (R * T));
  b2_rate = @(V) 19 * exp((-0.007 * V * F) / (R * T));
  eps_rate = @(V) 4.6 * exp((0.8 * V * F) / (R * T));
  delta_rate = @(V) 1.4 * exp((-0.7 * V * F) / (R * T));
  lambda = 142;
  micro = 52; %!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
  %single channel conductance
  g = 3.2 * 10^-12;

  % transition probabilities alpha = rate constants * ms
  a_prob = @(V, dt) a_rate(V) * dt;
  b_prob = @(V, dt) b_rate(V) * dt;
  a2_prob = @(V, dt) a2_rate(V) * dt;
  b2_prob = @(V, dt) b2_rate(V) * dt;
  eps_prob = @(V, dt) eps_rate(V) * dt;
  delta_prob = @(V, dt) delta_rate(V) * dt;
  lambda_prob = @(dt)lambda * dt;
  micro_prob = @(dt)micro * dt;

  %% define state space model:
  System_matrix_7_1 = @(V, dt) [1 - a_prob(V, dt), b_prob(V, dt), 0, 0, 0;
                                a_prob(V, dt), 1 - a2_prob(V, dt) - b_prob(V, dt), b2_prob(V, dt), 0, 0;
                                0, a2_prob(V, dt), 1 - b2_prob(V, dt) - eps_prob(V, dt), delta_prob(V, dt), 0;
                                0, 0, eps_prob(V, dt), 1 - delta_prob(V, dt) - lambda_prob(dt), micro_prob(dt);
                                0, 0, 0, lambda_prob(dt), 1 - micro_prob(dt)];
  state_space.A = System_matrix_7_1;
  state_space.b = [0, 0, 0, 0, 0]';
  state_space.c = [0, 0, 1, 1, 0];
  state_space.d = 0;
  state_space.g_k = g;
