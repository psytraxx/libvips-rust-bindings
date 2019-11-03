
#[derive(Debug)]
pub enum Error {
    InitializationError(&'static str),
    IOError(&'static str),
    LinearError,
    GetpointError,
    SystemError,
    AddError,
    SubtractError,
    MultiplyError,
    DivideError,
    RelationalError,
    RemainderError,
    BooleanError,
    Math2Error,
    Complex2Error,
    ComplexformError,
    SumError,
    InvertError,
    MathError,
    AbError,
    SignError,
    RoundError,
    RelationalConstError,
    RemainderConstError,
    BooleanConstError,
    Math2ConstError,
    ComplexError,
    ComplexgetError,
    AvgError,
    MinError,
    MaxError,
    DeviateError,
    StatError,
    HistFindError,
    HistFindNdimError,
    HistFindIndexedError,
    HoughLineError,
    HoughCircleError,
    ProjectError,
    ProfileError,
    MeasureError,
    FindTrimError,
    CopyError,
    TilecacheError,
    LinecacheError,
    SequentialError,
    CacheError,
    EmbedError,
    GravityError,
    FlipError,
    InsertError,
    JoinError,
    ArrayjoinError,
    ExtractAreaError,
    SmartcropError,
    ExtractBandError,
    BandjoinError,
    BandjoinConstError,
    BandrankError,
    BandmeanError,
    BandboolError,
    ReplicateError,
    CastError,
    RotError,
    Rot45Error,
    AutorotError,
    IfthenelseError,
    RecombError,
    BandfoldError,
    BandunfoldError,
    FlattenError,
    PremultiplyError,
    UnpremultiplyError,
    GridError,
    Transpose3DError,
    ScaleError,
    WrapError,
    ZoomError,
    SubsampleError,
    MsbError,
    ByteswapError,
    FalsecolourError,
    GammaError,
    CompositeError,
    Composite2Error,
    BlackError,
    GaussnoiseError,
    TextError,
    XyzError,
    GaussmatError,
    LogmatError,
    EyeError,
    GreyError,
    ZoneError,
    SineError,
    MaskIdealError,
    MaskIdealRingError,
    MaskIdealBandError,
    MaskButterworthError,
    MaskButterworthRingError,
    MaskButterworthBandError,
    MaskGaussianError,
    MaskGaussianRingError,
    MaskGaussianBandError,
    MaskFractalError,
    BuildlutError,
    InvertlutError,
    TonelutError,
    IdentityError,
    FractsurfError,
    WorleyError,
    PerlinError,
    CsvloadError,
    MatrixloadError,
    RawloadError,
    VipsloadError,
    AnalyzeloadError,
    PpmloadError,
    RadloadError,
    PdfloadError,
    PdfloadBufferError,
    SvgloadError,
    SvgloadBufferError,
    GifloadError,
    GifloadBufferError,
    PngloadError,
    PngloadBufferError,
    MatloadError,
    JpegloadError,
    JpegloadBufferError,
    WebploadError,
    WebploadBufferError,
    TiffloadError,
    TiffloadBufferError,
    OpenslideloadError,
    MagickloadError,
    MagickloadBufferError,
    FitsloadError,
    OpenexrloadError,
    HeifloadError,
    HeifloadBufferError,
    CsvsaveError,
    MatrixsaveError,
    MatrixprintError,
    RawsaveError,
    RawsaveFdError,
    VipssaveError,
    PpmsaveError,
    RadsaveError,
    RadsaveBufferError,
    DzsaveError,
    PngsaveError,
    PngsaveBufferError,
    JpegsaveError,
    JpegsaveBufferError,
    JpegsaveMimeError,
    WebpsaveError,
    WebpsaveBufferError,
    TiffsaveError,
    TiffsaveBufferError,
    MagicksaveError,
    MagicksaveBufferError,
    FitssaveError,
    HeifsaveError,
    HeifsaveBufferError,
    ThumbnailError,
    ThumbnailBufferError,
    ThumbnailImageError,
    MapimError,
    ShrinkError,
    ShrinkhError,
    ShrinkvError,
    ReducehError,
    ReducevError,
    ReduceError,
    QuadraticError,
    AffineError,
    SimilarityError,
    RotateError,
    ResizeError,
    ColourspaceError,
    Lab2XYZError,
    Xyz2LabError,
    Lab2LChError,
    Lch2LabError,
    Lch2CMCError,
    Cmc2LchError,
    Xyz2YxyError,
    Yxy2XYZError,
    ScRGB2XYZError,
    Xyz2ScRGBError,
    LabQ2LabError,
    Lab2LabQError,
    LabQ2LabSError,
    LabS2LabQError,
    LabS2LabError,
    Lab2LabSError,
    Rad2FloatError,
    Float2RadError,
    LabQ2SRGBError,
    Srgb2HsvError,
    Hsv2SrgbError,
    IccImportError,
    IccExportError,
    IccTransformError,
    De76Error,
    De00Error,
    DecmcError,
    Srgb2ScRGBError,
    ScRGB2BWError,
    ScRGB2SRGBError,
    Cmyk2XyzError,
    Xyz2CmykError,
    ProfileLoadError,
    MaplutError,
    PercentError,
    StdifError,
    HistCumError,
    HistMatchError,
    HistNormError,
    HistEqualError,
    HistPlotError,
    HistLocalError,
    HistIsmonotonicError,
    HistEntropyError,
    ConvError,
    ConvaError,
    ConvfError,
    ConviError,
    CompassError,
    ConvsepError,
    ConvasepError,
    FastcorError,
    SpcorError,
    SharpenError,
    GaussblurError,
    CannyError,
    SobelError,
    FwfftError,
    InvfftError,
    FreqmultError,
    SpectrumError,
    PhasecorError,
    MorphError,
    RankError,
    CountlineError,
    LabelregionError,
    FillNearestError,
    DrawRectError,
    DrawMaskError,
    DrawLineError,
    DrawCircleError,
    DrawFloodError,
    DrawImageError,
    DrawSmudgeError,
    MergeError,
    MosaicError,
    Mosaic1Error,
    MatchError,
    GlobalbalanceError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InitializationError(msg) => {
                write!(f, "vips error: InitializationError - {}", msg)
            }
            Error::IOError(msg) => write!(f, "vips error: IOError - {}", msg),
            Error::LinearError => write!(
                f,
                "vips error: LinearError. Check error buffer for more details"
            ),
            Error::GetpointError => write!(
                f,
                "vips error: GetpointError. Check error buffer for more details"
            ),
            Error::SystemError => write!(
                f,
                "vips error: SystemError. Check error buffer for more details"
            ),
            Error::AddError => write!(
                f,
                "vips error: AddError. Check error buffer for more details"
            ),
            Error::SubtractError => write!(
                f,
                "vips error: SubtractError. Check error buffer for more details"
            ),
            Error::MultiplyError => write!(
                f,
                "vips error: MultiplyError. Check error buffer for more details"
            ),
            Error::DivideError => write!(
                f,
                "vips error: DivideError. Check error buffer for more details"
            ),
            Error::RelationalError => write!(
                f,
                "vips error: RelationalError. Check error buffer for more details"
            ),
            Error::RemainderError => write!(
                f,
                "vips error: RemainderError. Check error buffer for more details"
            ),
            Error::BooleanError => write!(
                f,
                "vips error: BooleanError. Check error buffer for more details"
            ),
            Error::Math2Error => write!(
                f,
                "vips error: Math2Error. Check error buffer for more details"
            ),
            Error::Complex2Error => write!(
                f,
                "vips error: Complex2Error. Check error buffer for more details"
            ),
            Error::ComplexformError => write!(
                f,
                "vips error: ComplexformError. Check error buffer for more details"
            ),
            Error::SumError => write!(
                f,
                "vips error: SumError. Check error buffer for more details"
            ),
            Error::InvertError => write!(
                f,
                "vips error: InvertError. Check error buffer for more details"
            ),
            Error::MathError => write!(
                f,
                "vips error: MathError. Check error buffer for more details"
            ),
            Error::AbError => write!(
                f,
                "vips error: AbError. Check error buffer for more details"
            ),
            Error::SignError => write!(
                f,
                "vips error: SignError. Check error buffer for more details"
            ),
            Error::RoundError => write!(
                f,
                "vips error: RoundError. Check error buffer for more details"
            ),
            Error::RelationalConstError => write!(
                f,
                "vips error: RelationalConstError. Check error buffer for more details"
            ),
            Error::RemainderConstError => write!(
                f,
                "vips error: RemainderConstError. Check error buffer for more details"
            ),
            Error::BooleanConstError => write!(
                f,
                "vips error: BooleanConstError. Check error buffer for more details"
            ),
            Error::Math2ConstError => write!(
                f,
                "vips error: Math2ConstError. Check error buffer for more details"
            ),
            Error::ComplexError => write!(
                f,
                "vips error: ComplexError. Check error buffer for more details"
            ),
            Error::ComplexgetError => write!(
                f,
                "vips error: ComplexgetError. Check error buffer for more details"
            ),
            Error::AvgError => write!(
                f,
                "vips error: AvgError. Check error buffer for more details"
            ),
            Error::MinError => write!(
                f,
                "vips error: MinError. Check error buffer for more details"
            ),
            Error::MaxError => write!(
                f,
                "vips error: MaxError. Check error buffer for more details"
            ),
            Error::DeviateError => write!(
                f,
                "vips error: DeviateError. Check error buffer for more details"
            ),
            Error::StatError => write!(
                f,
                "vips error: StatError. Check error buffer for more details"
            ),
            Error::HistFindError => write!(
                f,
                "vips error: HistFindError. Check error buffer for more details"
            ),
            Error::HistFindNdimError => write!(
                f,
                "vips error: HistFindNdimError. Check error buffer for more details"
            ),
            Error::HistFindIndexedError => write!(
                f,
                "vips error: HistFindIndexedError. Check error buffer for more details"
            ),
            Error::HoughLineError => write!(
                f,
                "vips error: HoughLineError. Check error buffer for more details"
            ),
            Error::HoughCircleError => write!(
                f,
                "vips error: HoughCircleError. Check error buffer for more details"
            ),
            Error::ProjectError => write!(
                f,
                "vips error: ProjectError. Check error buffer for more details"
            ),
            Error::ProfileError => write!(
                f,
                "vips error: ProfileError. Check error buffer for more details"
            ),
            Error::MeasureError => write!(
                f,
                "vips error: MeasureError. Check error buffer for more details"
            ),
            Error::FindTrimError => write!(
                f,
                "vips error: FindTrimError. Check error buffer for more details"
            ),
            Error::CopyError => write!(
                f,
                "vips error: CopyError. Check error buffer for more details"
            ),
            Error::TilecacheError => write!(
                f,
                "vips error: TilecacheError. Check error buffer for more details"
            ),
            Error::LinecacheError => write!(
                f,
                "vips error: LinecacheError. Check error buffer for more details"
            ),
            Error::SequentialError => write!(
                f,
                "vips error: SequentialError. Check error buffer for more details"
            ),
            Error::CacheError => write!(
                f,
                "vips error: CacheError. Check error buffer for more details"
            ),
            Error::EmbedError => write!(
                f,
                "vips error: EmbedError. Check error buffer for more details"
            ),
            Error::GravityError => write!(
                f,
                "vips error: GravityError. Check error buffer for more details"
            ),
            Error::FlipError => write!(
                f,
                "vips error: FlipError. Check error buffer for more details"
            ),
            Error::InsertError => write!(
                f,
                "vips error: InsertError. Check error buffer for more details"
            ),
            Error::JoinError => write!(
                f,
                "vips error: JoinError. Check error buffer for more details"
            ),
            Error::ArrayjoinError => write!(
                f,
                "vips error: ArrayjoinError. Check error buffer for more details"
            ),
            Error::ExtractAreaError => write!(
                f,
                "vips error: ExtractAreaError. Check error buffer for more details"
            ),
            Error::SmartcropError => write!(
                f,
                "vips error: SmartcropError. Check error buffer for more details"
            ),
            Error::ExtractBandError => write!(
                f,
                "vips error: ExtractBandError. Check error buffer for more details"
            ),
            Error::BandjoinError => write!(
                f,
                "vips error: BandjoinError. Check error buffer for more details"
            ),
            Error::BandjoinConstError => write!(
                f,
                "vips error: BandjoinConstError. Check error buffer for more details"
            ),
            Error::BandrankError => write!(
                f,
                "vips error: BandrankError. Check error buffer for more details"
            ),
            Error::BandmeanError => write!(
                f,
                "vips error: BandmeanError. Check error buffer for more details"
            ),
            Error::BandboolError => write!(
                f,
                "vips error: BandboolError. Check error buffer for more details"
            ),
            Error::ReplicateError => write!(
                f,
                "vips error: ReplicateError. Check error buffer for more details"
            ),
            Error::CastError => write!(
                f,
                "vips error: CastError. Check error buffer for more details"
            ),
            Error::RotError => write!(
                f,
                "vips error: RotError. Check error buffer for more details"
            ),
            Error::Rot45Error => write!(
                f,
                "vips error: Rot45Error. Check error buffer for more details"
            ),
            Error::AutorotError => write!(
                f,
                "vips error: AutorotError. Check error buffer for more details"
            ),
            Error::IfthenelseError => write!(
                f,
                "vips error: IfthenelseError. Check error buffer for more details"
            ),
            Error::RecombError => write!(
                f,
                "vips error: RecombError. Check error buffer for more details"
            ),
            Error::BandfoldError => write!(
                f,
                "vips error: BandfoldError. Check error buffer for more details"
            ),
            Error::BandunfoldError => write!(
                f,
                "vips error: BandunfoldError. Check error buffer for more details"
            ),
            Error::FlattenError => write!(
                f,
                "vips error: FlattenError. Check error buffer for more details"
            ),
            Error::PremultiplyError => write!(
                f,
                "vips error: PremultiplyError. Check error buffer for more details"
            ),
            Error::UnpremultiplyError => write!(
                f,
                "vips error: UnpremultiplyError. Check error buffer for more details"
            ),
            Error::GridError => write!(
                f,
                "vips error: GridError. Check error buffer for more details"
            ),
            Error::Transpose3DError => write!(
                f,
                "vips error: Transpose3DError. Check error buffer for more details"
            ),
            Error::ScaleError => write!(
                f,
                "vips error: ScaleError. Check error buffer for more details"
            ),
            Error::WrapError => write!(
                f,
                "vips error: WrapError. Check error buffer for more details"
            ),
            Error::ZoomError => write!(
                f,
                "vips error: ZoomError. Check error buffer for more details"
            ),
            Error::SubsampleError => write!(
                f,
                "vips error: SubsampleError. Check error buffer for more details"
            ),
            Error::MsbError => write!(
                f,
                "vips error: MsbError. Check error buffer for more details"
            ),
            Error::ByteswapError => write!(
                f,
                "vips error: ByteswapError. Check error buffer for more details"
            ),
            Error::FalsecolourError => write!(
                f,
                "vips error: FalsecolourError. Check error buffer for more details"
            ),
            Error::GammaError => write!(
                f,
                "vips error: GammaError. Check error buffer for more details"
            ),
            Error::CompositeError => write!(
                f,
                "vips error: CompositeError. Check error buffer for more details"
            ),
            Error::Composite2Error => write!(
                f,
                "vips error: Composite2Error. Check error buffer for more details"
            ),
            Error::BlackError => write!(
                f,
                "vips error: BlackError. Check error buffer for more details"
            ),
            Error::GaussnoiseError => write!(
                f,
                "vips error: GaussnoiseError. Check error buffer for more details"
            ),
            Error::TextError => write!(
                f,
                "vips error: TextError. Check error buffer for more details"
            ),
            Error::XyzError => write!(
                f,
                "vips error: XyzError. Check error buffer for more details"
            ),
            Error::GaussmatError => write!(
                f,
                "vips error: GaussmatError. Check error buffer for more details"
            ),
            Error::LogmatError => write!(
                f,
                "vips error: LogmatError. Check error buffer for more details"
            ),
            Error::EyeError => write!(
                f,
                "vips error: EyeError. Check error buffer for more details"
            ),
            Error::GreyError => write!(
                f,
                "vips error: GreyError. Check error buffer for more details"
            ),
            Error::ZoneError => write!(
                f,
                "vips error: ZoneError. Check error buffer for more details"
            ),
            Error::SineError => write!(
                f,
                "vips error: SineError. Check error buffer for more details"
            ),
            Error::MaskIdealError => write!(
                f,
                "vips error: MaskIdealError. Check error buffer for more details"
            ),
            Error::MaskIdealRingError => write!(
                f,
                "vips error: MaskIdealRingError. Check error buffer for more details"
            ),
            Error::MaskIdealBandError => write!(
                f,
                "vips error: MaskIdealBandError. Check error buffer for more details"
            ),
            Error::MaskButterworthError => write!(
                f,
                "vips error: MaskButterworthError. Check error buffer for more details"
            ),
            Error::MaskButterworthRingError => write!(
                f,
                "vips error: MaskButterworthRingError. Check error buffer for more details"
            ),
            Error::MaskButterworthBandError => write!(
                f,
                "vips error: MaskButterworthBandError. Check error buffer for more details"
            ),
            Error::MaskGaussianError => write!(
                f,
                "vips error: MaskGaussianError. Check error buffer for more details"
            ),
            Error::MaskGaussianRingError => write!(
                f,
                "vips error: MaskGaussianRingError. Check error buffer for more details"
            ),
            Error::MaskGaussianBandError => write!(
                f,
                "vips error: MaskGaussianBandError. Check error buffer for more details"
            ),
            Error::MaskFractalError => write!(
                f,
                "vips error: MaskFractalError. Check error buffer for more details"
            ),
            Error::BuildlutError => write!(
                f,
                "vips error: BuildlutError. Check error buffer for more details"
            ),
            Error::InvertlutError => write!(
                f,
                "vips error: InvertlutError. Check error buffer for more details"
            ),
            Error::TonelutError => write!(
                f,
                "vips error: TonelutError. Check error buffer for more details"
            ),
            Error::IdentityError => write!(
                f,
                "vips error: IdentityError. Check error buffer for more details"
            ),
            Error::FractsurfError => write!(
                f,
                "vips error: FractsurfError. Check error buffer for more details"
            ),
            Error::WorleyError => write!(
                f,
                "vips error: WorleyError. Check error buffer for more details"
            ),
            Error::PerlinError => write!(
                f,
                "vips error: PerlinError. Check error buffer for more details"
            ),
            Error::CsvloadError => write!(
                f,
                "vips error: CsvloadError. Check error buffer for more details"
            ),
            Error::MatrixloadError => write!(
                f,
                "vips error: MatrixloadError. Check error buffer for more details"
            ),
            Error::RawloadError => write!(
                f,
                "vips error: RawloadError. Check error buffer for more details"
            ),
            Error::VipsloadError => write!(
                f,
                "vips error: VipsloadError. Check error buffer for more details"
            ),
            Error::AnalyzeloadError => write!(
                f,
                "vips error: AnalyzeloadError. Check error buffer for more details"
            ),
            Error::PpmloadError => write!(
                f,
                "vips error: PpmloadError. Check error buffer for more details"
            ),
            Error::RadloadError => write!(
                f,
                "vips error: RadloadError. Check error buffer for more details"
            ),
            Error::PdfloadError => write!(
                f,
                "vips error: PdfloadError. Check error buffer for more details"
            ),
            Error::PdfloadBufferError => write!(
                f,
                "vips error: PdfloadBufferError. Check error buffer for more details"
            ),
            Error::SvgloadError => write!(
                f,
                "vips error: SvgloadError. Check error buffer for more details"
            ),
            Error::SvgloadBufferError => write!(
                f,
                "vips error: SvgloadBufferError. Check error buffer for more details"
            ),
            Error::GifloadError => write!(
                f,
                "vips error: GifloadError. Check error buffer for more details"
            ),
            Error::GifloadBufferError => write!(
                f,
                "vips error: GifloadBufferError. Check error buffer for more details"
            ),
            Error::PngloadError => write!(
                f,
                "vips error: PngloadError. Check error buffer for more details"
            ),
            Error::PngloadBufferError => write!(
                f,
                "vips error: PngloadBufferError. Check error buffer for more details"
            ),
            Error::MatloadError => write!(
                f,
                "vips error: MatloadError. Check error buffer for more details"
            ),
            Error::JpegloadError => write!(
                f,
                "vips error: JpegloadError. Check error buffer for more details"
            ),
            Error::JpegloadBufferError => write!(
                f,
                "vips error: JpegloadBufferError. Check error buffer for more details"
            ),
            Error::WebploadError => write!(
                f,
                "vips error: WebploadError. Check error buffer for more details"
            ),
            Error::WebploadBufferError => write!(
                f,
                "vips error: WebploadBufferError. Check error buffer for more details"
            ),
            Error::TiffloadError => write!(
                f,
                "vips error: TiffloadError. Check error buffer for more details"
            ),
            Error::TiffloadBufferError => write!(
                f,
                "vips error: TiffloadBufferError. Check error buffer for more details"
            ),
            Error::OpenslideloadError => write!(
                f,
                "vips error: OpenslideloadError. Check error buffer for more details"
            ),
            Error::MagickloadError => write!(
                f,
                "vips error: MagickloadError. Check error buffer for more details"
            ),
            Error::MagickloadBufferError => write!(
                f,
                "vips error: MagickloadBufferError. Check error buffer for more details"
            ),
            Error::FitsloadError => write!(
                f,
                "vips error: FitsloadError. Check error buffer for more details"
            ),
            Error::OpenexrloadError => write!(
                f,
                "vips error: OpenexrloadError. Check error buffer for more details"
            ),
            Error::HeifloadError => write!(
                f,
                "vips error: HeifloadError. Check error buffer for more details"
            ),
            Error::HeifloadBufferError => write!(
                f,
                "vips error: HeifloadBufferError. Check error buffer for more details"
            ),
            Error::CsvsaveError => write!(
                f,
                "vips error: CsvsaveError. Check error buffer for more details"
            ),
            Error::MatrixsaveError => write!(
                f,
                "vips error: MatrixsaveError. Check error buffer for more details"
            ),
            Error::MatrixprintError => write!(
                f,
                "vips error: MatrixprintError. Check error buffer for more details"
            ),
            Error::RawsaveError => write!(
                f,
                "vips error: RawsaveError. Check error buffer for more details"
            ),
            Error::RawsaveFdError => write!(
                f,
                "vips error: RawsaveFdError. Check error buffer for more details"
            ),
            Error::VipssaveError => write!(
                f,
                "vips error: VipssaveError. Check error buffer for more details"
            ),
            Error::PpmsaveError => write!(
                f,
                "vips error: PpmsaveError. Check error buffer for more details"
            ),
            Error::RadsaveError => write!(
                f,
                "vips error: RadsaveError. Check error buffer for more details"
            ),
            Error::RadsaveBufferError => write!(
                f,
                "vips error: RadsaveBufferError. Check error buffer for more details"
            ),
            Error::DzsaveError => write!(
                f,
                "vips error: DzsaveError. Check error buffer for more details"
            ),
            Error::PngsaveError => write!(
                f,
                "vips error: PngsaveError. Check error buffer for more details"
            ),
            Error::PngsaveBufferError => write!(
                f,
                "vips error: PngsaveBufferError. Check error buffer for more details"
            ),
            Error::JpegsaveError => write!(
                f,
                "vips error: JpegsaveError. Check error buffer for more details"
            ),
            Error::JpegsaveBufferError => write!(
                f,
                "vips error: JpegsaveBufferError. Check error buffer for more details"
            ),
            Error::JpegsaveMimeError => write!(
                f,
                "vips error: JpegsaveMimeError. Check error buffer for more details"
            ),
            Error::WebpsaveError => write!(
                f,
                "vips error: WebpsaveError. Check error buffer for more details"
            ),
            Error::WebpsaveBufferError => write!(
                f,
                "vips error: WebpsaveBufferError. Check error buffer for more details"
            ),
            Error::TiffsaveError => write!(
                f,
                "vips error: TiffsaveError. Check error buffer for more details"
            ),
            Error::TiffsaveBufferError => write!(
                f,
                "vips error: TiffsaveBufferError. Check error buffer for more details"
            ),
            Error::MagicksaveError => write!(
                f,
                "vips error: MagicksaveError. Check error buffer for more details"
            ),
            Error::MagicksaveBufferError => write!(
                f,
                "vips error: MagicksaveBufferError. Check error buffer for more details"
            ),
            Error::FitssaveError => write!(
                f,
                "vips error: FitssaveError. Check error buffer for more details"
            ),
            Error::HeifsaveError => write!(
                f,
                "vips error: HeifsaveError. Check error buffer for more details"
            ),
            Error::HeifsaveBufferError => write!(
                f,
                "vips error: HeifsaveBufferError. Check error buffer for more details"
            ),
            Error::ThumbnailError => write!(
                f,
                "vips error: ThumbnailError. Check error buffer for more details"
            ),
            Error::ThumbnailBufferError => write!(
                f,
                "vips error: ThumbnailBufferError. Check error buffer for more details"
            ),
            Error::ThumbnailImageError => write!(
                f,
                "vips error: ThumbnailImageError. Check error buffer for more details"
            ),
            Error::MapimError => write!(
                f,
                "vips error: MapimError. Check error buffer for more details"
            ),
            Error::ShrinkError => write!(
                f,
                "vips error: ShrinkError. Check error buffer for more details"
            ),
            Error::ShrinkhError => write!(
                f,
                "vips error: ShrinkhError. Check error buffer for more details"
            ),
            Error::ShrinkvError => write!(
                f,
                "vips error: ShrinkvError. Check error buffer for more details"
            ),
            Error::ReducehError => write!(
                f,
                "vips error: ReducehError. Check error buffer for more details"
            ),
            Error::ReducevError => write!(
                f,
                "vips error: ReducevError. Check error buffer for more details"
            ),
            Error::ReduceError => write!(
                f,
                "vips error: ReduceError. Check error buffer for more details"
            ),
            Error::QuadraticError => write!(
                f,
                "vips error: QuadraticError. Check error buffer for more details"
            ),
            Error::AffineError => write!(
                f,
                "vips error: AffineError. Check error buffer for more details"
            ),
            Error::SimilarityError => write!(
                f,
                "vips error: SimilarityError. Check error buffer for more details"
            ),
            Error::RotateError => write!(
                f,
                "vips error: RotateError. Check error buffer for more details"
            ),
            Error::ResizeError => write!(
                f,
                "vips error: ResizeError. Check error buffer for more details"
            ),
            Error::ColourspaceError => write!(
                f,
                "vips error: ColourspaceError. Check error buffer for more details"
            ),
            Error::Lab2XYZError => write!(
                f,
                "vips error: Lab2XYZError. Check error buffer for more details"
            ),
            Error::Xyz2LabError => write!(
                f,
                "vips error: Xyz2LabError. Check error buffer for more details"
            ),
            Error::Lab2LChError => write!(
                f,
                "vips error: Lab2LChError. Check error buffer for more details"
            ),
            Error::Lch2LabError => write!(
                f,
                "vips error: Lch2LabError. Check error buffer for more details"
            ),
            Error::Lch2CMCError => write!(
                f,
                "vips error: Lch2CMCError. Check error buffer for more details"
            ),
            Error::Cmc2LchError => write!(
                f,
                "vips error: Cmc2LchError. Check error buffer for more details"
            ),
            Error::Xyz2YxyError => write!(
                f,
                "vips error: Xyz2YxyError. Check error buffer for more details"
            ),
            Error::Yxy2XYZError => write!(
                f,
                "vips error: Yxy2XYZError. Check error buffer for more details"
            ),
            Error::ScRGB2XYZError => write!(
                f,
                "vips error: ScRGB2XYZError. Check error buffer for more details"
            ),
            Error::Xyz2ScRGBError => write!(
                f,
                "vips error: Xyz2ScRGBError. Check error buffer for more details"
            ),
            Error::LabQ2LabError => write!(
                f,
                "vips error: LabQ2LabError. Check error buffer for more details"
            ),
            Error::Lab2LabQError => write!(
                f,
                "vips error: Lab2LabQError. Check error buffer for more details"
            ),
            Error::LabQ2LabSError => write!(
                f,
                "vips error: LabQ2LabSError. Check error buffer for more details"
            ),
            Error::LabS2LabQError => write!(
                f,
                "vips error: LabS2LabQError. Check error buffer for more details"
            ),
            Error::LabS2LabError => write!(
                f,
                "vips error: LabS2LabError. Check error buffer for more details"
            ),
            Error::Lab2LabSError => write!(
                f,
                "vips error: Lab2LabSError. Check error buffer for more details"
            ),
            Error::Rad2FloatError => write!(
                f,
                "vips error: Rad2FloatError. Check error buffer for more details"
            ),
            Error::Float2RadError => write!(
                f,
                "vips error: Float2RadError. Check error buffer for more details"
            ),
            Error::LabQ2SRGBError => write!(
                f,
                "vips error: LabQ2SRGBError. Check error buffer for more details"
            ),
            Error::Srgb2HsvError => write!(
                f,
                "vips error: Srgb2HsvError. Check error buffer for more details"
            ),
            Error::Hsv2SrgbError => write!(
                f,
                "vips error: Hsv2SrgbError. Check error buffer for more details"
            ),
            Error::IccImportError => write!(
                f,
                "vips error: IccImportError. Check error buffer for more details"
            ),
            Error::IccExportError => write!(
                f,
                "vips error: IccExportError. Check error buffer for more details"
            ),
            Error::IccTransformError => write!(
                f,
                "vips error: IccTransformError. Check error buffer for more details"
            ),
            Error::De76Error => write!(
                f,
                "vips error: De76Error. Check error buffer for more details"
            ),
            Error::De00Error => write!(
                f,
                "vips error: De00Error. Check error buffer for more details"
            ),
            Error::DecmcError => write!(
                f,
                "vips error: DecmcError. Check error buffer for more details"
            ),
            Error::Srgb2ScRGBError => write!(
                f,
                "vips error: Srgb2ScRGBError. Check error buffer for more details"
            ),
            Error::ScRGB2BWError => write!(
                f,
                "vips error: ScRGB2BWError. Check error buffer for more details"
            ),
            Error::ScRGB2SRGBError => write!(
                f,
                "vips error: ScRGB2SRGBError. Check error buffer for more details"
            ),
            Error::Cmyk2XyzError => write!(
                f,
                "vips error: Cmyk2XyzError. Check error buffer for more details"
            ),
            Error::Xyz2CmykError => write!(
                f,
                "vips error: Xyz2CmykError. Check error buffer for more details"
            ),
            Error::ProfileLoadError => write!(
                f,
                "vips error: ProfileLoadError. Check error buffer for more details"
            ),
            Error::MaplutError => write!(
                f,
                "vips error: MaplutError. Check error buffer for more details"
            ),
            Error::PercentError => write!(
                f,
                "vips error: PercentError. Check error buffer for more details"
            ),
            Error::StdifError => write!(
                f,
                "vips error: StdifError. Check error buffer for more details"
            ),
            Error::HistCumError => write!(
                f,
                "vips error: HistCumError. Check error buffer for more details"
            ),
            Error::HistMatchError => write!(
                f,
                "vips error: HistMatchError. Check error buffer for more details"
            ),
            Error::HistNormError => write!(
                f,
                "vips error: HistNormError. Check error buffer for more details"
            ),
            Error::HistEqualError => write!(
                f,
                "vips error: HistEqualError. Check error buffer for more details"
            ),
            Error::HistPlotError => write!(
                f,
                "vips error: HistPlotError. Check error buffer for more details"
            ),
            Error::HistLocalError => write!(
                f,
                "vips error: HistLocalError. Check error buffer for more details"
            ),
            Error::HistIsmonotonicError => write!(
                f,
                "vips error: HistIsmonotonicError. Check error buffer for more details"
            ),
            Error::HistEntropyError => write!(
                f,
                "vips error: HistEntropyError. Check error buffer for more details"
            ),
            Error::ConvError => write!(
                f,
                "vips error: ConvError. Check error buffer for more details"
            ),
            Error::ConvaError => write!(
                f,
                "vips error: ConvaError. Check error buffer for more details"
            ),
            Error::ConvfError => write!(
                f,
                "vips error: ConvfError. Check error buffer for more details"
            ),
            Error::ConviError => write!(
                f,
                "vips error: ConviError. Check error buffer for more details"
            ),
            Error::CompassError => write!(
                f,
                "vips error: CompassError. Check error buffer for more details"
            ),
            Error::ConvsepError => write!(
                f,
                "vips error: ConvsepError. Check error buffer for more details"
            ),
            Error::ConvasepError => write!(
                f,
                "vips error: ConvasepError. Check error buffer for more details"
            ),
            Error::FastcorError => write!(
                f,
                "vips error: FastcorError. Check error buffer for more details"
            ),
            Error::SpcorError => write!(
                f,
                "vips error: SpcorError. Check error buffer for more details"
            ),
            Error::SharpenError => write!(
                f,
                "vips error: SharpenError. Check error buffer for more details"
            ),
            Error::GaussblurError => write!(
                f,
                "vips error: GaussblurError. Check error buffer for more details"
            ),
            Error::CannyError => write!(
                f,
                "vips error: CannyError. Check error buffer for more details"
            ),
            Error::SobelError => write!(
                f,
                "vips error: SobelError. Check error buffer for more details"
            ),
            Error::FwfftError => write!(
                f,
                "vips error: FwfftError. Check error buffer for more details"
            ),
            Error::InvfftError => write!(
                f,
                "vips error: InvfftError. Check error buffer for more details"
            ),
            Error::FreqmultError => write!(
                f,
                "vips error: FreqmultError. Check error buffer for more details"
            ),
            Error::SpectrumError => write!(
                f,
                "vips error: SpectrumError. Check error buffer for more details"
            ),
            Error::PhasecorError => write!(
                f,
                "vips error: PhasecorError. Check error buffer for more details"
            ),
            Error::MorphError => write!(
                f,
                "vips error: MorphError. Check error buffer for more details"
            ),
            Error::RankError => write!(
                f,
                "vips error: RankError. Check error buffer for more details"
            ),
            Error::CountlineError => write!(
                f,
                "vips error: CountlineError. Check error buffer for more details"
            ),
            Error::LabelregionError => write!(
                f,
                "vips error: LabelregionError. Check error buffer for more details"
            ),
            Error::FillNearestError => write!(
                f,
                "vips error: FillNearestError. Check error buffer for more details"
            ),
            Error::DrawRectError => write!(
                f,
                "vips error: DrawRectError. Check error buffer for more details"
            ),
            Error::DrawMaskError => write!(
                f,
                "vips error: DrawMaskError. Check error buffer for more details"
            ),
            Error::DrawLineError => write!(
                f,
                "vips error: DrawLineError. Check error buffer for more details"
            ),
            Error::DrawCircleError => write!(
                f,
                "vips error: DrawCircleError. Check error buffer for more details"
            ),
            Error::DrawFloodError => write!(
                f,
                "vips error: DrawFloodError. Check error buffer for more details"
            ),
            Error::DrawImageError => write!(
                f,
                "vips error: DrawImageError. Check error buffer for more details"
            ),
            Error::DrawSmudgeError => write!(
                f,
                "vips error: DrawSmudgeError. Check error buffer for more details"
            ),
            Error::MergeError => write!(
                f,
                "vips error: MergeError. Check error buffer for more details"
            ),
            Error::MosaicError => write!(
                f,
                "vips error: MosaicError. Check error buffer for more details"
            ),
            Error::Mosaic1Error => write!(
                f,
                "vips error: Mosaic1Error. Check error buffer for more details"
            ),
            Error::MatchError => write!(
                f,
                "vips error: MatchError. Check error buffer for more details"
            ),
            Error::GlobalbalanceError => write!(
                f,
                "vips error: GlobalbalanceError. Check error buffer for more details"
            ),
        }
    }
}

impl std::error::Error for Error {}