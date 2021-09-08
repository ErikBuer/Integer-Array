using Plots
using FFTW

"""
Calculate cos(x) with a five-term Taylor series, using only the quarter-waves closest to the origin for increased accuracy.
"""
function fast_cos( x::Float64 ) 
        if x < -π/2
            x = -π/2 + abs(x+π/2);
            return -(1-(x^2/2 )+( x^4/24.0 )-( x^6/720.0 )+( x^8/40320.0 ));
        elseif π/2 < x
            x = π/2 - abs(x-π/2);
            return -(1-(x^2/2 )+( x^4/24.0 )-( x^6/720.0 )+( x^8/40320.0 ));
        else
            return 1-(x^2/2 )+( x^4/24.0 )-( x^6/720.0 )+( x^8/40320.0 );
            
        end
end

x = LinRange(-π, π, 1000);
cos_taylor(x) = 1-(x^2/2 )+( x^4/24.0 )-( x^6/720.0 )+( x^8/40320.0 );

cosx_taylor = cos_taylor.(x);
cosx_native = cos.(x);

delta = sinx_native-sinx_taylor

# Time domain comparison.
plot( 	x, cosx_native,
	 	label = "Julia native sine",
        size = (1024, 720),)
plot!( 	x, cosx_taylor,
	 	label = "5-term Taylor series",)
plot!( 	x, delta*100,
	 	label = "Difference*100",)

savefig("figures/cos/cos_time_domain_sinx")


taylor_sine2 = fast_sine.(x);
delta2 = sinx_native-taylor_sine2

# Taylor vs first quarter taylor.
plot( 	x, delta*100,
	 	label = "Difference",
        size = (1024, 720),)
plot!( 	x, delta2*100,
	 	label = "Difference (first quarter)",)

savefig("figures/cos/time_domain_comparison2")


"""
Wrap sample to a -π =< sample < π range.
"""
function wrap_phase( sample::Float64 )
    while π <= sample
        sample = sample - 2π;
    end
    while sample < -π
        sample = sample + 2π;
    end
    return sample;
end

# Frequency domain analysis
x = LinRange(0, 1024π, 2^14);
x = wrap_phase.(x);
cosx_taylor = cos_taylor.(x);
cosx_native = cos.(x);

"""
Calculate normalized power spectrum for real signals.
"""
function frequecny_domain_analysis( signal_t )
    power_f = abs.(fft(signal_t)).^2;
    normalized_power = power_f/maximum(power_f);
    power_dB = 10*log10.(normalized_power);
    return power_dB[1:Int(size(power_dB)[1]/2)]
end


omega = LinRange(0, 1, Int(size(x)[1]/2));

# Frequency domain.
plot( 	omega, frequecny_domain_analysis(sinx_native),
	 	label = "Julia native sine",
        ylims = [-100, 0],
        size = (1024, 720),)
xlabel!("Frequency relative to Fs")
ylabel!("Normalized power [dBC]")

plot!( 	omega, frequecny_domain_analysis(sinx_taylor),
	 	label = "5-term Taylor series",)

savefig("figures/cos/frequency_domain_sinx")


# Frequency domain first quarter.
plot( 	omega, frequecny_domain_analysis(sinx_native),
	 	label = "Julia native sine",
        ylims = [-100, 0],
        size = (1024, 720),)
xlabel!("Frequency relative to Fs")
ylabel!("Normalized power [dBC]")

plot!( 	omega, frequecny_domain_analysis(fast_sine.(x)),
	 	label = "5-term Taylor, first quarter",)

savefig("figures/cos/taylor_sine_comparison")