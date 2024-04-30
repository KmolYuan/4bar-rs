use crate::{
    io,
    syn_cmd::{self, Target},
};
use four_bar::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

macro_rules! impl_err_from {
    ($(($ty:ty, $kind:ident)),+ $(,)?) => {$(
        impl From<$ty> for SynErr {
            fn from(e: $ty) -> Self { Self::$kind(e) }
        }
    )+};
}

#[derive(Debug)]
enum SynErr {
    Format,
    Io(std::io::Error),
    Plot(plot::DrawingAreaErrorKind<std::io::Error>),
    CsvSer(csv::Error),
    RonSerde(ron::error::SpannedError),
    RonIo(ron::error::Error),
    Linkage,
}

impl std::fmt::Display for SynErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Format => write!(f, "unsupported format"),
            Self::Io(e) => write!(f, "[IO] {e}"),
            Self::Plot(e) => write!(f, "[Plot] {e}"),
            Self::CsvSer(e) => write!(f, "[CSV] {e}"),
            Self::RonSerde(e) => write!(f, "[RON-Serde] {e}"),
            Self::RonIo(e) => write!(f, "[RON-IO] {e}"),
            Self::Linkage => write!(f, "invalid linkage input"),
        }
    }
}

impl std::error::Error for SynErr {}

impl_err_from!(
    (std::io::Error, Io),
    (plot::DrawingAreaErrorKind<std::io::Error>, Plot),
    (csv::Error, CsvSer),
    (ron::error::SpannedError, RonSerde),
    (ron::error::Error, RonIo),
);

#[derive(clap::Args)]
#[clap(subcommand_precedence_over_arg = true)]
pub(super) struct Syn {
    /// Target file paths in "[path]/[name].[mode].[ron|csv|txt]" pattern
    #[clap(required = true)]
    files: Vec<PathBuf>,
    /// Force to rerun the result
    ///
    /// If the last result exists, the program will only redraw it
    #[clap(short = 'f', long, alias = "force")]
    rerun: bool,
    /// Remove the related project folders and exit
    ///
    /// This flag won't run the synthesis functions
    #[clap(long, alias = "clear")]
    clean: bool,
    /// Disable parallel for running all tasks, use a single loop for
    /// benchmarking
    #[clap(long)]
    each: bool,
    /// Provide pre-generated atlas databases, support multiple paths as
    #[cfg_attr(windows, doc = "\"a.npz;b.npz\"")]
    #[cfg_attr(not(windows), doc = "\"a.npz:b.npz\"")]
    ///
    /// If the atlas is provided, the rerun flag will be enabled
    #[clap(long)]
    atlas: Option<std::ffi::OsString>,
    /// Competitor path starting from file root with the same filename
    #[clap(short, long, default_value = "refer")]
    refer: PathBuf,
    /// Disable reference comparison
    #[clap(long)]
    no_ref: bool,
    #[clap(flatten)]
    cfg: syn_cmd::SynCfg,
    #[clap(subcommand)]
    alg: Option<syn_cmd::SynAlg>,
}

struct Info {
    root: PathBuf,
    title: String,
    mode: syn::Mode,
}

