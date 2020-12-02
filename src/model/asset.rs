pub const MAIN_ASSET: Asset = Asset::USDT;

macro_rules! gen_assets {
    ($($asset:ident),* $(,)?) => {
        #[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, serde::Serialize)]
        pub enum Asset {
            $($asset),*
        }

        impl Asset {
            pub fn all() -> Vec<Asset> {
                vec![$(Asset::$asset),*]
            }
        }

        impl std::fmt::Display for Asset {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $(Asset::$asset => stringify!($asset)),*
                })
            }
        }

        impl<S: AsRef<str>> From<S> for Asset {
            fn from(string: S) -> Asset {
                match string.as_ref() {
                    $(stringify!($asset) => Asset::$asset),*,
                    _ => panic!()
                }
            }
        }
    };
}

gen_assets! {
    USDT,
    BTC,
    ETH,
    DOT,
    LINK,
    BNB,
    SRM,
    SXP,
    SXPUP,
    SXPDOWN,
    XRP,
    EOS,
    LTC,
    YFI,
    YFII,
    ADA,
    BCH,
    ATOM,
    BAND,
    TRX,
    NEO,
    TRB,
    ONT,
    CRV,
    QTUM,
    OMG,
    XTZ,
    ZEC,
    VET,
    WNXM,
    STORJ,
    BTT,
    ALGO,
    WAVES,
    XLM,
    BAT,
    XMR,
    AAVE,
    ALPHA,
    AXS,
    FIL,
    HARD,
    SUSHI,
    KAVA,
    DASH,
    NEAR,
    XVS,
    BEL,
    REN,
    EUR,
    CVC,
    RSR,
    YFIUP,
    YFIDOWN,
    RUNE,
    FLM,
    INJ,
    BZRX,
    THETA,
    DOGE,
    UNI,
    PAX,
    SNX,
    AKRO,
    CTK,
    ZIL,
    UNIUP,
    UNIDOWN,
    BTCUP,
    BTCDOWN,
    OCEAN,
    SOL,
    LINKUP,
    LINKDOWN,
    ROSE,
    ANKR,
    COMP,
    BAL,
    UNFI,
    TOMO,
    ARDR,
    ETHDOWN,
    ETHUP,
    IOST,
    ANT,
    ZEN,
    EGLD,
    GBP,
    FET,
    ICX,
    MKR,
    XRPUP,
    XRPDOWN,
}
