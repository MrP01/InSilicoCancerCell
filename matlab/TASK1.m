%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM TASK-1
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space] = TASK1(F, R, T)
  %% parameter definitions
  %rate constants given in s^-1
  a0 = 13.3;
  za = -0.106;
  b0 = 17.6;
  zb = 0.105;
  x0 = 108;
  zx = -0.095;
  d0 = 9.7;
  zd = 0.307;
  %single channel conductance
  g = 16 * 10^-12;

  % transition rates
  a_rate = @(V)a0 * exp((za * V * F) / (R * T));
  b_rate = @(V)b0 * exp((-zb * V * F) / (R * T));
  x_rate = @(V)x0 * exp((zx * V * F) / (R * T));
  d_rate = @(V)d0 * exp((-zd * V * F) / (R * T));

  %  transition probabilities = rate constants * dt
  a_prob = @(V, dt)a_rate(V) * dt;
  b_prob = @(V, dt)b_rate(V) * dt;
  x_prob = @(V, dt)x_rate(V) * dt;
  d_prob = @(V, dt)d_rate(V) * dt;

  %% define state space model
  P_TASK1 = @(V, dt) [1 - a_prob(V, dt), b_prob(V, dt), 0;
                      a_prob(V, dt), 1 - b_prob(V, dt) - x_prob(V, dt), d_prob(V, dt);
                      0, x_prob(V, dt), 1 - d_prob(V, dt)];

  state_space.A = P_TASK1;
  state_space.b = zeros(3, 1);
  state_space.c = [0, 0, 1];
  state_space.d = 0;
  state_space.g_k = g;
