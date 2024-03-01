%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM KCa3.1
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space]= KCa_3_1(Ca_i)
%% parameter definitions
% rate constants
%values 1:
k12=27;
k23=5425;
k21=34;
k35=34;
k32=190;
k53=20;

%single channel conductance
g = 11*10^-12;

% transition rates
k12_rate = @(Ca_i) (k12 * Ca_i *10^6);
k23_rate = @(Ca_i) (k23 * Ca_i *10^6);
k21_rate = k21;
k35_rate = k35;
k32_rate = k32;
k53_rate = k53;

% transition probability = rate constants * s
k12_prob = @(Ca_i,dt) k12_rate(Ca_i)*dt;
k23_prob = @(Ca_i,dt) k23_rate(Ca_i)*dt;
k21_prob = @(dt)k21_rate*dt;
k35_prob = @(dt)k35_rate*dt;
k32_prob = @(dt)k32_rate*dt;
k53_prob = @(dt)k53_rate*dt;

%% define state space model:
P_KCa_3_1 = @(Ca_i,dt)[1-k12_prob(Ca_i,dt), k21_prob(dt), 0, 0;
                       k12_prob(Ca_i,dt),   1 - k21_prob(dt) - k23_prob(Ca_i,dt), k32_prob(dt), 0 ;
                       0, k23_prob(Ca_i,dt), 1-k32_prob(dt)-k35_prob(dt), k53_prob(dt);
                       0, 0, k35_prob(dt), 1-k53_prob(dt)];

state_space.A = @(dt) P_KCa_3_1(Ca_i,dt);
state_space.b =zeros(4,1);
state_space.c = [0,0,0,1];
state_space.g_k = g;

