fn xrd_peak_fit() {
    pearson_vii(30.0, 30.2, 10.0, 20.0, 2.0, 1.0);
    


    println!("\n Y is");
}

fn pearson_vii(x: f64, x_c: f64, width: f64, area: f64, y0_a: f64, y0_b: f64){
    let k_alpha2: f64 = 0.179285;  //Co wavelengths
    let k_alpha1: f64 = 0.1788965;
    let mu: f64 = 1.5;            //shape factor (1.5 is used in JADE 5)
    
    let y = x*y0_a+y0_b;

    println!("Params {}\n {}\n {}\n {}\n {}\n", x, width, area, mu, k_alpha1);
    //for i = 1:length(x_c)
    //    x_c_2(i) = 360.*asin(Kalpha2./(2.*(Kalpha1./(2.*sin(%pi*x_c(i)./(360))))))./(%pi)   //Kalpha 2 peak position
    //    Area_2(i) = Area(i)/2                                                               //alpha 2 area
    //    width_2(i) = width(i)                                                               //alpha 2 width
    //    
    //    y = y + Area(i)*((2*gamma(mu)*sqrt(2^(1/mu)-1))/(sqrt(%pi)*width(i)*gamma(mu-1/2)))*(1+4*((2^(1/mu)-1)/width(i)^2).*(x-x_c(i)).^2).^(-mu) + Area_2(i)*((2*gamma(mu)*sqrt(2^(1/mu)-1))/(sqrt(%pi)*width_2(i)*gamma(mu-1/2)))*(1+4*((2^(1/mu)-1)/width_2(i)^2).*(x-x_c_2(i)).^2).^(-mu)
    //end
    
}