fn get_info<'a>(
    title: &str,
    file: &Path,
    res: usize,
    atlas: Option<&'a io::AtlasPool>,
    rerun: bool,
    clean: bool,
) -> Result<(Info, Target<'static, 'a>), SynErr> {
    let ext = file.extension().and_then(|p| p.to_str());
    macro_rules! check {
        ($c:expr) => {
            efd::util::valid_curve($c).ok_or(SynErr::Linkage)?.into()
        };
        (@ $c:expr) => {{
            let c = $c;
            if c.len() < 3
                || c.iter()
                    .flat_map(|(c, v)| c.iter().chain(v))
                    .any(|x| !x.is_finite())
            {
                return Err(SynErr::Linkage);
            } else {
                c.into()
            }
        }};
    }
    let target = match ext.ok_or(SynErr::Format)? {
        "csv" | "txt" => match io::Curve::from_csv_reader(std::fs::File::open(file)?)? {
            io::Curve::P(t) => Target::fb(check!(t), None, atlas.map(|a| a.as_fb())),
            io::Curve::M(t) => Target::mfb(check!(@t), None),
            io::Curve::S(t) => Target::sfb(check!(t), None, None),
        },
        "ron" => match ron::de::from_reader(std::fs::File::open(file)?)? {
            io::Fb::P(fb) => Target::fb(check!(fb.curve(res)), Some(fb), None),
            io::Fb::M(fb) => Target::mfb(check!(@fb.pose_zipped(res)), Some(fb)),
            io::Fb::S(fb) => Target::sfb(check!(fb.curve(res)), Some(fb), None),
        },
        _ => {
            println!("Ignored: {}", file.display());
            Err(SynErr::Format)?
        }
    };
    let mode = match Path::new(title).extension().and_then(|p| p.to_str()) {
        Some("closed") => syn::Mode::Closed,
        Some("partial") => syn::Mode::Partial,
        Some("open") => syn::Mode::Open,
        _ => Err(SynErr::Format)?,
    };
    let root = file.parent().unwrap().join(title);
    if root.is_dir() {
        if rerun {
            // Clear the root folder
            // Avoid file browser missing opening folders
            for e in std::fs::read_dir(&root)? {
                let path = e?.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(path)?;
                } else {
                    std::fs::remove_file(path)?;
                }
            }
        } else if clean {
            // Just remove root folder
            std::fs::remove_dir_all(&root)?;
        }
    } else if !clean || rerun {
        std::fs::create_dir(&root)?;
    }
    let title = title.to_string();
    Ok((Info { root, title, mode }, target))
}

pub(super) fn syn(syn: Syn) {
    let Syn {
        files,
        each,
        cfg,
        mut atlas,
        refer,
        no_ref,
        alg,
        rerun,
        clean,
    } = syn;
    println!("=====");
    if let Some(seed) = cfg.seed {
        print!("seed={seed} ");
    }
    println!("gen={} pop={} res={}", cfg.gen, cfg.pop, cfg.res);
    // If atlas is provided, rerun is always enabled
    if !rerun {
        atlas = None;
    }
    println!("rerun={rerun} clean={clean}");
    println!("-----");
    // Load atlas
    let atlas = atlas
        .map(|atlas| std::env::split_paths(&atlas).collect::<Vec<_>>())
        .unwrap_or_default();
    let atlas = if atlas.is_empty() {
        None
    } else {
        println!("Loading atlas database...");
        Some(
            atlas
                .into_iter()
                .map(|path| Ok(io::Atlas::from_reader(std::fs::File::open(path)?)?))
                .collect::<Result<io::AtlasPool, Box<dyn std::error::Error>>>()
                .expect("Load atlas failed"),
        )
    };
    // Load target files & create project folders
    let tasks = files
        .into_iter()
        .filter_map(|file| {
            let file = file.canonicalize().ok().filter(|f| f.is_file())?;
            let title = file.file_stem()?.to_str()?;
            match get_info(title, &file, cfg.res, atlas.as_ref(), rerun, clean) {
                Ok(info) => Some(info),
                Err(SynErr::Format) => None,
                Err(e) => {
                    println!("Error in {title}: {e}");
                    None
                }
            }
        })
        .collect::<Vec<_>>();
    if tasks.is_empty() {
        panic!("No valid target files!");
    }
    if clean && !rerun {
        return;
    }
    // Progress bar
    const STYLE: &str = "{eta} {wide_bar} {percent}%";
    let pb = ProgressBar::new(tasks.len() as u64 * cfg.gen);
    pb.set_style(ProgressStyle::with_template(STYLE).unwrap());
    // Tasks
    let alg = alg.unwrap_or_default();
    let refer = (!no_ref).then_some(refer.as_path());
    let run = |(info, target)| run(&pb, alg.clone(), info, target, &cfg, refer, rerun);
    let t0 = std::time::Instant::now();
    if each {
        tasks.into_iter().for_each(run);
    } else {
        use mh::rayon::prelude::*;
        tasks.into_par_iter().for_each(run);
    }
    pb.finish_and_clear();
    println!("-----");
    println!("Finished in {:?}", t0.elapsed());
}

const HISTORY_SVG: &str = "history.svg";
const LNK_RON: &str = "linkage.ron";
const LNK_SVG: &str = "linkage.svg";
const LNK_FIG: &str = "linkage.fig.ron";
const EFD_CSV: &str = "target-efd.csv";
const CURVE_SVG: &str = "curve.svg";
const CURVE_FIG: &str = "curve.fig.ron";

