%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% A549 whole-cell model main
% * Load patch-clamp data: read in an visualize experimental data
% * Optimization: optimization of ion channel numbers for activation data of cells in G0
%   and G1 phase
% * Model evaluation: simulation of the whole-cell current for different
%   voltage-clamp protocols and calculation of the root mean square error
%   values (RMS) between simulated and measured data
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% Load patch-clamp data
data_file = 1; % choose data for visualization
% #1    activation_data_G0
% #2    activation_data_G1
% #3    deactivation_data_G0
% #4    deactivation_data_G1
% #5    ramp_data_G0
% #6    ramp_data_G1

[time_vector_measure, total_curr_meas, total_curr_meas_all] = load_patch_clamp_data(data_file);

%% Optimization

data = 2; % choose data for optimization#1 G0 or#2 G1
[time_vector_measure, total_curr_meas, total_curr_meas_all] = load_patch_clamp_data(data);

% define parameters for optimization
EvK = -77.4e-3; % reversal potential K
EvCa = 95.6e-3; % reversal potential Ca
EvCl = -7.9e-3; % reversal potential Cl
Ca_i = 0.0647 * 10^-6; % initial calcium concentration

% constants and temperature
F = 96485.3329; %As/mol
R = 8.3144598; %kgm^2/s^2molK
T = 273; %K

% calcium modeling
N_CRAC1 = 200; % number of CRAC channels
e_trans = 21.8976; % transfer coefficient �Mol/pAs
e_diff = 3; % diffusion coefficient 1/s

% define constrains for single ion channels
% lower bounds
const_low(1) = 0.2 % of initial current %Kv1.3
const_low(2) = 5; %Kv3.1
const_low(3) = 5; %Kv3.4
const_low(4) = 5; %Kv7.1
const_low(5) = 5; %KCa1.1
const_low(6) = 0.5 % of initial current %KCa3.1
const_low(7) = 10; %TASK1
const_low(8) = 10; %TRPV3
const_low(9) = 10; %TRPC6
const_low(10) = 10; %CLC-2
% upper bounds
const_high(1) = 0.8 % of initial current %Kv1.3
const_high(2) = 150; %Kv3.1
const_high(3) = 100; %Kv3.4
const_high(4) = 1350; %Kv7.1
const_high(5) = 50; %KCa1.1
const_high(6) = 0.9 % of initial current %KCa3.1
const_high(7) = 100; %TASK1
const_high(8) = 20; %TRPV3
const_high(9) = 20; %TRPC6
const_high(10) = 100; %CLC-2

[NOCs_global, Ca_steady_state] = optimization(time_vector_measure, total_curr_meas, total_curr_meas_all, EvK, EvCa, EvCl, Ca_i, F, R, T, N_CRAC1, e_trans, e_diff, const_low, const_high)

%% Model evaluation
data = 2; % choose data for evaluation#1 -#6
[time_vector_measure, total_curr_meas, total_curr_meas_all] = load_patch_clamp_data(data);

% define parameters for model evaluation
EvK = -77.4e-3; % reversal potential K
EvCa = 95.6e-3; % reversal potential Ca
EvCl = -7.9e-3; % reversal potential Cl
Ca_steady_state = 4.6847e-6; % steady state calcium concentration

% constants and temperature
F = 96485.3329; % Faraday constant [As/mol]
R = 8.3144598; % gas constant [kgm^2/s^2molK]
T = 273; % temperature [K]

% define number of ion channels for model evaluation
N_CRAC1 = 200; % number of CRAC channels
NOCs = 1; % choose ion channel numbers#0 NOCs of estimation for G0 phase
%                            #1 NOCs of estimation for G1 phase

%% Model evaluation using predefined values
[NOCs_evaluation, mean_RMS] = model_evaluation(time_vector_measure, total_curr_meas, total_curr_meas_all, EvK, EvCa, EvCl, ...
  Ca_steady_state, F, R, T, N_CRAC1, data, NOCs)

%% Model evaluation using NOCs from optimization
[NOCs_evaluation, mean_RMS] = model_evaluation(time_vector_measure, total_curr_meas, total_curr_meas_all, EvK, EvCa, EvCl, Ca_steady_state, F, R, T, N_CRAC1, data, NOCs, NOCs_global);

%% Model evaluation for user defined numbers of ion channels
random_NOCs = round(100 * rand(10, 1))
[NOCs_evaluation, mean_RMS] = model_evaluation(time_vector_measure, total_curr_meas, total_curr_meas_all, EvK, EvCa, EvCl, ...
  Ca_steady_state, F, R, T, N_CRAC1, data, NOCs, random_NOCs)
