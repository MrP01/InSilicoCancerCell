%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM TRPV3
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space] = TRPV_3()
  %% parameter definitions
  %single channel conductance
  g = 58e-12;

  %% define state space model:
  P_TRPV_3 = @(V, dt) 1;

  % state_space.A = P_TRPV_3;
  % state_space.b = zeros(3,1);
  % state_space.c = [0,1,0];
  % state_space.g_k@(V,dt) = g;

  % plot(time_vector,current_TRPV3);
  % hold on
  % ylabel('Single channel current [A]')
  % xlabel('Time [s]')
