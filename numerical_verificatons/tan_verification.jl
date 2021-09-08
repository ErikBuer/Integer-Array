using Plots
using FFTW

tan_taylor(x) = x+( x^3/3 )+( x^5*2/15.0 )+( x^7*17/315.0 )+( x^9*62/2835.0 )+( x^11*1382/155925.0 )+( x^13*21844/6081075.0 )+( x^15*929569/638512875.0 );

x = LinRange(-π/2+0.1, π/2-0.1, 1001);

tanx_taylor = tan_taylor.(x);
tanx_native = tan.(x);

delta = tanx_native-tanx_taylor

# Time domain comparison.
plot( 	x, tanx_native,
	 	label = "Julia native tan",
        size = (1024, 720),)
plot!( 	x, tanx_taylor,
	 	label = "8-term Taylor series",)

savefig("figures/tan/time_domain")
