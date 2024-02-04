use super::*;

/// Motion generation task of planar four-bar linkage.
pub type MFbSyn = MotionSyn<MNormFourBar, 6, 2>;
/// Motion generation of a mechanism `M`.
pub type MotionSyn<M, const N: usize, const D: usize> = Syn<efd::PosedEfd<D>, M, N, D>;

impl<M, const N: usize, const D: usize> MotionSyn<M, N, D>
where
    efd::U<D>: efd::EfdDim<D>,
{
    /// Create a new task from target (two series of points). The harmonic
    /// number is selected automatically.
    pub fn from_series<C1, C2>(curve1: C1, curve2: C2, mode: Mode) -> Self
    where
        C1: efd::Curve<D>,
        C2: efd::Curve<D>,
    {
        let efd = efd::PosedEfd::from_series(curve1, curve2, mode.is_target_open());
        Self::from_efd(efd, mode)
    }

    /// Create a new task from target (a curve and its unit vectors). The
    /// harmonic number is selected automatically.
    pub fn from_uvec<C, V>(curve: C, vectors: V, mode: Mode) -> Self
    where
        C: efd::Curve<D>,
        V: efd::Curve<D>,
    {
        let efd = efd::PosedEfd::from_uvec(curve, vectors, mode.is_target_open());
        Self::from_efd(efd, mode)
    }

    /// Create a new task from target EFD coefficients.
    pub fn from_efd(efd: efd::PosedEfd<D>, mode: Mode) -> Self {
        Self::new(efd, mode)
    }

    /// The harmonic used of target EFD.
    pub fn harmonic(&self) -> usize {
        self.tar.harmonic()
    }
}

impl<M, const N: usize, const D: usize> mh::Bounded for MotionSyn<M, N, D>
where
    Self: mh::ObjFunc,
    M: mech::FromVectorized<N>,
    efd::U<D>: efd::EfdDim<D>,
{
    #[inline]
    fn bound(&self) -> &[[f64; 2]] {
        if self.mode == Mode::Partial {
            M::BOUND_PARTIAL
        } else {
            &M::BOUND
        }
    }
}

impl<M, const N: usize, const D: usize> mh::ObjFunc for MotionSyn<M, N, D>
where
    M: SynBound<N> + mech::Normalized<D> + mech::PoseGen<D>,
    M::De: Default + Clone + Sync + Send + 'static,
    efd::Efd<D>: Sync + Send,
    efd::U<D>: efd::EfdDim<D>,
{
    type Fitness = mh::Product<M::De, f64>;

    fn fitness(&self, xs: &[f64]) -> Self::Fitness {
        let is_open = self.mode.is_target_open();
        let get_series = |fb: &M, start, end| {
            let (curve, pose) = fb.pose_in(start, end, self.res);
            (curve.len() > 2).then_some((curve, pose))
        };
        impl_fitness(self.mode, xs, get_series, |((c, v), fb)| {
            let (efd, pose_efd) =
                efd::PosedEfd::from_uvec_harmonic(c, v, is_open, self.harmonic()).into_inner();
            let geo = efd.as_geo().to(self.tar.curve_efd().as_geo());
            let fb = fb.clone().trans_denorm(&geo);
            let err = efd
                .distance(self.tar.curve_efd())
                .max(pose_efd.distance(self.tar.pose_efd()));
            mh::Product::new(err.max(self.unit_err(&geo)), fb)
        })
    }
}
