function [time_vector_measure, total_curr_meas, total_curr_meas_all] = load_patch_clamp_data(choose_data_flag)
  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  % Load pre-processed patch-clamp data of cells in G0 phase and G1 phase
  % mG0 corresponds to the whole-cell current patch-clamp recordings of cells in G0 phase
  % mG1 corresponds to the whole-cell current patch-clamp recordings of cells in G1 phase
  % input:
  % * choose_data_flag: a number between 1 and 6:
  %  1: Activation G0 phase
  %  2: Activation G1 phase
  %  3: Deactivation G0 phase
  %  4: Deactivation G1 phase
  %  5: Ramp G0 phase
  %  6: Ramp G1 phase
  % output:
  % * total_curr_meas: measured current at highest input voltage level averaged
  %    over all cells
  % * total_curr_meas_all: measured current at all input voltage levels averaged
  %    over all cells
  % * time_vector_measure: time vector of the measurements
  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

  %% Activation G0 phase
  switch choose_data_flag
    case 1
      load('../data/provision/patch_clamp_data_activation')
      num_of_voltage_levels = 9;

      mean_data = zeros(length(mG0_1), num_of_voltage_levels);

      for sweep_number = 1:num_of_voltage_levels
        curr_data = [mG0_1(:, sweep_number), mG0_2(:, sweep_number), mG0_3(:, sweep_number), ...
                    mG0_4(:, sweep_number), mG0_5(:, sweep_number), mG0_6(:, sweep_number), mG0_7(:, sweep_number), ...
                    mG0_8(:, sweep_number), mG0_9(:, sweep_number), mG0_10(:, sweep_number), mG0_11(:, sweep_number)];
        mean_data(:, sweep_number) = mean(curr_data * 1e9, 2);
      end

      figure
      plot(time_vector_measure, mean_data)
      axis([-0.01 1.01 -3 * 10^-1 5 * 10^-1])
      ylabel('Whole-cell current G0 phase [nA]')
      xlabel('Time [s]')
      grid on

      total_curr_meas = mean_data(:, 9);
      total_curr_meas_all = mean_data;

      %% Activation G1 phase

    case 2
      load('../data/provision/patch_clamp_data_activation')

      num_of_voltage_levels = 9;
      mean_data = zeros(length(mG1_1), num_of_voltage_levels);

      for sweep_number = 1:num_of_voltage_levels
        curr_data = [mG1_1(:, sweep_number), mG1_2(:, sweep_number), mG1_3(:, sweep_number), ...
                    mG1_4(:, sweep_number), mG1_5(:, sweep_number)];
        mean_data(:, sweep_number) = mean(curr_data * 1e9, 2);
      end

      figure
      plot(time_vector_measure, mean_data)

      axis([-0.01 1.01 -3 * 10^-1 4 * 10^-1])
      ylabel('Whole-cell current G1 phase [nA]')
      xlabel('Time [s]')
      grid on

      total_curr_meas = mean_data(:, 9);
      total_curr_meas_all = mean_data;

      %% Deactivation G0 phase

    case 3
      load('../data/provision/patch_clamp_data_deactivation')

      num_of_voltage_levels = 7;
      mean_data = zeros(length(mG0_2_deact), num_of_voltage_levels);

      for sweep_number = 1:num_of_voltage_levels
        curr_data = [mG0_2_deact(:, sweep_number), mG0_3_deact(:, sweep_number), mG0_7_deact(:, sweep_number), ...
                    mG0_8_deact(:, sweep_number), mG0_10_deact(:, sweep_number)];
        mean_data(:, sweep_number) = mean(curr_data, 2);
      end

      figure
      plot(time_vector_measure, mean_data)
      axis([-0.01 11 -5 * 10^-1 5 * 10^-1])
      ylabel('Whole-cell current deactivation G0 phase [nA]')
      xlabel('Time [s]')
      grid on

      total_curr_meas = mean_data(:, 7);
      total_curr_meas_all = mean_data;

      %% Deactivation G1 phase

    case 4
      load('../data/provision/patch_clamp_data_deactivation')

      num_of_voltage_levels = 7;
      mean_data = zeros(length(mG1_1_deact), num_of_voltage_levels);

      for sweep_number = 1:num_of_voltage_levels
        curr_data = [mG1_1_deact(:, sweep_number), mG1_2_deact(:, sweep_number), ...
                    mG1_3_deact(:, sweep_number)];
        mean_data(:, sweep_number) = mean(curr_data, 2);
      end

      figure
      plot(time_vector_measure, mean_data)
      axis([-0.01 11 -5 * 10^-1 5 * 10^-1])
      ylabel('Whole-cell current deactivation G1 phase [nA]')
      xlabel('Time [s]')
      grid on

      total_curr_meas = mean_data(:, 7);
      total_curr_meas_all = mean_data;

      %% Ramp G0 phase

    case 5
      load('../data/provision/patch_clamp_data_ramp')

      data_mat_ramp20(:, 1:9) = [mG0_1_ramp20(:, 1), mG0_2_ramp20(:, 1), mG0_3_ramp20(:, 1), mG0_4_ramp20(:, 1), mG0_6_ramp20(:, 1), mG0_7_ramp20(:, 1), mG0_8_ramp20(:, 1), mG0_10_ramp20(:, 1), mG0_11_ramp20(:, 1)]; %,...
      mean_ramp20 = mean(data_mat_ramp20 * 1e9, 2);

      figure
      hold on
      plot(time_vector_measure, mean_ramp20 * 1e-9)
      axis([-0.01 20.5 -5 * 10^-1 8 * 10^-1])
      ylabel('Whole-cell current ramp G0 phase [nA]')
      xlabel('Time [s]')
      grid on

      total_curr_meas = mean_ramp20;
      total_curr_meas_all = [mean_ramp20];

      %% Ramp G1 phase

    case 6

      load('../data/provision/patch_clamp_data_ramp')

      data_mat_ramp20(:, 1:4) = [mG1_1_ramp20(:, 1), mG1_3_ramp20(:, 1), mG1_4_ramp20(:, 1), mG1_5_ramp20(:, 1)]; %,...
      mean_ramp20 = mean(data_mat_ramp20 * 1e9, 2);

      figure
      hold on
      plot(time_vector_measure, mean_ramp20 * 1e-9)
      axis([-0.01 20.5 -5 * 10^-1 5 * 10^-1])
      ylabel('Whole-cell current ramp G1 phase [nA]')
      xlabel('Time [s]')
      grid on

      total_curr_meas = mean_ramp20;
      total_curr_meas_all = [mean_ramp20];
    otherwise
      error('Please provide a number between 1 and 6.')
  end

end
