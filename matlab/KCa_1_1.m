%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% HMM KCa1.1
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

function [state_space]= KCa_1_1_alternative(Ca_i)
%% parameter definitions
% rate constants
on_rate = 1000; %µMol^-1s^1
Ca_i = Ca_i*1e6*on_rate;
c0=1.8225;
c1= 1.215;
c2=0.855;
c3=0.49;
c4=0.11;
d=200;
b=36;
a4=0.396;
ko_u = 1.5*on_rate;
kc_u = 13.5*on_rate;
a0=0.001;
a1=0.006;
a2=0.038;
a3=0.196;

%single channel conductance
g_BK = 250*10^-12;

% transition rates 
a_rate=@(V)c0*exp(-V*1e3/d);
b_rate=@(V)a0*exp(V*1e3/b);
c_rate=@(V)c1*exp(-V*1e3/d);
d_rate=@(V)a1*exp(V*1e3/b);
e_rate=@(V)c2*exp(-V*1e3/d);
f_rate=@(V)a2*exp(V*1e3/b);
g_rate=@(V)c3*exp(-V*1e3/d);
h_rate=@(V)a3*exp(V*1e3/b);
i_rate=@(V)c4*exp(-V*1e3/d);
j_rate=@(V)a4*exp(V*1e3/b);

% transition probability alpha = rate constants * ms
a = @(V,dt)a_rate(V)*dt*1e3;
b = @(V,dt)b_rate(V)*dt*1e3;
c = @(V,dt)c_rate(V)*dt*1e3;
d = @(V,dt)d_rate(V)*dt*1e3;
e = @(V,dt)e_rate(V)*dt*1e3;
f = @(V,dt)f_rate(V)*dt*1e3;
g = @(V,dt)g_rate(V)*dt*1e3;
h = @(V,dt)h_rate(V)*dt*1e3;
i = @(V,dt)i_rate(V)*dt*1e3;
j = @(V,dt)j_rate(V)*dt*1e3;

ko = @(dt) ko_u*dt; 
kc = @(dt) kc_u*dt;
Ca_i_dt = @(dt) Ca_i*dt;



%% define state space model:
P_KCa_1_1 = @(V,dt) [1-4*Ca_i_dt(dt)-b(V,dt),kc(dt),0,0,0,0,0,0,0,a(V,dt);
                         4*Ca_i_dt(dt), 1-kc(dt)-3*Ca_i_dt(dt)-d(V,dt), 2*kc(dt),0, 0,0,0, 0,c(V,dt), 0;
                         0, 3*Ca_i_dt(dt), 1-2*kc(dt)-2*Ca_i_dt(dt)-f(V,dt), 3*kc(dt), 0,0,0, e(V,dt), 0, 0;
                         0,0,2*Ca_i_dt(dt), 1-3*kc(dt)-Ca_i_dt(dt)-h(V,dt), 4*kc(dt), 0, g(V,dt), 0,0,0;
                         0,0,0,Ca_i_dt(dt), 1-4*kc(dt)-j(V,dt), i(V,dt), 0,0,0,0;
                         0,0,0, 0, j(V,dt), 1-i(V,dt)-4*ko(dt), Ca_i_dt(dt), 0,0,0;
                         0,0,0,h(V,dt),0,4*ko(dt), 1-g(V,dt)-Ca_i_dt(dt)-3*ko(dt), 2*Ca_i_dt(dt), 0,0;
                         0,0,f(V,dt), 0, 0, 0, 3*ko(dt), 1-2*Ca_i_dt(dt)-e(V,dt)-2*ko(dt), 3*Ca_i_dt(dt), 0;
                         0, d(V,dt), 0, 0, 0, 0, 0, 2*ko(dt), 1-c(V,dt)-3*Ca_i_dt(dt)-ko(dt), 4*Ca_i_dt(dt);
                         b(V,dt), 0,0,0,0,0,0,0,ko(dt), 1-4*Ca_i_dt(dt)-a(V,dt)];
                         
state_space.A = P_KCa_1_1;
state_space.b = zeros(10,1);
state_space.c = [0,0,0,0,0,1,1,1,1,1];
state_space.g_k = g_BK;

