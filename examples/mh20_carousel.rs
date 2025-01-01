use puzzletools::search::print_result;
use puzzletools::word::Text;
use puzzletools::wordlist::load_wordlist_iter;
use regex::Regex;

const HORSES: &[(&str, usize)] = &[
    ("COUNTRY HOUSE", 2019),
    ("ALWAYS DREAMING", 2017),
    ("AMERICAN PHAROAH", 2015),
    ("CALIFORNIA CHROME", 2014),
    ("ANIMAL KINGDOM", 2011),
    ("SUPER SAVER", 2010),
    ("BIG BROWN", 2008),
    ("STREET SENSE", 2007),
    ("SMARTY JONES", 2004),
    ("FUNNY CIDE", 2003),
    ("WAR EMBLEM", 2002),
    ("FUSAICHI PEGASUS", 2000),
    ("REAL QUIET", 1998),
    ("SILVER CHARM", 1997),
    ("THUNDER GULCH", 1995),
    ("SEA HERO", 1993),
    ("SUNDAY SILENCE", 1989),
    ("WINNING COLORS", 1988),
    ("SUNNYS HALO", 1983),
    ("PLEASANT COLONY", 1981),
    ("GENUINE RISK", 1980),
    ("SPECTACULAR BID", 1979),
    ("SEATTLE SLEW", 1977),
    ("BOLD FORBES", 1976),
    ("FOOLISH PLEASURE", 1975),
    ("RIVA RIDGE", 1972),
    ("DUST COMMANDER", 1970),
    ("MAJESTIC PRINCE", 1969),
    ("FORWARD PASS", 1968),
    ("PROUD CLARON", 1967),
    ("KAUAI KING", 1966),
    ("LUCKY DEBONAIR", 1965),
    ("NORTHERN DANCER", 1964),
    ("CARRY BACK", 1961),
    ("TOMMY LEE", 1959),
    ("TIM TAM", 1958),
    ("IRON LIEGE", 1957),
    ("DARK STAR", 1953),
    ("HILL GAIL", 1952),
    ("COUNT TURF", 1951),
    ("JET PILOT", 1947),
    ("HOOP JR", 1945),
    ("COUNT FLEET", 1943),
    ("SHUT OUT", 1942),
    ("WAR ADMIRAL", 1937),
    ("BOLD VENTURE", 1936),
    ("BROKERS TIP", 1933),
    ("BURGOO KING", 1932),
    ("TWENTY GRAND", 1931),
    ("GALLANT FOX", 1930),
    ("REIGH COUNT", 1928),
    ("BUBBLING OVER", 1926),
    ("FLYING EBONY", 1925),
    ("BLACK GOLD", 1924),
    ("BEHAVE YOURSELF", 1921),
    ("PAUL JONES", 1920),
    ("SIR BARTON", 1919),
    ("OMAR KHAYYAM", 1917),
    ("GEORGE SMITH", 1916),
    ("OLD ROSEBUD", 1914),
    ("STONE STREET", 1908),
    ("PINK STAR", 1907),
    ("SIR HUON", 1906),
    ("JUDGE HIMES", 1903),
    ("HIS EMINENCE", 1901),
    ("LIEUT GIBSON", 1900),
    ("BEN BRUSH", 1896),
    ("BEN ALI", 1895),
    ("JOE COTTON", 1885),
    ("LORD MURPHY", 1879),
    ("DAY STAR", 1878),
];

const ANSWERS: &[(char, usize)] = &[
    ('S', 1907),
    ('T', 1908),
    ('I', 1916),
    ('R', 1932),
    ('R', 1933),
    ('U', 1951),
    ('P', 1969),
    ('D', 1970),
    ('R', 1976),
    ('A', 1997),
    ('M', 2002),
    ('A', 2015),
];

fn main() {
    let mut regs: Vec<Regex> = Vec::new();
    for (n, a) in ANSWERS.iter().enumerate() {
        let start = ANSWERS.get(n - 1).map(|t| t.1).unwrap_or(1800);
        let end = ANSWERS.get(n + 1).map(|t| t.1).unwrap_or(2100);
        for (hname, yr) in HORSES {
            if *yr <= start || *yr >= end {
                continue;
            }
            let parts: Vec<_> = hname.split_whitespace().collect();
            for m in 0..parts.len() {
                for l in 0..parts[m].len() {
                    if (parts[m].as_bytes()[l] as char) != a.0 {
                        continue;
                    }
                    let mut s = String::new();
                    s.push('^');
                    for (k, t) in parts.iter().enumerate() {
                        if k > 0 {
                            s.push(' ');
                        }
                        if k == m {
                            for (j, c) in t.chars().enumerate() {
                                if j == l {
                                    s.push(c);
                                } else {
                                    s.push_str("[^");
                                    s.push(c);
                                    s.push_str(" '-]");
                                }
                            }
                        } else {
                            s.push_str(t);
                        }
                    }
                    s.push('$');
                    regs.push(Regex::new(&s).unwrap());
                }
            }
        }
    }
    load_wordlist_iter("combined.freq.txt")
        .unwrap()
        .filter(|w| regs.iter().any(|r| r.is_match(&w.word)))
        .take(300)
        .for_each(print_result);
}
