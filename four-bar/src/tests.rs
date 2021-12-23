#![doc(hidden)]

#[cfg(feature = "plotters")]
#[allow(unused_imports)]
#[test]
fn planar() {
    use crate::*;
    use indicatif::ProgressBar;
    use metaheuristics_nature::ObjFunc;
    use ron::{from_str, to_string};
    use std::f64::consts::TAU;
    use std::fs::{read_to_string, write};

    // let target = Mechanism::four_bar(FourBar {
    //     p0: (0., 0.),
    //     a: 0.,
    //     l0: 90.,
    //     l1: 35.,
    //     l2: 70.,
    //     l3: 70.,
    //     // l4: 40.,
    //     // g: 0.5052948926891512,
    //     // l4: 84.7387,
    //     // g: 0.279854818911,
    //     l4: 77.0875,
    //     g: 5.88785793416,
    //     /////
    //     // NKhan 1
    //     // l0: 2.9587,
    //     // l1: 1.,
    //     // l2: 3.4723,
    //     // l3: 3.5771,
    //     // l4: 3.3454,
    //     // g: 3.3771,
    //     // NKhan 2
    //     // l0: 3.,
    //     // l1: 1.,
    //     // l2: 3.,
    //     // l3: 2.5,
    //     // l4: 1.,
    //     // g: 5.,
    //     inv: false,
    // })
    // .four_bar_loop(TAU / 6., 360);
    // let target = YU1;
    // let target = HAND;
    // let target = OPEN_CURVE1;
    // let target = TRIANGLE2;
    let target = CRUNODE;
    // let target = LINE;
    let gen = 40;
    let pb = ProgressBar::new(gen);
    let pb_inner = pb.clone();
    let s = synthesis::synthesis(
        &target,
        200,
        move |ctx| {
            pb_inner.set_position(ctx.gen);
            ctx.gen == gen
        },
        |ctx| (ctx.gen, ctx.best_f),
    );
    pb.finish();
    plot::plot_history(s.report(), "history.svg");
    let ans = s.result();
    write("result.ron", to_string(&ans).unwrap()).unwrap();
    let path = Mechanism::four_bar(&ans).four_bar_loop(0., 360);
    plot::plot_curve(
        "Synthesis Test",
        &[("Target", &target), ("Optimized", &path)],
        "result.svg",
    );
}

