%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Simulate pulse protocol TRPV3
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [P_O, current, time_vector] = simulate_pulse_protocol_TRPV(dt, pulse_protocol, EvCa)
  num_of_steps = length(pulse_protocol) / 2;
  total_length = sum([pulse_protocol{2:2:end}]);

  current = [];
  %% protocol steps
  for protocol_step = 1:2:length(pulse_protocol)
    %     sprintf(['start simulation with ',num2str(pulse_protocol{protocol_step}),' V for ',num2str(pulse_protocol{protocol_step+1}),' s'])
    simulation_time_vector = 0:dt:pulse_protocol{protocol_step + 1};

    if protocol_step == 1
      time_vector = 0:dt:pulse_protocol{2};
    else
      time_vector = [time_vector, simulation_time_vector + time_vector(end) + dt];
    end

    % for resting membrane potential
    g_k = 48e-12;

    %     if pulse_protocol{protocol_step} > 40e-3
    %         g_k = 48e-12;
    %     else if pulse_protocol{protocol_step} <= 40e-3
    %         g_k = 48e-12; %197e-12
    %         end
    %     end
    current = [current, g_k * (pulse_protocol{protocol_step} - EvCa) * ones(size(simulation_time_vector))];
  end

  P_O = ones(size(time_vector)); % always open

  % figure
  % plot(time_vector,current*10^9, 'linewidth',1.5);
  % figure
  % plot(time_vector,P_O, 'linewidth',1.5);
  % ylabel('Expected single channel current [nA]', 'FontSize',16)
  % xlabel('Time [s]', 'FontSize',16)
