/*
 * Data taken 2019-05-26 from https://www.kmu.gov.ua/ua/npas/243262567
 */

pub fn translit<'a>(input: &'a str) -> String {
    let input_len = input.len();
    if input_len == 0 {
        return input.to_string();
    }

    let mut output:Vec<u8> = Vec::with_capacity(input_len + input_len / 4);

    let mut is_word_start = true;
    let mut after_z = false;
    let mut buffer = [0; 4];

    for ch in input[..].chars() {
        match ch {
            'з'|'З' => {
                after_z = true;
            }, 
            'г' => if after_z {
                is_word_start = false;
                output.extend(b"gh");
                continue;
            },
            'Г' => if after_z {
                if is_word_start {
                    is_word_start = false;
                    output.extend(b"Gh");
                } else {
                    output.extend(b"GH");
                }
                continue;
            },
            '’'|'\'' => {
                if is_word_start {
                    is_word_start = true;
                    output.extend_from_slice(
                        &ch.encode_utf8(&mut buffer).as_bytes());
                    continue;
                } else {
                    // apostroph should be skipped
                    continue;
                }
            },
            'ь'|'Ь' => {
                // should be skipped
                continue;
            },
            _ => { after_z = false; }
        }

        if is_word_start {
            if let Some(latin) = translit_word_start(ch) {
                output.extend(latin);
                is_word_start = false;
            } else {
                output.extend_from_slice(
                    &ch.encode_utf8(&mut buffer).as_bytes());
            }
        } else {
            if let Some(latin) = translit_word_rest(ch) {
                output.extend(latin);
            } else {
                output.extend_from_slice(
                    &ch.encode_utf8(&mut buffer).as_bytes());
                is_word_start =  true;
            }
        }
    }

    unsafe {String::from_utf8_unchecked(output)}
}

pub fn translit_word_start(ch: char) -> Option<&'static [u8]> {
    match ch {
        'а' => Some(b"a"),
        'А' => Some(b"A"),
        'б' => Some(b"b"),
        'Б' => Some(b"B"),
        'в' => Some(b"v"),
        'В' => Some(b"V"),
        'г' => Some(b"h"),
        'Г' => Some(b"H"),
        'ґ' => Some(b"g"),
        'Ґ' => Some(b"G"),
        'д' => Some(b"d"),
        'Д' => Some(b"D"),
        'е' => Some(b"e"),
        'Е' => Some(b"E"),
        'є' => Some(b"ye"),
        'Є' => Some(b"Ye"),
        'ж' => Some(b"zh"),
        'Ж' => Some(b"Zh"),
        'з' => Some(b"z"),
        'З' => Some(b"Z"),
        'и' => Some(b"y"),
        'И' => Some(b"Y"),
        'і' => Some(b"i"),
        'І' => Some(b"I"),
        'ї' => Some(b"yi"),
        'Ї' => Some(b"Yi"),
        'й' => Some(b"y"),
        'Й' => Some(b"Y"),
        'к' => Some(b"k"),
        'К' => Some(b"K"),
        'л' => Some(b"l"),
        'Л' => Some(b"L"),
        'м' => Some(b"m"),
        'М' => Some(b"M"),
        'н' => Some(b"n"),
        'Н' => Some(b"N"),
        'о' => Some(b"o"),
        'О' => Some(b"O"),
        'п' => Some(b"p"),
        'П' => Some(b"P"),
        'р' => Some(b"r"),
        'Р' => Some(b"R"),
        'с' => Some(b"s"),
        'С' => Some(b"S"),
        'т' => Some(b"t"),
        'Т' => Some(b"T"),
        'у' => Some(b"u"),
        'У' => Some(b"U"),
        'ф' => Some(b"f"),
        'Ф' => Some(b"F"),
        'х' => Some(b"kh"),
        'Х' => Some(b"Kh"),
        'ц' => Some(b"ts"),
        'Ц' => Some(b"Ts"),
        'ч' => Some(b"ch"),
        'Ч' => Some(b"Ch"),
        'ш' => Some(b"sh"),
        'Ш' => Some(b"Sh"),
        'щ' => Some(b"shch"),
        'Щ' => Some(b"Shch"),
        'ю' => Some(b"yu"),
        'Ю' => Some(b"Yu"),
        'я' => Some(b"ya"),
        'Я' => Some(b"Ya"),
        _ => None
    }
}

