function [P_O, current, time_vector, state] = simulate_pulse_protocol(model, dt, pulse_protocol, EvK, Ca_i, v_dep, x0)
  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  % Simulate pulse protocol
  % simulate pulse protocol for ion channels Kv1.3, Kv3.1, Kv3.4, Kv7.1, KCa1.1, KCa3.1, TASK-1 and ClC-2
  % input:
  %  * model: the model for the desired ion channel
  %  * dt: sampling time for the model
  %  * pulse_protocol: voltage clamp protocol specified as {voltage level, duration, voltage level, duration,....}
  %  * EvK: reversal potential for the specified ion
  %  * Ca_i: calcium concentration
  %  * v_dep: is the model voltage dependent? v_dep = 1 -> yes
  %  * x0: initial state: fractions of ion channels in each if the HMM-states
  %  (optional)
  % output:
  %  * P_O: the open probability of the ion channel
  %  * current: the modeled current: P_O * g *(Voltage-EvK)
  %  * time_vector: the time vector of the simulations
  %  * state: the fraction of ion channels in each state at the end of the
  %    simulations
  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  % length of simulation over all voltage steps specified
  total_length = sum([pulse_protocol{2:2:end}]);

  %% initial state:
  % if not given, start in closed state (1)
  if nargin < 8
    x0 = zeros(size(model.b));
    x0(1) = 1;
  end

  %% simulate model
  X(1, :) = x0;
  current = [];
  P_O = [];
  %% protocol steps
  for protocol_step = 1:2:length(pulse_protocol)
    sprintf(['start simulation with ', num2str(pulse_protocol{protocol_step}), ' V for ', num2str(pulse_protocol{protocol_step + 1}), ' s'])
    simulation_time_vector = 0:dt:pulse_protocol{protocol_step + 1};

    if protocol_step == 1
      time_vector = 0:dt:pulse_protocol{2};
    else
      time_vector = [time_vector, simulation_time_vector + time_vector(end) + dt];
    end

    % inital state of previous voltage level
    x0_1 = X(end, :);

    if v_dep == 1
      sim_model = ss(model.A(pulse_protocol{protocol_step}, dt), model.b, model.c, 0, dt);
    else
      sim_model = ss(model.A(dt), model.b, model.c, 0, dt);
    end

    %     if sum(sum(sim_model.A<0))>0 || sum(isinf(X(end,:))) > 0 || sum(sum(isnan(sim_model.A)))>0 ...
    %             || sum(sum(isinf(sim_model.A)))>0
    % %         sprintf('A Inf OR NaN or neg Eigenvalue')
    %         sim_model.A;
    %         eig(sim_model.A);
    %         P_O = Inf*ones(length(0:dt:total_length),1, 'linewidth', 1.5);
    %         current = P_O;
    %         time_vector = 0:dt:total_length-dt;
    %         return;
    %     end

    [Y, ~, X] = lsim(sim_model, zeros(size(simulation_time_vector)), simulation_time_vector, x0_1);
    current = [current; Y * model.g_k * (pulse_protocol{protocol_step} - EvK)];
    P_O = [P_O; Y];
    state = X(end, :);
  end

  % figure
  % plot(time_vector,current*10^9, 'linewidth',1.5);
  % % figure
  % % plot(time_vector,P_O, 'linewidth',1.5);
  % % ylabel('Expected open probability', 'FontSize',16)
  % ylabel('Expected single channel current [nA]', 'FontSize',16)
  % xlabel('Time [s]', 'FontSize',16)
  % axis([0.1 1.1 0 1])
