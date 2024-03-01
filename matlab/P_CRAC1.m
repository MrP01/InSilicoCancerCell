function t = P_CRAC1(V,dt,alpha,beta)
if V <= 0

t = [1-alpha(V,dt),beta(V,dt);
    alpha(V,dt),1-beta(V,dt)];

else
    t =  [1,1;0,0];
end