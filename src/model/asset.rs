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
    XRP,
    EOS,
    LTC,
    YFI,
    LEND,
    ADA,
    BCH,
    ATOM,
    BAND,
    TRX,
}
