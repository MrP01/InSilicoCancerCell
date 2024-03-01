function [NOCs_global, Ca_steady_state] = optimization(time_vector_measure, total_curr_meas, total_curr_meas_all, ...
    EvK, EvCa, EvCl, Ca_i, F, R, T, N_CRAC1, e_trans, e_diff, const_low, const_high)
  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  % optimization
  % input:
  %  * time_vector_measure: time vector of the measurements
  %  * total_curr_meas: measured current at highest input voltage level averaged
  %    over all cells
  %  * total_curr_meas_all: measured current at all input voltage levels averaged
  %    over all cells
  %  * EvK: reversal potential K
  %  * EvCa: reversal potential Ca
  %  * EvCl: reversal potential Cl
  %  * Ca_i: initial calcium concentration
  %  * F: Faraday constant [As/mol]
  %  * R: gas constant [kgm�/s�molK]
  %  * T: temperature [K]
  %  * N_CRAC1: number of CRAC channels
  %  * e_trans: transfer coefficient scaling the calcium influx to the
  %    calcium concentration
  %  * e_diff: calcium diffusion coefficient
  %  * const_low: lower bounds of ion channel numbers for optimization
  %  * const_high: upper bounds of ion channel numbers for optimization
  % output:
  %  * NOCs_global: estimated number of channels for each channel type
  %  * Ca_steady_state: steady state calcium concentration
  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

  %% sampling frequency and subsampling for simulation
  freq = 2000000;
  dt = 1 / freq;

  dt_measure = diff(time_vector_measure(1:2));
  sub_sampling = round(dt_measure / dt);

  %% activation pulse protocol for optimization
  V_hold = -100e-3;
  t_hold = 100e-3;
  V_initial = -80e-3;
  t_initial = 115.6e-3;
  V_test = [-0.040:0.01:0.040]' % voltagte levels activation protocol
  t_test = 800e-3;
  V_post = -80e-3;
  t_post = 84.4e-3;

  %% define pulse protocol for simulation of CRAC channels
  pulse_protocol_crac = {V_hold, 60000e-3}

  %% obtain calcium concentration in steady state
  CRAC1_model = CRAC1();
  [P_o_CRAC1_initial, current_CRAC1_initial, crac_time] = simulate_pulse_protocol(CRAC1_model, dt, pulse_protocol_crac, EvCa, Ca_i, 1);
  I_g_initial_pA = P_o_CRAC1_initial * N_CRAC1 * CRAC1_model.g_k * (pulse_protocol_crac{1} - EvCa);
  Ca_conc = e_trans / e_diff *- (I_g_initial_pA(end) * 10^12);
  plot(crac_time, I_g_initial_pA);
  Ca_i = Ca_conc(end) * 1e-6;
  Ca_steady_state = Ca_i;

  %% simulate all HMMs for activation protocol and create current matrix for optimization
  total_current_matrix = [];
  measured_current = [];

  % HMMs
  Kv_1_3_model = Kv_1_3();
  Kv_3_1_model = Kv_3_1();
  Kv_3_4_model = Kv_3_4(F, R, T);
  Kv_7_1_model = Kv_7_1(F, R, T);
  KCa_1_1_model = KCa_1_1(Ca_i);
  KCa_3_1_model = KCa_3_1(Ca_i);
  TASK1_model = TASK1(F, R, T);
  CLC_2_model = CLC_2(F, R, T);

  for step_index = 1:length(V_test)
    pulse_protocol = {V_hold, t_hold, V_initial, t_initial, V_test(step_index), t_test, V_post, t_post} % define pulse protocol for simulation

    [P_o_KV_1_3, current_KV_1_3, time_vector] = simulate_pulse_protocol(Kv_1_3_model, dt, pulse_protocol, EvK, 0, 1);
    [P_o_KV_3_1, current_KV_3_1] = simulate_pulse_protocol(Kv_3_1_model, dt, pulse_protocol, EvK, 0, 1);
    [P_o_KV_3_4, current_KV_3_4] = simulate_pulse_protocol(Kv_3_4_model, dt, pulse_protocol, EvK, 0, 1);
    [P_o_KV_7_1, current_KV_7_1] = simulate_pulse_protocol(Kv_7_1_model, dt, pulse_protocol, EvK, 0, 1);
    [P_o_KCa_1_1, current_KCa_1_1] = simulate_pulse_protocol(KCa_1_1_model, dt, pulse_protocol, EvK, Ca_i, 1);
    [P_o_KCa_3_1, current_KCa_3_1] = simulate_pulse_protocol(KCa_3_1_model, dt, pulse_protocol, EvK, Ca_i, 0);
    [P_o_TASK1, current_TASK1] = simulate_pulse_protocol(TASK1_model, dt, pulse_protocol, EvK, 0, 1);
    [P_o_CRAC1, current_CRAC1] = simulate_pulse_protocol(CRAC1_model, dt, pulse_protocol, EvCa, Ca_i, 1);
    [P_o_CLC_2, current_CLC_2] = simulate_pulse_protocol(CLC_2_model, dt, pulse_protocol, EvCl, 0, 1);

    [P_O_TRPV3, current_TRPV3] = simulate_pulse_protocol_TRPV(dt, pulse_protocol, EvCa);
    [P_O_TRPC6, current_TRPC6] = simulate_pulse_protocol_TRPC(dt, pulse_protocol, EvCa);

    close all

    %activation
    data = total_curr_meas(:, end);
    start_time = 51.6e-3;
    start_model = find(time_vector == t_hold + start_time + dt)
    start_meas = find(time_vector_measure == start_time)
    stop_time = 850e-3;
    end_time_meas = find(time_vector_measure == stop_time)
    end_time_model = find(time_vector == stop_time + t_hold)

    %% current matrix
    current_matrix = [current_KV_1_3(start_model:sub_sampling:end_time_model)';
                      current_KV_3_1(start_model:sub_sampling:end_time_model)';
                      current_KV_3_4(start_model:sub_sampling:end_time_model)';
                      current_KV_7_1(start_model:sub_sampling:end_time_model)';
                      current_KCa_1_1(start_model:sub_sampling:end_time_model)';
                      current_KCa_3_1(start_model:sub_sampling:end_time_model)';
                      current_TASK1(start_model:sub_sampling:end_time_model)';
                      current_TRPV3(start_model:sub_sampling:end_time_model);
                      current_TRPC6(start_model:sub_sampling:end_time_model);
                      current_CLC_2(start_model:sub_sampling:end_time_model)'];

    %% define measurement vector and remove CRAC current:
    measured_current = [measured_current, total_curr_meas_all(start_meas:end_time_meas - 1, step_index)' * 10^-9 - ...
                          N_CRAC1 * current_CRAC1(start_model:sub_sampling:end_time_model - 1)'];

    optim_current_length = length(total_curr_meas_all(start_meas:end_time_meas - 1, step_index));

    total_current_matrix = [total_current_matrix, current_matrix(:, 1:optim_current_length)];

  end

  %% define constraints for optimization
  % initial current
  t_comp = 117.95e-3;
  find(time_vector_measure == t_comp)
  total_initial_current = total_curr_meas(find(time_vector_measure == t_comp)) * 1e-9;

  % lower bounds
  LB = 5 * ones(10, 1);
  start_100 = find(time_vector == t_hold + dt);
  subsampled_KV1_3 = current_KV_1_3(start_100:sub_sampling:end)';
  subsampled_KCa_1_3 = current_KCa_3_1(start_100:sub_sampling:end)';
  current_KCa_3_1_initial = subsampled_KCa_1_3(find(time_vector_measure == t_comp));
  current_KV1_3_initial = subsampled_KV1_3(find(time_vector_measure == t_comp));

  subsampled_KV1_3 = current_KV_1_3(start_100:sub_sampling:end)';
  subsampled_KCa_1_3 = current_KCa_3_1(start_100:sub_sampling:end)';

  % set constraints for single ion channels
  LB(1) = round(total_initial_current * const_low(1) / current_KV1_3_initial) %Kv1.3
  LB(2) = const_low(2); %Kv3.1
  LB(3) = const_low(3); %Kv3.4
  LB(4) = const_low(4); %Kv7.1
  LB(5) = const_low(5); %KCa1.1
  LB(6) = round(total_initial_current * const_low(6) / current_KCa_3_1_initial) %KCa3.1
  LB(7) = const_low(7); %TASK1
  LB(8) = const_low(8); %TRPV3
  LB(9) = const_low(9); %TRPC6
  LB(10) = const_low(10); %CLC-2

  % upper bounds
  UB(1) = round(total_initial_current * const_high(1) / current_KV1_3_initial); %Kv1.3
  UB(2) = const_high(2); %Kv3.1
  UB(3) = const_high(3); %Kv3.4
  UB(4) = const_high(4); %Kv7.1
  UB(5) = const_high(5); %KCa1.1
  UB(6) = round(total_initial_current * const_high(6) / current_KCa_3_1_initial); %KCa3.1
  UB(7) = const_high(7); %TASK1
  UB(8) = const_high(8); %TRPV3
  UB(9) = const_high(9); %TRPC6
  UB(10) = const_high(10); %CLC2

  %% GLOBAL OPTIMIZATION
  % particle swarm
  for i = 1:1:3
    function_handle = @(try_NOCs) sqrt((try_NOCs * total_current_matrix - measured_current) * (try_NOCs * total_current_matrix - measured_current)' / (measured_current * measured_current'))

    problem.objective = function_handle;
    problem.nvars = length(UB);
    problem.ub = UB;
    problem.lb = LB;
    problem.solver = 'particleswarm';
    problem.options = optimoptions('particleswarm', 'SwarmSize', 200, 'HybridFcn', @fmincon, 'MaxIterations', 500, 'FunctionTolerance', 1e-6); %,'PlotFcn', @pswplotbestf);
    [x, fval] = particleswarm(problem);
    NOCs_global(:, i) = round(x)'
  end

  %% plot global optimization result
  C_full = [current_KV_1_3';
            current_KV_3_1';
            current_KV_3_4';
            current_KV_7_1';
            current_KCa_1_1';
            current_KCa_3_1';
            current_TASK1';
            current_TRPV3;
            current_TRPC6;
            current_CLC_2'];
  figure
  hold on

  for i = 1
    plot(time_vector - t_hold, NOCs_global' * C_full * 10^9 + N_CRAC1 * current_CRAC1', 'Color', [0, 0.4470, 0.7410], 'linewidth', 1.5);
  end

  hold on
  plot(time_vector_measure, total_curr_meas_all(:, 9)', 'k--', 'linewidth', 1.5)
  axis([-0.1 1.1 -0.5 0.5])
  ylabel('Whole-cell current [nA]', 'Fontsize', 16)
  xlabel('Time [s]', 'Fontsize', 16)
  grid on
