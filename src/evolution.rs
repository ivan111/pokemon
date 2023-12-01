//! ポケモンのデータを保持する。

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::pokepedia::{Pokepedia, pokepedia_by_no, pokepedia_by_name};

pub fn evolutions(no: &str) -> Vec<&'static Pokepedia> {
    let m = EVOLUTION_NO_MAP.get_or_init(|| {
        let mut m = HashMap::new();

        for [from, to] in &EVOLUTION {
            let v: &mut Vec<_> = m.entry(from.to_string()).or_default();
            let p = pokepedia_by_no(to).unwrap();
            v.push(p);
        }

        m
    });

    match m.get(no) {
        None => Vec::new(),
        Some(p) => p.clone()
    }
}

#[test]
fn test_evolutions_by_no() {
    let p = pokepedia_by_name("ホゲータ").unwrap();
    let v = evolutions(p.no());
    assert_eq!(v[0].name(), "アチゲータ");
}

pub fn rev_evolutions(no: &str) -> Vec<&'static Pokepedia> {
    let m = REV_EVOLUTION_NO_MAP.get_or_init(|| {
        let mut m = HashMap::new();

        for [from, to] in &EVOLUTION {
            let v: &mut Vec<_> = m.entry(to.to_string()).or_default();
            let p = pokepedia_by_no(from).unwrap();
            v.push(p);
        }

        m
    });

    match m.get(no) {
        None => Vec::new(),
        Some(p) => p.clone()
    }
}

#[test]
fn test_rev_evolutions_by_no() {
    let p = pokepedia_by_name("アチゲータ").unwrap();
    let v = rev_evolutions(p.no());
    assert_eq!(v[0].name(), "ホゲータ");
}

static EVOLUTION_NO_MAP: OnceLock<HashMap<String, Vec<&'static Pokepedia>>> = OnceLock::new();
static REV_EVOLUTION_NO_MAP: OnceLock<HashMap<String, Vec<&'static Pokepedia>>> = OnceLock::new();

// Love Revolution
pub const NUM_EVOLUTION: usize = 428;

