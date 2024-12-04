use rest_dftd3::prelude::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_d3bj() {
        // structure definition
        #[rustfmt::skip]
        let coords = [
            [-0.358732711996, -1.219622503921,  0.131917549659],
            [ 2.119585463724, -0.669086968776,  0.829772153711],
            [ 2.809117479597,  1.818998649482,  1.345687626574],
            [ 1.038082099274,  3.759060826791,  1.167603107711],
            [-1.431707789689,  3.178259749841,  0.466491948077],
            [-2.150868246993,  0.7002414472  , -0.054012766253],
            [-3.85213636554 ,  5.802328175966,  0.217631119828],
            [ 3.501540139604, -2.166447600808,  0.971544490939],
            [ 1.559116125892,  5.692189547927,  1.567311393773],
            [ 4.728585655927,  2.260931747079,  1.890325797025],
            [-4.076404165723,  0.286497902539, -0.591827207478],
            [-0.90861903551 , -3.146363064474, -0.270076664397],
            [-7.856429540441, 10.827345280117, -1.660617862724],
            [-9.115644677165,  9.063295559651, -2.954460822835],
            [-5.344785190358, 11.282785671401, -3.951775457912],
            [-4.096127437596, 12.703412959362, -3.153238235825],
            [-4.280144282975,  9.55225545373 , -4.233361627772],
            [-6.082130593652, 11.976040508211, -5.735483303832]
        ];
        let coords = coords.iter().flatten().map(|&x| x).collect::<Vec<f64>>(); // coordinates needs to be flatten
        let natoms = 18;
        let charges = [6, 6, 6, 6, 6, 6, 35, 1, 1, 1, 1, 1, 16, 1, 6, 1, 1, 1];
        let latice = [0.0; 9];
        let periodic = [false; 3];
        let structure = DFTD3Structure::new(natoms, &charges, &coords, &latice, &periodic);
        let model = DFTD3Model::new(&structure);

        // PW6B95, d3bj
        let param = DFTD3Param::load_rational_damping("PW6B95", false);
        let disp_result = get_dispersion(&structure, &model, &param);
        assert!((disp_result.0 - -0.01009386).abs() < 1e-7);

        // PW6B95, d3zero
        let param = DFTD3Param::load_zero_damping("PW6B95", false);
        let disp_result = get_dispersion(&structure, &model, &param);
        assert!((disp_result.0 - -0.00574098).abs() < 1e-7);

        // PW6B95, d3zero, atm
        let param = DFTD3Param::load_zero_damping("PW6B95", true);
        let disp_result = get_dispersion(&structure, &model, &param);
        assert!((disp_result.0 - -0.00574289).abs() < 1e-7);

        /*
           Above example corresponds to the following python example:

           >>> from pyscf import gto
           >>> import dftd3.pyscf as disp
           >>> mol = gto.M(
           ...     atom='''
           ...          C   -0.189833176  -0.645396435   0.069807761
           ...          C    1.121636324  -0.354065576   0.439096514
           ...          C    1.486520953   0.962572632   0.712107225
           ...          C    0.549329390   1.989209324   0.617868956
           ...          C   -0.757627135   1.681862630   0.246856908
           ...          C   -1.138190460   0.370551816  -0.028582325
           ...          Br  -2.038462778   3.070459841   0.115165429
           ...          H    1.852935245  -1.146434699   0.514119204
           ...          H    0.825048723   3.012176989   0.829385472
           ...          H    2.502259769   1.196433556   1.000317333
           ...          H   -2.157140187   0.151608161  -0.313181471
           ...          H   -0.480820487  -1.664983631  -0.142918416
           ...          S   -4.157443472   5.729584377  -0.878761129
           ...          H   -4.823791426   4.796089466  -1.563433338
           ...          C   -2.828338520   5.970593053  -2.091189515
           ...          H   -2.167577293   6.722356639  -1.668621815
           ...          H   -2.264954814   5.054835899  -2.240198499
           ...          H   -3.218524904   6.337447714  -3.035087058
           ...          '''
           ... )
           >>> d3 = disp.DFTD3Dispersion(mol, xc="PW6B95", version="d3bj")
           >>> d3.kernel()[0]
           array(-0.01009386)
           >>> d3.version = "d3zero"  # Change to zero damping
           >>> d3.kernel()[0]
           array(-0.00574098)
           >>> d3.atm = True  # Activate three-body dispersion
           >>> d3.kernel()[0]
           array(-0.00574289)
        */
    }
}
