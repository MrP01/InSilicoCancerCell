%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM CRACM1
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space]= CRAC1()
%% parameter definitions
%single channel conductance
g = 24*10^-15;


% transition rates
tau_o = @(V) 41*exp(V/110);
tau_c = @(V) 19*exp(V/48);

% transition probabilities
alpha = @(V,dt)1/tau_c(V)*dt*1e3;
beta  = @(V,dt)1/tau_o(V)*dt*1e3;

%% define state space model:
state_space.A = @(V,dt) P_CRAC1(V,dt,alpha,beta);
state_space.b = zeros(2,1);
state_space.c = [0,1];
state_space.g_k = g;