impl<M, const N: usize, const D: usize> syn_cmd::PathSynData<'_, M, N, D>
where
    syn::PathSyn<M, N, D>: mh::ObjFunc<Ys = mh::WithProduct<f64, M::De>>,
    M: atlas::Code<N, D>,
    M::De: mech::CurveGen<D>
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Default
        + Clone
        + Sync
        + Send
        + 'static,
    efd::U<D>: efd::EfdDim<D>,
    efd::Efd<D>: Sync,
    for<'f1, 'f2> plot::FigureBase<'f1, 'f2, M::De, [f64; D]>: plot::Plot + serde::Serialize,
{
    fn solve_cli(
        self,
        cfg: &syn_cmd::SynCfg,
        info: &Info,
        refer: Option<&Path>,
        history: Arc<Mutex<Vec<f64>>>,
    ) -> Result<(), SynErr> {
        use four_bar::mech::CurveGen as _;
        use plot::full_palette::*;
        let Self { s, target, target_fb, atlas_fb } = self;
        let Info { root, title, mode } = info;
        let t0 = std::time::Instant::now();
        let s = s.solve();
        let t1 = t0.elapsed();
        let func = s.func();
        let harmonic = func.harmonic();
        let efd = func.tar.clone();
        let (cost, fb) = s.into_err_result();
        {
            let path = root.join(HISTORY_SVG);
            let svg = plot::SVGBackend::new(&path, (800, 600));
            plot::fb::history(svg, Arc::into_inner(history).unwrap().into_inner().unwrap())?;
        }
        let refer = refer
            .map(|p| root.join("..").join(p).join(format!("{title}.ron")))
            .filter(|p| p.is_file());
        let mut log = std::fs::File::create(root.join(format!("{title}.log")))?;
        let mut log = super::logger::Logger::new(&mut log);
        log.top_title(title)?;
        write_tar_efd(root.join(EFD_CSV), &efd)?;
        write_ron(root.join(LNK_RON), &fb)?;
        let efd_target = efd::Efd::from_curve_harmonic(&target, mode.is_target_open(), harmonic);
        let curve = fb.curve(cfg.res);
        let mut fig = plot::FigureBase::new_ref(&fb)
            .add_line("Target", &*target, plot::Style::Circle, RED)
            .add_line("Optimized", &curve, plot::Style::Line, BLUE_900);
        {
            write_ron(root.join(LNK_FIG), &fig)?;
            let path = root.join(LNK_SVG);
            let svg = plot::SVGBackend::new(&path, (1600, 1600));
            fig.plot(svg)?;
        }
        if let Some(fb) = target_fb {
            log.title("target.fb")?;
            log.log(fb)?;
        }
        if let Some((cost, fb)) = atlas_fb {
            let c = fb.curve(cfg.res);
            let efd = efd::Efd::from_curve_harmonic(c, mode.is_result_open(), harmonic);
            let geo = efd.as_geo().to(efd_target.as_geo());
            let fb = fb.clone().trans_denorm(&geo);
            let c = fb.curve(cfg.res.min(30));
            log.title("atlas")?;
            log.log(Performance::cost(cost).harmonic(harmonic))?;
            log.title("atlas.fb")?;
            log.log(&fb)?;
            write_ron(root.join("atlas.ron"), &fb)?;
            fig.push_line("Atlas", c, plot::Style::Triangle, GREEN_900);
        }
        log.title("optimized")?;
        log.log(Performance::cost(cost).time(t1).harmonic(harmonic))?;
        log.title("optimized.fb")?;
        log.log(&fb)?;
        if let Some(refer) = refer {
            let fb = ron::de::from_reader::<_, M::De>(std::fs::File::open(refer)?)?;
            let c = fb.curve(cfg.res);
            log.title("competitor")?;
            if !matches!(mode, syn::Mode::Partial) {
                let efd = efd::Efd::from_curve_harmonic(&c, mode.is_result_open(), harmonic);
                log.log(Performance::cost(efd.err(&efd_target)))?;
            }
            log.title("competitor.fb")?;
            log.log(&fb)?;
            fig.push_line("Ref. [?]", c, plot::Style::DashedLine, ORANGE_900);
        }
        fig.fb = None;
        write_ron(root.join(CURVE_FIG), &fig)?;
        let path = root.join(CURVE_SVG);
        let svg = plot::SVGBackend::new(&path, (1600, 1600));
        fig.plot(svg)?;
        log.flush()?;
        Ok(())
    }
}