/// 進化
pub static EVOLUTION: [[&'static str; 2]; NUM_EVOLUTION] = [
    ["0001", "0002"],
    ["0002", "0003"],
    ["0003", "0003M"],
    ["0004", "0005"],
    ["0005", "0006"],
    ["0006", "0006MX"],
    ["0006", "0006MY"],
    ["0007", "0008"],
    ["0008", "0009"],
    ["0009", "0009M"],
    ["0010", "0011"],
    ["0011", "0012"],
    ["0013", "0014"],
    ["0014", "0015"],
    ["0015", "0015M"],
    ["0016", "0017"],
    ["0017", "0018"],
    ["0018", "0018M"],
    ["0019", "0020"],
    ["0021", "0022"],
    ["0023", "0024"],
    ["0172", "0025"],
    ["0025", "0026"],
    ["0027", "0028"],
    ["0029", "0030"],
    ["0030", "0031"],
    ["0032", "0033"],
    ["0033", "0034"],
    ["0173", "0035"],
    ["0035", "0036"],
    ["0037", "0038"],
    ["0174", "0039"],
    ["0039", "0040"],
    ["0041", "0042"],
    ["0042", "0169"],
    ["0043", "0044"],
    ["0044", "0045"],
    ["0046", "0047"],
    ["0048", "0049"],
    ["0050", "0051"],
    ["0052", "0053"],
    ["0052", "0863"],
    ["0054", "0055"],
    ["0056", "0057"],
    ["0058", "0059"],
    ["0060", "0061"],
    ["0061", "0062"],
    ["0063", "0064"],
    ["0064", "0065"],
    ["0065", "0065M"],
    ["0066", "0067"],
    ["0067", "0068"],
    ["0069", "0070"],
    ["0070", "0071"],
    ["0072", "0073"],
    ["0074", "0075"],
    ["0075", "0076"],
    ["0077", "0078"],
    ["0079", "0080"],
    ["0080", "0080M"],
    ["0081", "0082"],
    ["0082", "0462"],
    ["0083", "0865"],
    ["0084", "0085"],
    ["0086", "0087"],
    ["0088", "0089"],
    ["0090", "0091"],
    ["0092", "0093"],
    ["0093", "0094"],
    ["0094", "0094M"],
    ["0095", "0208"],
    ["0208", "0208M"],
    ["0096", "0097"],
    ["0098", "0099"],
    ["0100", "0101"],
    ["0102", "0103"],
    ["0104", "0105"],
    ["0236", "0106"],
    ["0108", "0463"],
    ["0109", "0110"],
    ["0111", "0112"],
    ["0112", "0464"],
    ["0440", "0113"],
    ["0113", "0242"],
    ["0114", "0465"],
    ["0115", "0115M"],
    ["0116", "0117"],
    ["0117", "0230"],
    ["0118", "0119"],
    ["0120", "0121"],
    ["0439", "0122"],
    ["0122", "0866"],
    ["0123", "0212"],
    ["0212", "0212M"],
    ["0238", "0124"],
    ["0239", "0125"],
    ["0125", "0466"],
    ["0240", "0126"],
    ["0126", "0467"],
    ["0127", "0127M"],
    ["0129", "0130"],
    ["0130", "0130M"],
    ["0133", "0134"],
    ["0137", "0233"],
    ["0233", "0474"],
    ["0138", "0139"],
    ["0140", "0141"],
    ["0142", "0142M"],
    ["0446", "0143"],
    ["0147", "0148"],
    ["0148", "0149"],
    ["0152", "0153"],
    ["0153", "0154"],
    ["0155", "0156"],
    ["0156", "0157"],
    ["0158", "0159"],
    ["0159", "0160"],
    ["0161", "0162"],
    ["0163", "0164"],
    ["0165", "0166"],
    ["0167", "0168"],
    ["0170", "0171"],
    ["0175", "0176"],
    ["0176", "0468"],
    ["0177", "0178"],
    ["0179", "0180"],
    ["0180", "0181"],
    ["0181", "0181M"],
    ["0298", "0183"],
    ["0183", "0184"],
    ["0438", "0185"],
    ["0187", "0188"],
    ["0188", "0189"],
    ["0190", "0424"],
    ["0191", "0192"],
    ["0193", "0469"],
    ["0194", "0980"],
    ["0194", "0195"],
    ["0198", "0430"],
    ["0200", "0429"],
    ["0360", "0202"],
    ["0204", "0205"],
    ["0207", "0472"],
    ["0209", "0210"],
    ["0211", "0904"],
    ["0215", "0461"],
    ["0215", "0903"],
    ["0216", "0217"],
    ["0217", "0901"],
    ["0218", "0219"],
    ["0220", "0221"],
    ["0221", "0473"],
    ["0223", "0224"],
    ["0458", "0226"],
    ["0228", "0229"],
    ["0229", "0229M"],
    ["0231", "0232"],
    ["0246", "0247"],
    ["0247", "0248"],
    ["0248", "0248M"],
    ["0252", "0253"],
    ["0253", "0254"],
    ["0254", "0254M"],
    ["0255", "0256"],
    ["0256", "0257"],
    ["0257", "0257M"],
    ["0258", "0259"],
    ["0259", "0260"],
    ["0260", "0260M"],
    ["0261", "0262"],
    ["0263", "0264"],
    ["0264", "0862"],
    ["0265", "0266"],
    ["0266", "0267"],
    ["0268", "0269"],
    ["0270", "0271"],
    ["0271", "0272"],
    ["0273", "0274"],
    ["0274", "0275"],
    ["0276", "0277"],
    ["0278", "0279"],
    ["0280", "0281"],
    ["0281", "0282"],
    ["0282", "0282M"],
    ["0283", "0284"],
    ["0285", "0286"],
    ["0287", "0288"],
    ["0288", "0289"],
    ["0290", "0291"],
    ["0293", "0294"],
    ["0294", "0295"],
    ["0296", "0297"],
    ["0299", "0476"],
    ["0300", "0301"],
    ["0302", "0302M"],
    ["0304", "0305"],
    ["0305", "0306"],
    ["0306", "0306M"],
    ["0307", "0308"],
    ["0308", "0308M"],
    ["0309", "0310"],
    ["0310", "0310M"],
    ["0406", "0315"],
    ["0315", "0407"],
    ["0316", "0317"],
    ["0318", "0319"],
    ["0320", "0321"],
    ["0322", "0323"],
    ["0325", "0326"],
    ["0328", "0329"],
    ["0329", "0330"],
    ["0331", "0332"],
    ["0333", "0334"],
    ["0334", "0334M"],
    ["0339", "0340"],
    ["0341", "0342"],
    ["0343", "0344"],
    ["0345", "0346"],
    ["0347", "0348"],
    ["0349", "0350"],
    ["0353", "0354"],
    ["0354", "0354M"],
    ["0355", "0356"],
    ["0356", "0477"],
    ["0433", "0358"],
    ["0359", "0359M"],
    ["0361", "0362"],
    ["0362", "0362M"],
    ["0363", "0364"],
    ["0364", "0365"],
    ["0366", "0367"],
    ["0371", "0372"],
    ["0372", "0373"],
    ["0373", "0373M"],
    ["0374", "0375"],
    ["0375", "0376"],
    ["0380", "0380M"],
    ["0381", "0381M"],
    ["0382", "0382P"],
    ["0383", "0383P"],
    ["0384", "0384M"],
    ["0387", "0388"],
    ["0388", "0389"],
    ["0390", "0391"],
    ["0391", "0392"],
    ["0393", "0394"],
    ["0394", "0395"],
    ["0396", "0397"],
    ["0397", "0398"],
    ["0399", "0400"],
    ["0401", "0402"],
    ["0403", "0404"],
    ["0404", "0405"],
    ["0408", "0409"],
    ["0410", "0411"],
    ["0412", "0413"],
    ["0415", "0416"],
    ["0418", "0419"],
    ["0420", "0421"],
    ["0422", "0423"],
    ["0425", "0426"],
    ["0427", "0428"],
    ["0428", "0428M"],
    ["0431", "0432"],
    ["0434", "0435"],
    ["0436", "0437"],
    ["0443", "0444"],
    ["0444", "0445"],
    ["0445", "0445M"],
    ["0447", "0448"],
    ["0449", "0450"],
    ["0451", "0452"],
    ["0453", "0454"],
    ["0456", "0457"],
    ["0459", "0460"],
    ["0460", "0460M"],
    ["0495", "0496"],
    ["0496", "0497"],
    ["0498", "0499"],
    ["0499", "0500"],
    ["0501", "0502"],
    ["0502", "0503"],
    ["0504", "0505"],
    ["0506", "0507"],
    ["0507", "0508"],
    ["0509", "0510"],
    ["0511", "0512"],
    ["0513", "0514"],
    ["0515", "0516"],
    ["0517", "0518"],
    ["0519", "0520"],
    ["0520", "0521"],
    ["0522", "0523"],
    ["0524", "0525"],
    ["0525", "0526"],
    ["0527", "0528"],
    ["0529", "0530"],
    ["0532", "0533"],
    ["0533", "0534"],
    ["0535", "0536"],
    ["0536", "0537"],
    ["0540", "0541"],
    ["0541", "0542"],
    ["0543", "0544"],
    ["0544", "0545"],
    ["0546", "0547"],
    ["0548", "0549"],
    ["0551", "0552"],
    ["0552", "0553"],
    ["0554", "0555"],
    ["0557", "0558"],
    ["0559", "0560"],
    ["0562", "0563"],
    ["0562", "0867"],
    ["0564", "0565"],
    ["0566", "0567"],
    ["0568", "0569"],
    ["0570", "0571"],
    ["0572", "0573"],
    ["0574", "0575"],
    ["0575", "0576"],
    ["0577", "0578"],
    ["0578", "0579"],
    ["0580", "0581"],
    ["0582", "0583"],
    ["0583", "0584"],
    ["0585", "0586"],
    ["0588", "0589"],
    ["0590", "0591"],
    ["0592", "0593"],
    ["0595", "0596"],
    ["0597", "0598"],
    ["0599", "0600"],
    ["0600", "0601"],
    ["0602", "0603"],
    ["0603", "0604"],
    ["0605", "0606"],
    ["0607", "0608"],
    ["0608", "0609"],
    ["0610", "0611"],
    ["0611", "0612"],
    ["0613", "0614"],
    ["0616", "0617"],
    ["0619", "0620"],
    ["0622", "0623"],
    ["0624", "0625"],
    ["0627", "0628"],
    ["0629", "0630"],
    ["0633", "0634"],
    ["0634", "0635"],
    ["0636", "0637"],
    ["0650", "0651"],
    ["0651", "0652"],
    ["0653", "0654"],
    ["0654", "0655"],
    ["0656", "0657"],
    ["0657", "0658"],
    ["0659", "0660"],
    ["0661", "0662"],
    ["0662", "0663"],
    ["0664", "0665"],
    ["0665", "0666"],
    ["0667", "0668"],
    ["0669", "0670"],
    ["0670", "0671"],
    ["0674", "0675"],
    ["0677", "0678"],
    ["0682", "0683"],
    ["0684", "0685"],
    ["0686", "0687"],
    ["0688", "0689"],
    ["0690", "0691"],
    ["0692", "0693"],
    ["0694", "0695"],
    ["0696", "0697"],
    ["0698", "0699"],
    ["0704", "0705"],
    ["0705", "0706"],
    ["0708", "0709"],
    ["0710", "0711"],
    ["0712", "0713"],
    ["0714", "0715"],
    ["0719", "0719M"],
    ["0722", "0723"],
    ["0723", "0724"],
    ["0725", "0726"],
    ["0726", "0727"],
    ["0728", "0729"],
    ["0729", "0730"],
    ["0731", "0732"],
    ["0732", "0733"],
    ["0734", "0735"],
    ["0736", "0737"],
    ["0737", "0738"],
    ["0739", "0740"],
    ["0742", "0743"],
    ["0744", "0745Md"],
    ["0744", "0745Mn"],
    ["0747", "0748"],
    ["0751", "0752"],
    ["0753", "0754"],
    ["0755", "0756"],
    ["0757", "0758"],
    ["0759", "0760"],
    ["0761", "0762"],
    ["0762", "0763"],
    ["0767", "0768"],
    ["0769", "0770"],
    ["0782", "0783"],
    ["0783", "0784"],
    ["0789", "0790"],
    ["0790", "0791"],
    ["0808", "0809"],
    ["0819", "0820"],
    ["0831", "0832"],
    ["0906", "0907"],
    ["0907", "0908"],
    ["0909", "0910"],
    ["0910", "0911"],
    ["0912", "0913"],
    ["0913", "0914"],
    ["0915", "0916"],
    ["0919", "0920"],
    ["0921", "0922"],
    ["0922", "0923"],
    ["0996", "0997"],
    ["0997", "0998"],
    ["0999", "1000"]
];