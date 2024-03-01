%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM CLC-2
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space]= CLC_2(F,R,T)
%% parameter definitions
%single channel conductance
g = 2.8*10^-12;

% transition rates
a1 = @(V)4.1*exp((-0.57*V*F)/(R*T));
b1 = @(V)100.3*exp((0.18*V*F)/(R*T));
a2 = @(V)6.4*exp((-0.2*V*F)/(R*T));
b2 = @(V)10.6*exp((0.32*V*F)/(R*T));
l = @(V)1.7*exp((-0.3*V*F)/(R*T))+8.8*exp((0.14*V*F)/(R*T));
u = @(V)4.9*exp((0.18*V*F)/(R*T));

% transition probabilities
a1 = @(V,dt) a1(V)*dt;
b1 = @(V,dt) b1(V)*dt;
a2 = @(V,dt) a2(V)*dt;
b2 = @(V,dt) b2(V)*dt;
u = @(V,dt) u(V)*dt;
l = @(V,dt) l(V)*dt;

%% define state space model:
P_CLC_2 = @(V,dt) [1-2*a1(V,dt)-l(V,dt),u(V,dt),b1(V,dt),0,0,0,0,0,0,0,0,0;
                    l(V,dt),1-u(V,dt)-2*a1(V,dt),0,b1(V,dt),0,0,0,0,0,0,0,0;
                    2*a1(V,dt),0,1-b1(V,dt)-l(V,dt)-a2(V,dt)-a1(V,dt),u(V,dt),b2(V,dt),2*b1(V,dt),0,0,0,0,0,0;
                    0,2*a1(V,dt),l(V,dt),1-b1(V,dt)-u(V,dt)-a1(V,dt)-a2(V,dt),0,0,2*b1(V,dt),b2(V,dt),0,0,0,0;
                    0,0,a2(V,dt),0,1-b2(V,dt)-l(V,dt)-a1(V,dt),0,0,u(V,dt),b1(V,dt),0,0,0;
                    0,0,a1(V,dt),0,0,1-2*b1(V,dt)-l(V,dt)-2*a2(V,dt),u(V,dt),0,2*b2(V,dt),0,0,0;
                    0,0,0,a1(V,dt),0,l(V,dt),1-2*b1(V,dt)-u(V,dt)-2*a2(V,dt),0,0,0,0,0;
                    0,0,0,a2(V,dt),l(V,dt),0,0,1-b2(V,dt)-u(V,dt)-a1(V,dt),0,b1(V,dt),0,0;
                    0,0,0,0,a1(V,dt),2*a2(V,dt),0,0,1-b1(V,dt)-b2(V,dt)-l(V,dt)-a2(V,dt),u(V,dt),b2(V,dt),0;
                    0,0,0,0,0,0,2*a2(V,dt),a1(V,dt),l(V,dt),1-b2(V,dt)-b1(V,dt)-u(V,dt)-a2(V,dt),0,b2(V,dt);
                    0,0,0,0,0,0,0,0,a2(V,dt),0,1-b2(V,dt)-l(V,dt),u(V,dt);
                    0,0,0,0,0,0,0,0,0,a2(V,dt),l(V,dt),1-b2(V,dt)-u(V,dt)];
                
state_space.A = P_CLC_2;
state_space.b = zeros(12,1);
state_space.c = [0,0,0,1,0,0,1,1,0,1,0,1];
state_space.g_k = g;