fn from_runtime(
    pb: &ProgressBar,
    alg: syn_cmd::SynAlg,
    info: &Info,
    target: Target,
    cfg: &syn_cmd::SynCfg,
    refer: Option<&Path>,
) -> Result<(), SynErr> {
    let history = Arc::new(Mutex::new(Vec::with_capacity(cfg.gen as usize)));
    let s = {
        let history = history.clone();
        let cfg = syn_cmd::SynCfg { mode: info.mode, ..cfg.clone() };
        let stop = || false;
        syn_cmd::Solver::new(alg, target, cfg, stop, move |best_f, _| {
            history.lock().unwrap().push(best_f);
            pb.inc(1);
        })
    };
    match s {
        syn_cmd::Solver::Fb(s) => s.solve_cli(cfg, info, refer, history),
        syn_cmd::Solver::SFb(s) => s.solve_cli(cfg, info, refer, history),
    }
}

fn from_exist(root: &Path, target: &Target) -> Result<(), SynErr> {
    fn plot<Fig>(root: &Path) -> Result<(), SynErr>
    where
        Fig: serde::de::DeserializeOwned + plot::Plot,
    {
        for (path, svg_path) in [
            (root.join(LNK_FIG), root.join(LNK_SVG)),
            (root.join(CURVE_FIG), root.join(CURVE_SVG)),
        ] {
            ron::de::from_reader::<_, Fig>(std::fs::File::open(path)?)?
                .plot(plot::SVGBackend::new(&svg_path, (1600, 1600)))?;
        }
        Ok(())
    }
    match target {
        // HINT: `fb::Figure` and `mfb::Figure` are the same type
        Target::Fb { .. } | Target::MFb { .. } => plot::<plot::fb::Figure>(root),
        Target::SFb { .. } => plot::<plot::sfb::Figure>(root),
    }
}

fn run(
    pb: &ProgressBar,
    alg: syn_cmd::SynAlg,
    info: Info,
    target: Target,
    cfg: &syn_cmd::SynCfg,
    refer: Option<&Path>,
    rerun: bool,
) {
    // FIXME: Try block, ignore errors
    let f = || {
        let root = &info.root;
        if !rerun && root.join(LNK_FIG).is_file() && root.join(CURVE_FIG).is_file() {
            pb.inc(cfg.gen);
            from_exist(&info.root, &target)
        } else {
            from_runtime(pb, alg, &info, target, cfg, refer)
        }
    };
    let title = &info.title;
    match f() {
        Ok(()) => pb.println(format!("Finished: {title}")),
        Err(e) => pb.println(format!("Error in {title}: {e}")),
    }
}

#[derive(serde::Serialize)]
struct Performance {
    #[serde(serialize_with = "ser_time")]
    time: Option<std::time::Duration>,
    cost: f64,
    harmonic: Option<usize>,
}

fn ser_time<S>(time: &Option<std::time::Duration>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match time {
        Some(time) => s.serialize_str(&format!("{:.3?}", time)),
        None => s.serialize_none(),
    }
}

impl Performance {
    fn cost(cost: f64) -> Self {
        Self { time: None, cost, harmonic: None }
    }

    fn time(self, time: std::time::Duration) -> Self {
        Self { time: Some(time), ..self }
    }

    fn harmonic(self, harmonic: usize) -> Self {
        Self { harmonic: Some(harmonic), ..self }
    }
}

fn write_ron<S>(path: impl AsRef<Path>, s: &S) -> Result<(), SynErr>
where
    S: serde::Serialize,
{
    std::fs::write(path, ron::ser::to_string_pretty(s, Default::default())?)?;
    Ok(())
}

fn write_tar_efd<const D: usize>(path: impl AsRef<Path>, efd: &efd::Efd<D>) -> Result<(), SynErr>
where
    efd::U<D>: efd::EfdDim<D>,
{
    use std::io::Write as _;
    let mut w = std::fs::File::create(path)?;
    for m in efd.coeffs_iter() {
        for (i, c) in m.iter().enumerate() {
            if i == m.len() - 1 {
                write!(w, "{c:.4}")?;
            } else {
                write!(w, "{c:.4},")?;
            }
        }
        writeln!(w)?;
    }
    w.flush()?;
    Ok(())
}
