use super::{TranslitMethod, Transliterator};

// Russian
const SOURCE_RU: &'static str =
    "Везувий зев открыл — дым хлынул клубом — пламя \
     Широко развилось, как боевое знамя. \
     Земля волнуется — с шатнувшихся колонн \
     Кумиры падают! Народ, гонимый страхом, \
     Толпами, стар и млад, под воспаленным прахом, \
     Под каменным дождем бежит из града вон.";

const TRANSLIT_GOST779B_RU: &'static str =
    "Vezuvij zev otkry`l — dy`m xly`nul klubom — plamya \
     Shiroko razvilos`, kak boevoe znamya. \
     Zemlya volnuetsya — s shatnuvshixsya kolonn \
     Kumiry` padayut! Narod, gonimy`j straxom, \
     Tolpami, star i mlad, pod vospalenny`m praxom, \
     Pod kamenny`m dozhdem bezhit iz grada von.";

#[test]
fn test_russian_to_latin_translit_gost779b_ru_1() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_ru).convert("Терминал", false),
        "Terminal"
    );
}

#[test]
fn test_russian_to_latin_translit_gost779b_ru_2() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_ru).convert(SOURCE_RU, false),
        TRANSLIT_GOST779B_RU
    );
}

#[test]
fn test_fn_to_latin_gost779b_ru_1() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_ru).to_latin(SOURCE_RU),
        TRANSLIT_GOST779B_RU
    );
}

#[test]
fn test_latin_to_russian_translit_gost779b_ru_1() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_ru).convert("Terminal", true),
        "Терминал"
    );
}

#[test]
fn test_latin_to_russian_translit_gost779b_ru_2() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_ru).convert(TRANSLIT_GOST779B_RU, true),
        SOURCE_RU
    );
}

#[test]
fn test_fn_from_latin_gost779b_ru_2() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_ru).from_latin(TRANSLIT_GOST779B_RU),
        SOURCE_RU
    );
}

const SOURCE_PASSPORT_2013_RU_1: &'static str = "Большое преимущество получает тот, \
    кто достаточно рано сделал ошибки на которых можно учиться.© Уинстон Черчилль";

const TRANSLIT_PASSPORT_2013_RU_1: &'static str =
    "Bolshoe preimushchestvo poluchaet tot, \
     kto dostatochno rano sdelal oshibki na kotorykh mozhno uchitsia.© Uinston Cherchill";

#[test]
fn test_fn_to_latin_iternational_passport_2013_ru_1() {
    assert_eq!(
        Transliterator::new(TranslitMethod::iternational_passport_2013_ru)
            .to_latin(SOURCE_PASSPORT_2013_RU_1),
        TRANSLIT_PASSPORT_2013_RU_1
    );
}

// Belarusian
const SOURCE_BY: &'static str =
    "У рудога вераб'я ў сховішчы \
     пад фатэлем ляжаць нейкія гаючыя зёлкі.";

const TRANSLIT_BY: &'static str = "U rudoha verab'ya u` sxovishchy` \
                                   pad fate`lem lyazhac` nejkiya hayuchy`ya zyolki.";

#[test]
fn test_fn_to_latin_gost779b_by_1() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_by).to_latin(SOURCE_BY),
        TRANSLIT_BY
    );
}

#[test]
fn test_fn_from_latin_gost779b_by_1() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_by).from_latin(TRANSLIT_BY),
        SOURCE_BY
    );
}

// Ukrainian
const SOURCE_UA: &'static str =
    "Гей, хлопці, не вспію - на ґанку \
     ваша файна їжа знищується бурундучком.";

const TRANSLIT_UA: &'static str = "Gej, xlopci, ne vspiyu - na g`anku \
                                   vasha fajna yizha zny`shhuyet`sya burunduchkom.";

#[test]
fn test_fn_to_latin_gost779b_ua_1() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_ua).to_latin(SOURCE_UA),
        TRANSLIT_UA
    );
}

#[test]
fn test_fn_from_latin_gost779b_ua_1() {
    assert_eq!(
        Transliterator::new(TranslitMethod::gost779b_ua).from_latin(TRANSLIT_UA),
        SOURCE_UA
    );
}