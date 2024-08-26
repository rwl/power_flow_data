use crate::dyr::exc::*;
use crate::dyr::gov::*;
use crate::dyr::pss::*;
use crate::dyr::sym::*;

pub enum Record {
    GENCLS(GENCLS),
    GENROU(GENROU),

    SEXS(SEXS),

    TGOV1(TGOV1),
    GAST(GAST),
    HYGOV(HYGOV),

    IEEEST(IEEEST),
    ST2CUT(ST2CUT),
}
