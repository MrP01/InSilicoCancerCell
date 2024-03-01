%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Simulate pulse protocol TRPC6
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [P_O, current, time_vector] = simulate_pulse_protocol_TRPC(dt, pulse_protocol, EvCa)
  num_of_steps = length(pulse_protocol) / 2;
  time_vector = 0:dt:pulse_protocol{2};

  % sprintf(['start simulation with ',num2str(pulse_protocol{1}),' V for ',num2str(pulse_protocol{2}),' s'])
  simulation_time_vector = time_vector;

  %single channel conductance
  g_k = 35e-12;
  current = g_k * (pulse_protocol{1} - EvCa) * ones(size(simulation_time_vector)); % P_o is unknown, so we set it to one
  size(current);

  for protocol_step = 3:2:num_of_steps * 2
    %     sprintf(['start simulation with ',num2str(pulse_protocol{protocol_step}),' V for ',num2str(pulse_protocol{protocol_step+1}),' s']);
    simulation_time_vector = 0:dt:pulse_protocol{protocol_step + 1};
    time_vector = [time_vector, [0:dt:pulse_protocol{protocol_step + 1}] + time_vector(end) + dt];
    current = [current, g_k * (pulse_protocol{protocol_step} - EvCa) * ones(size(simulation_time_vector))];
  end

  P_O = ones(size(time_vector));
  % figure
  % plot(time_vector,current*10^9, 'linewidth', 1.5);
  % ylabel('Expected single channel current [nA]', 'FontSize', 16)
  % % plot(time_vector,P_O, 'linewidth', 1.5);
  % % ylabel('Expected open probability', 'FontSize', 16)
  % xlabel('Time [s]', 'FontSize', 16)
  % % axis([0 1.2 0 1.2])