pub fn translit_word_rest(ch: char) -> Option<&'static [u8]> {
    match ch {
        'а' => Some(b"a"),
        'А' => Some(b"A"),
        'б' => Some(b"b"),
        'Б' => Some(b"B"),
        'в' => Some(b"v"),
        'В' => Some(b"V"),
        'г' => Some(b"h"),
        'Г' => Some(b"H"),
        'ґ' => Some(b"g"),
        'Ґ' => Some(b"G"),
        'д' => Some(b"d"),
        'Д' => Some(b"D"),
        'е' => Some(b"e"),
        'Е' => Some(b"E"),
        'є' => Some(b"ie"),
        'Є' => Some(b"IE"),
        'ж' => Some(b"zh"),
        'Ж' => Some(b"ZH"),
        'з' => Some(b"z"),
        'З' => Some(b"Z"),
        'и' => Some(b"y"),
        'И' => Some(b"Y"),
        'і' => Some(b"i"),
        'І' => Some(b"I"),
        'ї' => Some(b"i"),
        'Ї' => Some(b"I"),
        'й' => Some(b"i"),
        'Й' => Some(b"I"),
        'к' => Some(b"k"),
        'К' => Some(b"K"),
        'л' => Some(b"l"),
        'Л' => Some(b"L"),
        'м' => Some(b"m"),
        'М' => Some(b"M"),
        'н' => Some(b"n"),
        'Н' => Some(b"N"),
        'о' => Some(b"o"),
        'О' => Some(b"O"),
        'п' => Some(b"p"),
        'П' => Some(b"P"),
        'р' => Some(b"r"),
        'Р' => Some(b"R"),
        'с' => Some(b"s"),
        'С' => Some(b"S"),
        'т' => Some(b"t"),
        'Т' => Some(b"T"),
        'у' => Some(b"u"),
        'У' => Some(b"U"),
        'ф' => Some(b"f"),
        'Ф' => Some(b"F"),
        'х' => Some(b"kh"),
        'Х' => Some(b"KH"),
        'ц' => Some(b"ts"),
        'Ц' => Some(b"TS"),
        'ч' => Some(b"ch"),
        'Ч' => Some(b"CH"),
        'ш' => Some(b"sh"),
        'Ш' => Some(b"SH"),
        'щ' => Some(b"shch"),
        'Щ' => Some(b"SHCH"),
        'ю' => Some(b"iu"),
        'Ю' => Some(b"IU"),
        'я' => Some(b"ia"),
        'Я' => Some(b"IA"),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn smoke_test() {
        assert_eq!(Some(b"Y" as &[u8]), translit_word_start('Й'));
        assert_eq!(Some(b"y" as &[u8]), translit_word_start('й'));
        assert_eq!(Some(b"I" as &[u8]), translit_word_rest('Й'));
        assert_eq!(Some(b"i" as &[u8]), translit_word_rest('й'));
    }

    #[test]
    pub fn with_ascii_test() {
        assert_eq!("before Alushta after",  translit("before Алушта after"));
        assert_eq!("before_Alushta_after",  translit("before_Алушта_after"));
        assert_eq!("beforeAlushtaafter",  translit("beforeАлуштаafter"));
    }

    #[test]
    pub fn edge_cases_test() {
        assert_eq!("ye",   translit("є"));
        assert_eq!("pie",  translit("пє"));
        assert_eq!("pie",  translit("п’є"));
        assert_eq!("pie",  translit("п'є"));
        assert_eq!("'ye",  translit("'є"));
    }

    #[test]
    pub fn words_test() {
        assert_eq!("Alushta",        translit("Алушта"));
        assert_eq!("Andrii",         translit("Андрій"));
        assert_eq!("Borshchahivka",  translit("Борщагівка"));
        assert_eq!("Borysenko",      translit("Борисенко"));
        assert_eq!("Vinnytsia",      translit("Вінниця"));
        assert_eq!("Volodymyr",      translit("Володимир"));
        assert_eq!("Hadiach",        translit("Гадяч"));
        assert_eq!("Bohdan",         translit("Богдан"));
        assert_eq!("Zghurskyi",      translit("Згурський"));
        assert_eq!("Pidzghurskyi",   translit("Підзгурський"));
        assert_eq!("Galagan",        translit("Ґалаґан"));
        assert_eq!("Gorgany",        translit("Ґорґани"));
        assert_eq!("Donetsk",        translit("Донецьк"));
        assert_eq!("Dmytro",         translit("Дмитро"));
        assert_eq!("Rivne",          translit("Рівне"));
        assert_eq!("Oleh",           translit("Олег"));
        assert_eq!("Esman",          translit("Есмань"));
        assert_eq!("Yenakiieve",     translit("Єнакієве"));
        assert_eq!("Haievych",       translit("Гаєвич"));
        assert_eq!("Koropie",        translit("Короп’є"));
        assert_eq!("Zhytomyr",       translit("Житомир"));
        assert_eq!("Zhanna",         translit("Жанна"));
        assert_eq!("Zhezheliv",      translit("Жежелів"));
        assert_eq!("Zakarpattia",    translit("Закарпаття"));
        assert_eq!("Kazymyrchuk",    translit("Казимирчук"));
        assert_eq!("Medvyn",         translit("Медвин"));
        assert_eq!("Mykhailenko",    translit("Михайленко"));
        assert_eq!("Ivankiv",        translit("Іванків"));
        assert_eq!("Ivashchenko",    translit("Іващенко"));
        assert_eq!("Yizhakevych",    translit("Їжакевич"));
        assert_eq!("Kadyivka",       translit("Кадиївка"));
        assert_eq!("Marine",         translit("Мар’їне")); 
        assert_eq!("Yosypivka",      translit("Йосипівка"));
        assert_eq!("Stryi",          translit("Стрий"));
        assert_eq!("Oleksii",        translit("Олексій"));
        assert_eq!("Kyiv",           translit("Київ"));
        assert_eq!("Kovalenko",      translit("Коваленко"));
        assert_eq!("Lebedyn",        translit("Лебедин"));
        assert_eq!("Leonid",         translit("Леонід"));
        assert_eq!("Mykolaiv",       translit("Миколаїв"));
        assert_eq!("Marynych",       translit("Маринич"));
        assert_eq!("Nizhyn",         translit("Ніжин"));
        assert_eq!("Nataliia",       translit("Наталія"));
        assert_eq!("Odesa",          translit("Одеса"));
        assert_eq!("Onyshchenko",    translit("Онищенко"));
        assert_eq!("Poltava",        translit("Полтава"));
        assert_eq!("Petro",          translit("Петро"));
        assert_eq!("Reshetylivka",   translit("Решетилівка"));
        assert_eq!("Rybchynskyi",    translit("Рибчинський"));
        assert_eq!("Sumy",           translit("Суми"));
        assert_eq!("Solomiia",       translit("Соломія"));
        assert_eq!("Ternopil",       translit("Тернопіль"));
        assert_eq!("Trots",          translit("Троць"));
        assert_eq!("Uzhhorod",       translit("Ужгород"));
        assert_eq!("Uliana",         translit("Уляна"));
        assert_eq!("Fastiv",         translit("Фастів"));
        assert_eq!("Filipchuk",      translit("Філіпчук"));
        assert_eq!("Kharkiv",        translit("Харків"));
        assert_eq!("Khrystyna",      translit("Христина"));
        assert_eq!("Bila Tserkva",   translit("Біла Церква"));
        assert_eq!("Stetsenko",      translit("Стеценко"));
        assert_eq!("Chernivtsi",     translit("Чернівці"));
        assert_eq!("Shevchenko",     translit("Шевченко"));
        assert_eq!("Shostka",        translit("Шостка"));
        assert_eq!("Kyshenky",       translit("Кишеньки"));
        assert_eq!("Shcherbukhy",    translit("Щербухи"));
        assert_eq!("Hoshcha",        translit("Гоща"));
        assert_eq!("Harashchenko",   translit("Гаращенко")); 
        assert_eq!("Yurii",          translit("Юрій"));
        assert_eq!("Koriukivka",     translit("Корюківка"));
        assert_eq!("Yahotyn",        translit("Яготин"));
        assert_eq!("Yaroshenko",     translit("Ярошенко"));
        assert_eq!("Kostiantyn",     translit("Костянтин"));
        assert_eq!("Znamianka",      translit("Знам’янка"));           
        assert_eq!("Feodosiia",      translit("Феодосія"));
    }

    #[test]
    pub fn words_upper_test() {
        assert_eq!("ALUSHTA",        translit("АЛУШТА")); 
        assert_eq!("ANDRII",         translit("АНДРІЙ"));
        assert_eq!("BORSHCHAHIVKA",  translit("БОРЩАГІВКА"));
        assert_eq!("BORYSENKO",      translit("БОРИСЕНКО"));
        assert_eq!("VINNYTSIA",      translit("ВІННИЦЯ"));
        assert_eq!("VOLODYMYR",      translit("ВОЛОДИМИР"));
        assert_eq!("HADIACH",        translit("ГАДЯЧ"));
        assert_eq!("BOHDAN",         translit("БОГДАН"));
        assert_eq!("ZGHURSKYI",     translit("ЗГУРСЬКИЙ"));
        assert_eq!("PIDZGHURSKYI",     translit("ПІДЗГУРСЬКИЙ"));
        assert_eq!("GALAGAN",        translit("ҐАЛАҐАН"));
        assert_eq!("GORGANY",        translit("ҐОРҐАНИ"));
        assert_eq!("DONETSK",        translit("ДОНЕЦЬК"));
        assert_eq!("DMYTRO",         translit("ДМИТРО"));
        assert_eq!("RIVNE",          translit("РІВНЕ"));
        assert_eq!("OLEH",           translit("ОЛЕГ"));
        assert_eq!("ESMAN",          translit("ЕСМАНЬ"));
        assert_eq!("YeNAKIIEVE",     translit("ЄНАКІЄВЕ"));
        assert_eq!("HAIEVYCH",       translit("ГАЄВИЧ"));
        assert_eq!("KOROPIE",        translit("КОРОП’Є"));
        assert_eq!("ZhYTOMYR",       translit("ЖИТОМИР"));
        assert_eq!("ZhANNA",         translit("ЖАННА"));
        assert_eq!("ZhEZHELIV",      translit("ЖЕЖЕЛІВ"));
        assert_eq!("ZAKARPATTIA",    translit("ЗАКАРПАТТЯ"));
        assert_eq!("KAZYMYRCHUK",    translit("КАЗИМИРЧУК"));
        assert_eq!("MEDVYN",         translit("МЕДВИН"));
        assert_eq!("MYKHAILENKO",    translit("МИХАЙЛЕНКО"));
        assert_eq!("IVANKIV",        translit("ІВАНКІВ"));
        assert_eq!("IVASHCHENKO",    translit("ІВАЩЕНКО"));
        assert_eq!("YiZHAKEVYCH",    translit("ЇЖАКЕВИЧ"));
        assert_eq!("KADYIVKA",       translit("КАДИЇВКА"));
        assert_eq!("MARINE",         translit("МАР’ЇНЕ")); 
        assert_eq!("YOSYPIVKA",      translit("ЙОСИПІВКА"));
        assert_eq!("STRYI",          translit("СТРИЙ"));
        assert_eq!("OLEKSII",        translit("ОЛЕКСІЙ"));
        assert_eq!("KYIV",           translit("КИЇВ"));
        assert_eq!("KOVALENKO",      translit("КОВАЛЕНКО"));
        assert_eq!("LEBEDYN",        translit("ЛЕБЕДИН"));
        assert_eq!("LEONID",         translit("ЛЕОНІД"));
        assert_eq!("MYKOLAIV",       translit("МИКОЛАЇВ"));
        assert_eq!("MARYNYCH",       translit("МАРИНИЧ"));
        assert_eq!("NIZHYN",         translit("НІЖИН"));
        assert_eq!("NATALIIA",       translit("НАТАЛІЯ"));
        assert_eq!("ODESA",          translit("ОДЕСА"));
        assert_eq!("ONYSHCHENKO",    translit("ОНИЩЕНКО"));
        assert_eq!("POLTAVA",        translit("ПОЛТАВА"));
        assert_eq!("PETRO",          translit("ПЕТРО"));
        assert_eq!("RESHETYLIVKA",   translit("РЕШЕТИЛІВКА"));
        assert_eq!("RYBCHYNSKYI",    translit("РИБЧИНСЬКИЙ"));
        assert_eq!("SUMY",           translit("СУМИ"));
        assert_eq!("SOLOMIIA",       translit("СОЛОМІЯ"));
        assert_eq!("TERNOPIL",       translit("ТЕРНОПІЛЬ"));
        assert_eq!("TROTS",          translit("ТРОЦЬ"));
        assert_eq!("UZHHOROD",       translit("УЖГОРОД"));
        assert_eq!("ULIANA",         translit("УЛЯНА"));
        assert_eq!("FASTIV",         translit("ФАСТІВ"));
        assert_eq!("FILIPCHUK",      translit("ФІЛІПЧУК"));
        assert_eq!("KhARKIV",        translit("ХАРКІВ"));
        assert_eq!("KhRYSTYNA",      translit("ХРИСТИНА"));
        assert_eq!("BILA TsERKVA",   translit("БІЛА ЦЕРКВА"));
        assert_eq!("STETSENKO",      translit("СТЕЦЕНКО"));
        assert_eq!("ChERNIVTSI",     translit("ЧЕРНІВЦІ"));
        assert_eq!("ShEVCHENKO",     translit("ШЕВЧЕНКО"));
        assert_eq!("ShOSTKA",        translit("ШОСТКА"));
        assert_eq!("KYSHENKY",       translit("КИШЕНЬКИ"));
        assert_eq!("ShchERBUKHY",    translit("ЩЕРБУХИ"));
        assert_eq!("HOSHCHA",        translit("ГОЩА"));
        assert_eq!("HARASHCHENKO",   translit("ГАРАЩЕНКО")); 
        assert_eq!("YuRII",          translit("ЮРІЙ"));
        assert_eq!("KORIUKIVKA",     translit("КОРЮКІВКА"));
        assert_eq!("YaHOTYN",        translit("ЯГОТИН"));
        assert_eq!("YaROSHENKO",     translit("ЯРОШЕНКО"));
        assert_eq!("KOSTIANTYN",     translit("КОСТЯНТИН"));
        assert_eq!("ZNAMIANKA",      translit("ЗНАМ’ЯНКА"));           
        assert_eq!("FEODOSIIA",      translit("ФЕОДОСІЯ"));
    }
}