pub const HAND: &[[f64; 2]] = &[
    [107.31911969201228, 81.59276839878613],
    [107.2463224148719, 82.21336703713774],
    [107.09767748992344, 82.8096978257258],
    [106.87350662227556, 83.37712932129989],
    [106.57507284914166, 83.91136896339214],
    [106.20456015538429, 84.40854784369684],
    [105.76502948370728, 84.86529396169908],
    [105.26035235410475, 85.27879204999705],
    [104.69512402388091, 85.64682849039397],
    [104.07455877255757, 85.96782032041233],
    [103.40437046207884, 86.24082783272775],
    [102.69064198180553, 86.4655507798514],
    [101.93968752370878, 86.64230869597502],
    [101.15791183425222, 86.7720063206304],
    [100.35167064898712, 86.8560855393411],
    [99.52713643236567, 86.8964656311437],
    [98.69017332242603, 86.89547392032281],
    [97.84622482663502, 86.85576916111027],
    [97.00021734483698, 86.7802601334607],
    [96.15648202569761, 86.67202199234208],
    [95.31869681551282, 86.53421289233917],
    [94.48984985671751, 86.36999330679299],
    [93.67222466359516, 86.18245028203907],
    [92.8674067710462, 85.9745286209323],
    [92.07631084508772, 85.74897068629815],
    [91.2992265860777, 85.50826616650141],
    [90.5358811703725, 85.2546127654966],
    [89.78551548312964, 84.9898883827541],
    [89.0469710104255, 84.71563494877088],
    [88.31878399465106, 84.43305369354597],
    [87.59928332046609, 84.14301126164631],
    [86.88668859173009, 83.84605576021451],
    [86.17920498016159, 83.54244154565382],
    [85.47511166665717, 83.2321613289109],
    [84.77284104447227, 82.91498401411799],
    [84.0710462942516, 82.59049658427077],
    [83.36865545539672, 82.25814831154912],
    [82.66491068529979, 81.91729559732507],
    [81.95939199482362, 81.56724583403634],
    [81.25202535172939, 81.20729882203862],
    [80.54307563049495, 80.83678446158672],
    [79.83312543528132, 80.45509566410531],
    [79.12304131287203, 80.06171567771193],
    [78.41392928717721, 79.65623928875132],
    [77.70708197268509, 79.23838763291982],
    [77.00391975123452, 78.80801661566994],
    [76.305928618995, 78.36511919192738],
    [75.61459732717438, 77.90982198068507],
    [74.9313563535751, 77.44237688307152],
    [74.25752105953276, 76.96314852693622],
    [73.59424111851, 76.47259847254509],
    [72.94245796230254, 75.9712671802288],
    [72.30287159458089, 75.45975476128268],
    [71.67591768723906, 74.93870150946367],
    [71.0617554216584, 74.4087691452031],
    [70.46026608361711, 73.87062360284901],
    [69.87106198569316, 73.32492005888345],
    [69.29350489177783, 72.77229074316723],
    [68.72673276988193, 72.2133359035938],
    [68.16969341431452, 71.64861811518323],
    [67.62118326604734, 71.07865994574458],
    [67.07988962676134, 70.50394481957197],
    [66.5444344103151, 69.92492076538834],
    [66.01341760422352, 69.34200660116508],
    [65.4854587188504, 68.75560000163951],
    [64.95923467592608, 68.16608681812343],
    [64.43351282052636, 67.57385097690097],
    [63.90717801941475, 66.97928427301918],
    [63.37925311967692, 66.38279539996236],
    [62.84891236995029, 65.78481761053723],
    [62.31548773708055, 65.18581448695727],
    [61.77846836894092, 64.58628340416703],
    [61.23749374570035, 63.98675639455843],
    [60.69234131491437, 63.3877982584224],
    [60.14290961045745, 62.79000190639724],
    [59.5891980040459, 62.19398106135909],
    [59.03128432621916, 61.60036058137255],
    [58.469301619383174, 61.009764786628786],
    [57.90341525001237, 60.42280427657185],
    [57.3338015142978, 59.840061804340614],
    [56.76062872790151, 59.262077830967755],
    [56.18404160469575, 58.6893364093502],
    [55.60414951182473, 58.122252046922135],
    [55.021018950738004, 57.56115816653816],
    [54.43467036824567, 57.006297728795374],
    [53.8450791604491, 56.45781649849144],
    [53.2521805073781, 55.915759336686556],
    [52.655877478039564, 55.38006978227936],
    [52.056051683516046, 54.85059305809291],
    [51.45257563696147, 54.327082501563424],
    [50.84532590877475, 53.809209284762],
    [50.23419614539209, 53.29657515813417],
    [49.61910905097875, 52.78872783219855],
    [49.00002651029305, 52.285178506215026],
    [48.376957153262886, 51.78542096655504],
    [47.74996082042011, 51.28895161341555],
    [47.1191495746234, 50.795289734921205],
    [46.4846851085448, 50.30399733387347],
    [45.84677260848657, 49.81469782471648],
    [45.205651342275914, 49.327092955972944],
    [44.561582431570976, 48.84097737476783],
    [43.91483443701089, 48.35625033253587],
    [43.265667519638576, 47.8729241312742],
    [42.614317036971975, 47.391129023819296],
    [41.96097748207152, 46.91111440521276],
    [41.30578767625485, 46.43324626062317],
    [40.64881808042783, 45.958000963779156],
    [39.99006099838817, 45.48595564379846],
    [39.329424312256606, 45.017775453310136],
    [38.66672922180632, 44.55419817290814],
    [38.00171226406979, 44.096016672872885],
    [37.33403167674908, 43.64405982003248],
    [36.663277949107275, 43.199172463646995],
    [35.988988188057284, 42.76219515812577],
    [35.310663725874534, 42.33394428187448],
    [34.62779021950146, 41.91519319107542],
    [33.9398593488435, 41.506655005936054],
    [33.24639112027922, 41.10896756679949],
    [32.54695572742067, 40.72268102098722],
    [31.841193917348168, 40.348248411316085],
    [31.128834858135182, 39.98601953723402],
    [30.409710601039777, 39.636238253016664],
    [29.683766374448247, 39.29904325811366],
    [28.95106613044038, 38.974472326150526],
    [28.2117929806532, 38.66246981473335],
    [27.46624439627075, 38.36289720124969],
    [26.714822296591038, 38.07554630312585],
    [25.958018400139995, 37.80015476684654],
    [25.1963954499746, 37.53642335032626],
    [24.430565139261002, 37.28403447925699],
    [23.66116374395195, 37.04267153059621],
    [22.888826607324553, 36.812038285603336],
    [22.11416270901415, 36.59187800045423],
    [21.33773058389827, 36.381991563636035],
    [20.560016831101294, 36.18225424479929],
    [19.781418370441095, 35.99263058789167],
    [19.002229465394315, 35.81318706029654],
    [18.222634343223824, 35.64410213721319],
    [17.44270601175004, 35.48567357436508],
    [16.662411607857322, 35.33832269997143],
    [15.88162432635373, 35.2025956364442],
    [15.100141681559002, 35.079161441237375],
    [14.31770956093311, 34.96880723257924],
    [13.534051253193013, 34.87243043755642],
    [12.748900385211428, 34.791028365519196],
    [11.962036493998276, 34.72568536762677],
    [11.173321802100112, 34.677557892424026],
    [10.382737664649596, 34.647857786800344],
    [9.590419119494953, 34.63783422098178],
    [8.796686001102032, 34.6487546350957],
    [8.002069174208394, 34.681885113336925],
    [7.207330601620605, 34.73847059012965],
    [6.4134761764394455, 34.81971528142214],
    [5.6217605141852545, 34.92676371408341],
    [4.833683204379518, 35.06068269818154],
    [4.050976351932803, 35.22244455175337],
    [3.2755835827200883, 35.412911846685084],
    [2.509631030806595, 35.632823898770646],
    [1.7553911526040054, 35.88278517621289],
    [1.0152405119721166, 36.163255750137836],
    [0.29161293718626524, 36.474543859463054],
    [-0.4130503453434642, 36.81680061203541],
    [-1.0963518536931645, 37.19001679562145],
    [-1.7559884488912019, 37.59402172730927],
    [-2.38979956727826, 38.02848402927675],
    [-2.9958113362055983, 38.492914183678955],
    [-3.5722749644627143, 38.986668690439046],
    [-4.11769800608365, 39.50895562964505],
    [-4.630867360773571, 40.058841415515026],
    [-5.110863185729826, 40.63525852173899],
    [-5.557063239722119, 41.237013958447974],
    [-5.969137547079875, 41.86279828887762],
    [-6.347033641940001, 42.511194988524345],
    [-6.690953016667258, 43.18068997053983],
    [-7.0013197380424685, 43.86968112736116],
    [-7.278742496829345, 44.57648776901267],
    [-7.523971608416687, 45.29935987185644],
    [-7.7378526741527125, 46.036487086392754],
    [-7.921278736967324, 46.786007487513054],
    [-8.075142815921069, 47.54601608384178],
    [-8.200292680420127, 48.31457313294641],
    [-8.297489627052464, 49.08971233480354],
    [-8.367372854414178, 49.86944899567473],
    [-8.410430800831278, 50.65178826736886],
    [-8.42698052601856, 51.434733571892544],
    [-8.417155892085681, 52.21629531817127],
    [-8.380904945196491, 52.99450000565737],
    [-8.317996531014664, 53.76739978938862],
    [-8.228035809739133, 54.53308255297842],
    [-8.11048798484645, 55.28968250103329],
    [-7.964709237688069, 56.035391241907185],
    [-7.789983580614191, 56.768469287146644],
    [-7.585564115258805, 57.48725784736303],
    [-7.350717018707876, 58.19019075771814],
    [-7.084766484558749, 58.87580632198723],
    [-6.787138821594624, 59.54275882457424],
    [-6.457403960225143, 60.18982942715674],
    [-6.095312733321109, 60.81593614292966],
    [-5.700828478134653, 61.420142568546595],
    [-5.274151741634903, 62.00166505332111],
    [-4.81573715259956, 62.559877998106266],
    [-4.326301838236887, 63.09431700306658],
    [-3.806825097802232, 63.60467962427745],
    [-3.258539386778743, 64.0908235531145],
    [-2.6829129988065787, 64.55276209850274],
    [-2.081625145259416, 64.99065692846227],
    [-1.4565344118419503, 65.40480811164011],
    [-0.8096418070347724, 65.79564158879472],
    [-0.14304979986399502, 66.16369429525469],
    [0.5410811321228408, 66.50959724466976],
    [1.2405768635066607, 66.834056968243],
    [1.953293313104119, 67.13783577839463],
    [2.677157957114609, 67.42173138794574],
    [3.410207778256378, 67.68655646220833],
    [4.150622443310731, 67.93311870907351],
    [4.896751722493391, 68.16220211915135],
    [5.647136418366102, 68.37454995279542],
    [6.400522348169979, 68.57085003282712],
    [7.155867209401173, 68.75172284122327],
    [7.912340441913813, 68.9177128361297],
    [8.669316468970656, 69.0692833044138],
    [9.426361943502329, 69.20681494754291],
    [10.183217834780166, 69.33060826864613],
    [10.939777356890573, 69.44088969063509],
    [11.696060858020271, 69.5378211942092],
    [12.452188855087932, 69.6215131257935],
    [13.208354410533058, 69.69203969444426],
    [13.964796008323361, 69.74945655894832],
    [14.721771997971011, 69.79381980688565],
    [15.479537544126746, 69.8252055509765],
    [16.238324852499947, 69.8437293185345],
    [16.99832724919259, 69.84956439034114],
    [17.75968747976963, 69.8429582577297],
    [18.522490376781256, 69.82424641192503],
    [19.286759830288858, 69.7938627572731],
    [20.052459795090087, 69.75234604816536],
    [20.819498889757952, 69.70034188519394],
    [21.58773799399888, 69.63859996514208],
    [22.357000138275556, 69.56796645651619],
    [23.127081907340052, 69.48937156124471],
    [23.897765549468826, 69.40381251699708],
    [24.66883099586894, 69.31233248594897],
    [25.440067047966235, 69.21599595721058],
    [26.211281080186374, 69.11586145412485],
    [26.98230672679328, 69.01295247723864],
    [27.753009166327505, 68.90822772266048],
    [28.523287778121322, 68.80255168843428],
    [29.29307611353792, 68.69666681439051],
    [30.06233929106149, 68.59116829101731],
    [30.831069080459926, 68.48648261913488],
    [31.599277078904905, 68.38285090513041],
    [32.366986494165374, 68.2803177384914],
    [33.134223131170565, 68.17872632329932],
    [33.901006224384545, 68.0777203287036],
    [34.66733976740362, 67.97675269207316],
    [35.43320496275564, 67.8751013605671],
    [36.19855435074847, 67.77189170120461],
    [36.96330807992513, 67.66612505563988],
    [37.727352658418354, 67.5567126734547],
    [38.490542381837955, 67.44251403640202],
    [39.25270347690362, 67.32237839465735],
    [40.01364083916687, 67.19518818284766],
    [40.77314708641997, 67.0599028752732],
    [41.531013505173405, 66.9156017816184],
    [42.2870423437122, 66.76152428008314],
    [43.04105980854382, 66.59710603580452],
    [43.792929057030314, 66.42200985816226],
    [44.542562451534714, 66.23615000843226],
    [45.28993235156762, 66.03970897460036],
    [46.035079770313914, 65.83314597635984],
    [46.77812030869993, 65.61719674208605],
    [47.51924690008759, 65.39286440116821],
    [48.25872904627241, 65.16140164865838],
    [48.99690839376331, 64.92428465326428],
    [49.73419068021591, 64.68317948248972],
    [50.47103426549447, 64.43990209863799],
    [51.20793564091565, 64.19637322549242],
    [51.945412474660806, 63.954569587907386],
    [52.68398489255006, 63.71647317688573],
    [53.42415580377098, 63.48402028441791],
    [54.166391154520575, 63.25905208095827],
    [54.911101024330904, 63.04326847178464],
    [55.65862246753206, 62.83818686695121],
    [56.40920494538937, 62.64510733589627],
    [57.162999094641314, 62.465085397228954],
    [57.920049439283005, 62.298913424262835],
    [58.68029148031376, 62.14711133697259],
    [59.44355340033904, 62.009926912383975],
    [60.209562405379344, 61.88734569039701],
    [60.97795550496856, 61.77911009394179],
    [61.74829431417348, 61.684747034737356],
    [62.52008325814966, 61.60360295214337],
    [63.29279038143687, 61.534884945306345],
    [64.06586981962896, 61.47770641945052],
    [64.8387848881801, 61.431135485504605],
    [65.61103068797489, 61.39424423598143],
    [66.38215512382274, 61.36615697442045],
    [67.15177728179938, 61.346095503409146],
    [67.91960221342191, 61.333419677084805],
    [68.6854313255783, 61.327661595133556],
    [69.44916776909265, 61.3285520509606],
    [70.2108164477571, 61.3360381386585],
    [70.9704785236929, 61.35029126112557],
    [71.72834056264335, 61.37170515275931],
    [72.48465873190875, 61.40088392069586],
    [73.23973872135281, 61.438620503779575],
    [73.99391229162427, 61.485866333135434],
    [74.74751155160058, 61.543693337401585],
    [75.50084221850554, 61.61324975514977],
    [76.25415721047551, 61.69571148391075],
    [77.00763195609349, 61.79223089850239],
    [77.76134277479579, 61.90388520230271],
    [78.51524958515486, 62.03162642763949],
    [79.26918403696922, 62.17623517243125],
    [80.02284394294269, 62.33828004953639],
    [80.77579461448904, 62.51808463599298],
    [81.52747739440201, 62.71570344754915],
    [82.27722533954449, 62.93090813852436],
    [83.02428565376528, 63.16318474958008],
    [83.7678481204889, 63.41174241001812],
    [84.50707845182383, 63.675533462017206],
    [85.24115517232312, 63.95328452808764],
    [85.96930840550354, 64.2435376067813],
    [86.69085874303397, 64.54469987198406],
    [87.40525426107183, 64.85510048380712],
    [88.11210371367736, 65.17305240865679],
    [88.81120398449096, 65.49691700501445],
    [89.50256001629128, 65.82516896988508],
    [90.18639566135927, 66.15645916599965],
    [90.8631541977384, 66.48967286578029],
    [91.53348762793559, 66.82398105555222],
    [92.19823430452557, 67.1588826398957],
    [92.85838489587182, 67.49423566542542],
    [93.51503719688586, 67.83027603659848],
    [94.16934078500037, 68.16762261149486],
    [94.82243300011679, 68.50726802860508],
    [95.47536816902465, 68.85055511032033],
    [96.12904238035665, 69.19913919757366],
    [96.78411642788777, 69.55493727477523],
    [97.44093976270038, 69.92006522665848],
    [98.09947841632746, 70.29676501140855],
    [98.75924986904903, 70.687323921275],
    [99.41926773576726, 71.09398841848292],
    [100.07799892642112, 71.51887526877529],
    [100.73333561331191, 71.96388283835374],
    [101.3825839129856, 72.43060546654608],
    [102.02247067859214, 72.92025377385951],
    [102.64916921676826, 73.43358361434883],
    [103.25834411103472, 73.97083613709127],
    [103.84521467385079, 74.53169109199689],
    [104.4046358857587, 75.11523511121345],
    [104.93119503711094, 75.71994623267622],
    [105.41932169001964, 76.34369542271531],
    [105.8634080485119, 76.98376531747189],
    [106.25793638436835, 77.63688585653549],
    [106.59760983273677, 78.29928594535215],
    [106.87748265959081, 78.96675977384264],
    [107.09308602237105, 79.63474595456469],
    [107.24054530086866, 80.29841724027493],
    [107.31668526773566, 80.95277825132874],
];
pub const YU1: &[[f64; 2]] = &[
    [-27., 1.],
    [-21.857, -3.214],
    [-16.7, -7.428],
    [-11.571, -11.642],
    [-6.428, -15.857],
    [-1.285, -20.071],
    [3.857, -24.285],
    [9., -28.5],
    [15., -29.9],
    [20., -30.],
    [27.2, -25.],
    [29.2, -20.],
    [28., -10.],
    [22.7, 2.],
    [15., 10.6],
    [5., 16.5],
    [-10., 19.6],
    [-22., 17.],
    [-28., 11.],
    [-29., 5.],
];
pub const YU2: &[[f64; 2]] = &[
    [-24., 40.],
    [-30., 41.],
    [-34., 40.],
    [-38., 36.],
    [-36., 30.],
    [-28., 29.],
    [-21., 31.],
    [-17., 32.],
    [-8., 34.],
    [3., 37.],
    [10., 41.],
    [17., 41.],
    [26., 39.],
    [28., 33.],
    [29., 26.],
    [26., 23.],
    [17., 23.],
    [11., 24.],
    [6., 27.],
    [0., 31.],
];
pub const OPEN_CURVE1: &[[f64; 2]] = &[
    [0.028607755880487345, 47.07692307692308],
    [6.182453909726641, 52.76923076923077],
    [14.797838525111256, 57.07692307692308],
    [24.643992371265103, 58.61538461538461],
    [41.10553083280357, 59.07692307692308],
    [50.18245390972664, 56.76923076923077],
    [60.6439923712651, 51.53846153846154],
    [65.41322314049587, 46.0],
    [68.79783852511126, 36.92307692307692],
    [67.41322314049587, 25.384615384615383],
    [60.6439923712651, 18.153846153846153],
];
pub const TRIANGLE1: &[[f64; 2]] = &[
    [140.1207548649233, 240.19095327938354],
    [122.18597225622764, 236.93008371416616],
    [107.51205921274939, 233.1257358880792],
    [96.64249399535808, 227.14747501851397],
    [111.31640703883635, 220.6257358880792],
    [129.79466790840155, 213.56051849677485],
    [150.44684182144505, 205.40834458373138],
    [173.81640703883633, 193.45182284460094],
    [193.38162443014068, 181.4953011054705],
    [210.2294505170972, 170.6257358880792],
    [224.90336356057546, 161.93008371416616],
    [241.20771138666242, 161.93008371416616],
    [253.70771138666242, 171.16921414894875],
    [264.0337983431842, 184.75617067068788],
    [272.1859722562276, 203.77790980112266],
    [280.8816244301407, 217.90834458373138],
    [281.42510269101024, 231.4953011054705],
    [271.64249399535805, 240.7344315402531],
    [253.16423312579286, 246.16921414894875],
    [237.40336356057546, 248.343127192427],
    [214.57727660405374, 249.43008371416616],
    [195.55553747361895, 247.79964893155744],
    [172.7294505170972, 246.71269240981832],
    [155.8816244301407, 243.9953011054705],
];
pub const TRIANGLE2: &[[f64; 2]] = &[
    [2., 3.5],
    [2.1333, 3.5],
    [2.2667, 3.5],
    [2.4, 3.5],
    [2.5333, 3.5],
    [2.6667, 3.5],
    [2.8, 3.5],
    [2.9333, 3.5],
    [3.0667, 3.5],
    [3.2, 3.5],
    [3.3333, 3.5],
    [3.4667, 3.5],
    [3.6, 3.5],
    [3.7333, 3.5],
    [3.8667, 3.5],
    [4., 3.5],
    [4.1474, 3.4778],
    [4.2817, 3.4131],
    [4.3909, 3.3117],
    [4.4654, 3.1827],
    [4.4986, 3.0374],
    [4.4875, 2.8887],
    [4.433, 2.75],
    [4.3616, 2.6263],
    [4.2901, 2.5026],
    [4.2187, 2.3788],
    [4.1473, 2.2551],
    [4.0759, 2.1314],
    [4.0044, 2.0077],
    [3.933, 1.884],
    [3.8346, 1.7624],
    [3.7034, 1.6772],
    [3.5523, 1.6367],
    [3.396, 1.6449],
    [3.25, 1.701],
];
pub const TRIANGLE3: &[[f64; 2]] = &[
    [5.515148442627067, -8.11495936079062],
    [6.787164098204367, -6.84294370521332],
    [8.10810343284233, -5.619851728696687],
    [9.380119088419631, -4.347836073119387],
    [10.603211064936264, -5.619851728696687],
    [11.8263030414529, -6.891867384273985],
    [13.147242376090864, -8.21280671891195],
    [14.419258031668162, -9.386975016367918],
    [15.593426329124132, -10.707914351005883],
    [16.86544198470143, -11.979930006583182],
    [18.186381319339397, -13.251945662160482],
    [14.614952747910824, -13.300869341221146],
    [11.19029521366425, -13.203021983099816],
    [7.667790321296343, -13.251945662160482],
    [4.145285428928437, -13.203021983099816],
    [0.4760094993785353, -13.15409830403915],
    [1.8458725130771654, -11.979930006583182],
    [3.0689644895937995, -10.707914351005883],
    [4.340980145171098, -9.435898695428584],
];
pub const CRUNODE: &[[f64; 2]] = &[
    [91.29891650574413, 21.948985854733884],
    [90.60723299336192, 21.302180946414268],
    [89.90175662044871, 20.644050755972415],
    [89.18270523481445, 19.97535428156354],
    [88.45033594966421, 19.296881128544115],
    [87.70494654971915, 18.609448695493604],
    [86.94687664032978, 17.913899065988872],
    [86.17650851023181, 17.211095627904207],
    [85.39426768141885, 16.50191944734378],
    [84.60062312316892, 15.7872654293826],
    [83.79608711152787, 15.068038302455705],
    [82.98121472043732, 14.345148467339598],
    [82.15660293611404, 13.619507755088549],
    [81.32288939211062, 12.892025140893153],
    [80.48075072858305, 12.163602462531067],
    [79.6309005854988, 11.435130192799967],
    [78.77408724568055, 10.707483315026046],
    [77.91109094952658, 9.981517349414439],
    [77.04272090881814, 9.258064575678667],
    [76.16981205206332, 8.537930494109098],
    [75.2932215382012, 7.821890563108298],
    [74.41382507908303, 7.1106872463447175],
    [73.53251311387324, 6.405027397199085],
    [72.65018688031138, 5.705580002251075],
    [71.76775442861856, 5.012974299345032],
    [70.88612662372404, 4.327798279451109],
    [70.00621318046004, 3.6505975752674686],
    [69.12891877448789, 2.9818747334489473],
    [68.25513926906098, 2.322088861636935],
    [67.38575809440292, 1.671655636230689],
    [66.52164281260296, 1.0309476521807426],
    [65.6636418966311, 0.4002950920770729],
    [64.81258174748437, -0.2200133114995637],
    [63.96926396872502, -0.8297290489695754],
    [63.13446291288239, -1.4286423725398834],
    [62.308923509480245, -2.0165806323628246],
    [61.49335937992239, -2.5934064421031096],
    [60.68845124020654, -3.159015705780263],
    [59.894845588513476, -3.713335537028165],
    [59.11315367118929, -4.2563221007032155],
    [58.34395071753836, -4.787958405156509],
    [57.58777543120061, -5.30825207154151],
    [56.84512972369992, -5.817233104337667],
    [56.116478674022616, -6.314951684905964],
    [55.402250696794106, -6.801476007425293],
    [54.70283790074505, -7.276890174052781],
    [54.01859661866484, -7.741292163660987],
    [53.34984808989068, -8.194791886079415],
    [52.69687927653505, -8.637509331445948],
    [52.05994379507223, -9.069572822085973],
    [51.43926294554324, -9.49111737230826],
    [50.835026821456424, -9.902283159651553],
    [50.24739548441996, -10.303214109445761],
    [49.6765001886066, -10.694056593069611],
    [49.12244464128608, -11.074958238993311],
    [48.585306286837955, -11.446066854584458],
    [48.06513760285262, -11.807529455720761],
    [47.561967398115094, -12.159491400485003],
    [47.07580210343134, -12.502095622602177],
    [46.60662704738163, -12.835481959804852],
    [46.15440771015882, -13.159786571965434],
    [45.719090949663304, -13.47514144360013],
    [45.300606194974065, -13.781673965214559],
    [44.89886660319115, -14.079506587912775],
    [44.51377017644965, -14.368756545716273],
    [44.145200836634864, -14.649535640126523],
    [43.793029455987906, -14.921950081601771],
    [43.4571148423774, -15.186100382797974],
    [43.137304678534036, -15.442081298633113],
    [42.83343641500009, -15.68998180846954],
    [42.54533811694202, -15.929885135959566],
    [42.27282926531305, -16.161868802363053],
    [42.01572151314127, -16.386004709412887],
    [41.77381939795844, -16.60235924807538],
    [41.54692101158271, -16.810993429819625],
    [41.33481862862717, -17.011963037272743],
    [41.13729929523053, -17.20531879139341],
    [40.954145379599765, -17.391106532541926],
    [40.785135086021924, -17.569367413060256],
    [40.63004293404481, -17.740138099199385],
    [40.48864020455017, -17.903450980442294],
    [40.36069535444745, -18.059334384469466],
    [40.24597440170778, -18.207812796199732],
    [40.144241282435715, -18.348907079511747],
    [40.05525818164437, -18.482634700411793],
    [39.978785839359055, -18.60900995056189],
    [39.9145838336278, -18.728044170217807],
    [39.86241084196471, -18.839745969751938],
    [39.82202488269606, -18.944121449049966],
    [39.79318353762039, -19.04117441417408],
    [39.77564415733296, -19.130906590780164],
    [39.76916405050374, -19.21331783386197],
    [39.77350065833641, -19.28840633347224],
    [39.78841171537446, -19.356168816140766],
    [39.813655397760066, -19.416600741771646],
    [39.84899045999276, -19.46969649585815],
    [39.89417636117729, -19.515449576903777],
    [39.94897338169472, -19.553852778982666],
    [40.01314273117742, -19.584898369412805],
    [40.08644664861831, -19.608578261550072],
    [40.16864849539555, -19.6248841827437],
    [40.25951284194828, -19.63380783752055],
    [40.35880554879575, -19.63534106609103],
    [40.466293842551366, -19.62947599829112],
    [40.58174638754527, -19.616205203094424],
    [40.704933353633244, -19.595521833845474],
    [40.83562648073751, -19.56741976938109],
    [40.9735991406343, -19.53189375122011],
    [41.118626396475705, -19.488939517014643],
    [41.27048506050771, -19.438553930467446],
    [41.428953750424526, -19.38073510792982],
    [41.59381294477802, -19.315482541905617],
    [41.764845037844026, -19.242797221694783],
    [41.941834394331735, -19.162681751420003],
    [42.12456740430886, -19.075140465688253],
    [42.31283253870517, -18.980179543148324],
    [42.50642040574777, -18.877807118214147],
    [42.70512380867612, -18.76803339123321],
    [42.90873780508041, -18.650870737389035],
    [43.11705976820524, -18.52633381463713],
    [43.329889450562035, -18.394439670984735],
    [43.54702905019589, -18.25520785143639],
    [43.76828327995866, -18.10866050494044],
    [43.99345944014779, -17.954822491684894],
    [44.222367494881, -17.793721491106403],
    [44.45482015258983, -17.625388110992017],
    [44.69063295103065, -17.449855998071087],
    [44.929624347230785, -17.26716195051353],
    [45.17161581280756, -17.077346032771327],
    [45.41643193512358, -16.880451693222184],
    [45.6639005247678, -16.676525885098147],
    [45.913852729883274, -16.46561919120753],
    [46.16612315789543, -16.247785952985687],
    [46.42055000523269, -16.0230844044395],
    [46.6769751956718, -15.791576811580821],
    [46.93524452798411, -15.553329617977006],
    [47.19520783360798, -15.308413597080097],
    [47.45671914512351, -15.056904012031513],
    [47.71963687636186, -14.798880783675493],
    [47.98382401504036, -14.534428667550344],
    [48.24914832887711, -14.263637440664294],
    [48.5154825862047, -13.986602098897492],
    [48.78270479217149, -13.703423065906183],
    [49.05069844168876, -13.414206414436062],
    [49.319352790355, -13.119064100978626],
    [49.58856314465959, -12.818114214723884],
    [49.85823117283961, -12.511481241774362],
    [50.128265237830284, -12.199296345584703],
    [50.39858075381028, -11.881697664575029],
    [50.66910056789611, -11.558830627829916],
    [50.93975536857734, -11.23084828973375],
    [51.21048412250504, -10.89791168429953],
    [51.48123454124092, -10.56019019981543],
    [51.75196357953746, -10.217861974250553],
    [52.0226379666389, -9.871114311619039],
    [52.29323477196107, -9.520144119185456],
    [52.56374200630529, -9.165158364989168],
    [52.83415925947728, -8.806374554653814],
    [53.104498374792286, -8.444021225809045],
    [53.37478416043045, -8.078338457662436],
    [53.64505513693733, -7.709578392293253],
    [53.91536431930854, -7.338005763069404],
    [54.185780031024194, -6.963898424182847],
    [54.45638674606869, -6.587547873623883],
    [54.727285953342545, -6.209259759943475],
    [54.99859703590399, -5.829354360849969],
    [55.270458155125894, -5.448167019032951],
    [55.543027127078744, -5.0660485175848535],
    [55.81648227521983, -4.683365373999667],
    [56.09102323976654, -4.300500027991376],
    [56.36687171995408, -3.9178508943418855],
    [56.64427212076261, -3.535832247752815],
    [56.92349207071518, -3.1548739023817873],
    [57.20482277212665, -2.775420644605326],
    [57.48857913992677, -2.397931373851378],
    [57.77509968016945, -2.0228779034672293],
    [58.06474605496221, -1.650743372002431],
    [58.35790227729631, -1.2820202155606957],
    [58.654973477725434, -0.9172076546544794],
    [58.95638418572444, -0.5568086549731142],
    [59.262576072613335, -0.20132633134555178],
    [59.57400511091299, 0.1487402214574267],
    [59.89113811759254, 0.4929006681531902],
    [60.214448666365904, 0.8306787153733701],
    [60.5444123771731, 1.161617880856582],
    [60.88150161894842, 1.485287660418095],
    [61.22617969386309, 1.8012899687686499],
    [61.57889460587375, 2.1092656954239963],
    [61.940072551375756, 2.408901185129835],
    [62.310111302210146, 2.6999344267527654],
    [62.68937367799137, 2.9821607188833106],
    [63.07818132245805, 3.2554375775292606],
    [63.4768090044378, 3.5196886634927367],
    [63.88547965605822, 3.774906535246071],
    [64.30436033828201, 4.021154076641675],
    [64.73355928748356, 4.258564505128092],
    [65.1731241489694, 4.487339931212745],
    [65.62304144779651, 4.707748508444055],
    [66.08323728860542, 4.920120279390918],
    [66.55357921942593, 5.124841881421048],
    [67.0338791441657, 5.322350321920794],
    [67.52389712848293, 5.513126062873734],
    [68.02334591637661, 5.697685668134863],
    [68.53189596101723, 5.876574263821894],
    [69.04918077256838, 6.0503580450450585],
    [69.57480239633072, 6.219617033843438],
    [70.10833685399741, 6.384938257385336],
    [70.64933940629491, 6.546909475955474],
    [71.19734952395712, 6.706113550364762],
    [71.7518954833119, 6.863123500902322],
    [72.31249853070699, 7.018498276756791],
    [72.87867658511081, 7.172779227126924],
    [73.44994746959459, 7.326487243493169],
    [74.02583167964895, 7.48012052664167],
    [74.60585470941562, 7.634152921534799],
    [75.18954896621437, 7.789032757277351],
    [75.77645530968752, 7.945182127389295],
    [76.36612425502278, 8.102996546509306],
    [76.95811688062767, 8.26284492271412],
    [77.55200547985459, 8.425069789149646],
    [78.14737399440392, 8.589987744041444],
    [78.74381826426759, 8.7578900539277],
    [79.34094612585115, 8.929043380782382],
    [79.93837738648722, 9.10369059932573],
    [80.53574370011401, 9.282051676084048],
    [81.13268836557688, 9.464324586561546],
    [81.72886606590646, 9.650686251177657],
    [82.32394256408521, 9.84129347438769],
    [82.91759436826312, 10.036283874661063],
    [83.50950837712509, 10.23577679576806],
    [84.09938151414478, 10.439874192164865],
    [84.68692035775776, 10.648661483213957],
    [85.2718407730386, 10.862208372579037],
    [85.85386754923822, 11.080569630435303],
    [86.43273404651, 11.30378583718294],
    [87.00818185429947, 11.53188408817956],
    [87.57996046316771, 11.764878659654386],
    [88.14782695124539, 12.002771636461084],
    [88.71154568604899, 12.245553502697344],
    [89.2708880420188, 12.493203696486741],
    [89.82563213384438, 12.745691130405035],
    [90.37556256541318, 13.002974679153628],
    [90.92047019404112, 13.265003636149906],
    [91.4601519095124, 13.531718140732849],
    [91.99441042735755, 13.803049577676642],
    [92.52305409573125, 14.07892095067755],
    [93.0458967152075, 14.359247231433713],
    [93.56275737078113, 14.643935685878148],
    [94.07346027535476, 14.932886179058606],
    [94.57783462398875, 15.225991460084927],
    [95.07571445819985, 15.523137428488823],
    [95.56693853960866, 15.82420338326347],
    [96.05135023225517, 16.129062255773995],
    [96.52879739292496, 16.43758082765485],
    [96.9991322688518, 16.749619934736483],
    [97.46221140219012, 17.06503465797397],
    [97.91789554067608, 17.38367450228303],
    [98.36604955392343, 17.705383564126112],
    [98.80654235482591, 18.03000068863057],
    [99.23924682556417, 18.357359616965528],
    [99.66403974773894, 18.687289124651716],
    [100.08080173617645, 19.019613151429574],
    [100.4894171759733, 19.354150923266182],
    [100.88977416236989, 19.690717067039863],
    [101.28176444306007, 20.029121718402923],
    [101.66528336256363, 20.36917062328833],
    [102.04022980830445, 20.71066523349382],
    [102.4065061580526, 21.05340279674818],
    [102.7640182284029, 21.397176441637935],
    [103.11267522397569, 21.741775257749996],
    [103.4523896870359, 22.086984371363837],
    [103.78307744723858, 22.432585017009508],
    [104.10465757121676, 22.77835460519069],
    [104.41705231173711, 23.124066786559492],
    [104.72018705615486, 23.46949151281723],
    [105.01399027390684, 23.81439509460671],
    [105.29839346278588, 24.158540256654067],
    [105.57333109374537, 24.501686190413988],
    [105.83874055398577, 24.843588604467907],
    [106.09456208807845, 25.183999772924835],
    [106.34073873688484, 25.522668582075763],
    [106.5772162740306, 25.859340575554896],
    [106.80394313969595, 26.193757998267433],
    [107.02087037148496, 26.525659839350546],
    [107.22795153213701, 26.854781874443788],
    [107.42514263384383, 27.18085670755762],
    [107.6124020589371, 27.50361381284255],
    [107.7896904767112, 27.82277957657827],
    [107.95697075614618, 28.13807733972075],
    [108.11420787429776, 28.44922744136805],
    [108.26136882012119, 28.755947263528228],
    [108.39842249349836, 29.057951277601546],
    [108.52533959923953, 29.35495109301788],
    [108.6420925358344, 29.64665550850355],
    [108.74865527873169, 29.932770566488166],
    [108.84500325793204, 30.212999611200864],
    [108.93111322968622, 30.487043351048207],
    [109.00696314210006, 30.754599925912196],
    [109.07253199445833, 31.015364980055914],
    [109.12779969009424, 31.269031741378647],
    [109.17274688264735, 31.51529110781832],
    [109.20735481557321, 31.75383174176033],
    [109.23160515479132, 31.984340173376758],
    [109.24547981438647, 32.20650091388781],
    [109.24896077531128, 32.41999657980993],
    [109.2420298970756, 32.6245080293307],
    [109.22466872245306, 32.819714512029805],
    [109.1968582752857, 33.005293833248295],
    [109.15857885152593, 33.1809225344934],
    [109.10980980372139, 33.34627609135451],
    [109.05052931922384, 33.501029130495716],
    [108.98071419248815, 33.644855667381336],
    [108.90033959192345, 33.77742936648308],
    [108.8093788218658, 33.898423825808116],
    [108.70780308036187, 34.00751288767702],
    [108.59558121358597, 34.104370977766685],
    [108.47267946786081, 34.18867347451495],
    [108.33906124041474, 34.260097111057924],
    [108.19468683018667, 34.31832041193739],
    [108.03951319018557, 34.363024166869096],
    [107.87349368312302, 34.39389194390309],
    [107.69657784226752, 34.41061064432942],
    [107.50871113971672, 34.41287110168296],
    [107.3098347645479, 34.40036872717676],
    [107.09988541358965, 34.372804203839856],
    [106.87879509785492, 34.32988423154522],
    [106.6464909679884, 34.2713223249864],
    [106.40289516240546, 34.19683966648628],
    [106.14792468213528, 34.106166015296765],
    [105.88149129672135, 33.99904067476511],
    [105.60350148587506, 33.8752135183958],
    [105.31385642191842, 33.73444607542141],
    [105.01245199837959, 33.576512676000995],
    [104.69917891041835, 33.40120165559109],
    [104.37392279304191, 33.20831661736935],
    [104.03656442332098, 32.997677750836324],
    [103.68697999301477, 32.769123203867146],
    [103.32504145815311, 32.522510504534985],
    [102.9506169721858, 32.25771802797679],
    [102.56357140928304, 31.97464650242415],
    [102.16376698423493, 31.67322054727994],
    [101.75106397514, 31.353390234792982],
    [101.32532155467308, 31.015132665479392],
    [100.88639873516286, 30.658453545972534],
    [100.43415543197744, 30.283388756477407],
    [99.96845364879024, 29.890005893477102],
    [99.48915878717106, 29.478405771822835],
    [98.99614108160478, 29.048723868864464],
    [98.48927715947704, 28.60113169188552],
    [97.96845172378161, 28.13583804883769],
    [97.43355935430152, 27.65309020127222],
    [96.88450642080724, 27.15317487748866],
    [96.32121309941934, 26.636419123319108],
    [95.74361548072869, 26.103190967691006],
    [95.15166775558949, 25.553899880215134],
    [94.54534446175023, 24.98899699858115],
    [93.9246427717166, 24.408975104548794],
    [93.28958479951831, 23.814368328843713],
    [92.64021990145054, 23.205751567322388],
    [91.97662694346114, 22.583739593379768],
];
pub const LINE: &[[f64; 2]] = &[
    [20., 20.],
    [20., 25.],
    [20., 30.],
    [20., 35.],
    [20., 40.],
    [20., 45.],
];
pub const HAND_NORMAL: &[[f64; 2]] = &[
    [0.5, 1.1],
    [0.4, 1.1],
    [0.3, 1.1],
    [0.2, 1.],
    [0.1, 0.9],
    [0.05, 0.75],
    [0.02, 0.6],
    [0., 0.5],
    [0., 0.4],
    [0.03, 0.3],
    [0.1, 0.25],
    [0.15, 0.2],
    [0.2, 0.3],
    [0.3, 0.4],
    [0.4, 0.5],
    [0.5, 0.7],
    [0.6, 0.9],
    [0.6, 1.],
];
pub const HAND_CRUNODE: &[[f64; 2]] = &[
    [4.15, 2.21],
    [4.5, 2.18],
    [4.53, 1.83],
    [4.13, 1.68],
    [3.67, 1.58],
    [2.96, 1.33],
    [2.67, 1.06],
    [2.63, 0.82],
    [2.92, 0.81],
    [3.23, 1.07],
    [3.49, 1.45],
    [3.76, 1.87],
];
pub const HAND_CUSP: &[[f64; 2]] = &[
    [7.03, 5.99],
    [6.95, 5.45],
    [6.77, 5.03],
    [6.4, 4.6],
    [5.91, 4.03],
    [5.43, 3.56],
    [4.93, 2.94],
    [4.67, 2.6],
    [4.38, 2.2],
    [4.04, 1.67],
    [3.76, 1.22],
    [3.76, 1.97],
    [3.76, 2.78],
    [3.76, 3.56],
    [3.76, 4.34],
    [3.76, 4.91],
    [3.76, 5.47],
    [3.8, 5.98],
    [4.07, 6.4],
    [4.53, 6.75],
    [5.07, 6.85],
    [5.05, 6.84],
    [5.89, 6.83],
    [6.41, 6.8],
    [6.92, 6.58],
];
