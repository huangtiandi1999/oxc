use std::{
    borrow::Cow,
    fmt::{self, Display},
    iter::Peekable,
};

use crate::{
    ast::*,
    surrogate_pair::{combine_surrogate_pair, is_lead_surrogate, is_trail_surrogate},
};

impl Display for Pattern<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.body.fmt(f)
    }
}

impl Display for Disjunction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_join(f, "|", &self.body)
    }
}

impl Display for Alternative<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn as_character<'a>(term: &'a Term) -> Option<&'a Character> {
            if let Term::Character(ch) = term { Some(ch) } else { None }
        }

        write_join_with(f, "", &self.body, |iter| {
            let next = iter.next()?;
            let Some(next) = as_character(next) else { return Some(Cow::Owned(next.to_string())) };

            let peek = iter.peek().and_then(|it| as_character(it));
            let (result, eat) = character_to_string(next, peek);
            if eat {
                iter.next();
            }

            Some(result)
        })
    }
}

impl Display for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BoundaryAssertion(it) => it.fmt(f),
            Self::LookAroundAssertion(it) => it.fmt(f),
            Self::Quantifier(it) => it.fmt(f),
            Self::Character(it) => it.fmt(f),
            Self::Dot(it) => it.fmt(f),
            Self::CharacterClassEscape(it) => it.fmt(f),
            Self::UnicodePropertyEscape(it) => it.fmt(f),
            Self::CharacterClass(it) => it.fmt(f),
            Self::CapturingGroup(it) => it.fmt(f),
            Self::IgnoreGroup(it) => it.fmt(f),
            Self::IndexedReference(it) => it.fmt(f),
            Self::NamedReference(it) => it.fmt(f),
        }
    }
}

impl Display for BoundaryAssertion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Display for BoundaryAssertionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Start => "^",
            Self::End => "$",
            Self::Boundary => r"\b",
            Self::NegativeBoundary => r"\B",
        };
        f.write_str(s)
    }
}

impl Display for LookAroundAssertion<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.kind, self.body)
    }
}

impl Display for LookAroundAssertionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Lookahead => "?=",
            Self::NegativeLookahead => "?!",
            Self::Lookbehind => "?<=",
            Self::NegativeLookbehind => "?<!",
        };
        f.write_str(s)
    }
}

impl Display for Quantifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.body.fmt(f)?;

        match (self.min, self.max) {
            (0, None) => f.write_str("*")?,
            (1, None) => f.write_str("+")?,
            (0, Some(1)) => f.write_str("?")?,
            (min, Some(max)) if min == max => write!(f, "{{{min}}}",)?,
            (min, Some(max)) => {
                write!(f, "{{{min},{max}}}",)?;
            }
            (min, None) => {
                write!(f, "{{{min},}}",)?;
            }
        }

        if !self.greedy {
            f.write_str("?")?;
        }

        Ok(())
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (string, _) = character_to_string(self, None);
        string.fmt(f)
    }
}

impl Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(".")
    }
}

impl Display for CharacterClassEscape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Display for CharacterClassEscapeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let escape = match self {
            Self::D => r"\d",
            Self::NegativeD => r"\D",
            Self::S => r"\s",
            Self::NegativeS => r"\S",
            Self::W => r"\w",
            Self::NegativeW => r"\W",
        };
        f.write_str(escape)
    }
}

impl Display for UnicodePropertyEscape<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.negative { r"\P{" } else { r"\p{" })?;
        match (&self.name, &self.value) {
            (name, Some(value)) if name == "General_Category" => value.fmt(f),
            (name, Some(value)) => write!(f, "{name}={value}"),
            (name, _) => name.fmt(f),
        }?;
        f.write_str("}")
    }
}

impl Display for CharacterClass<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn as_character<'a>(content: &'a CharacterClassContents) -> Option<&'a Character> {
            if let CharacterClassContents::Character(ch) = content { Some(ch) } else { None }
        }

        f.write_str("[")?;
        if self.negative {
            f.write_str("^")?;
        }

        if !self.body.is_empty() {
            let sep = match self.kind {
                CharacterClassContentsKind::Union => "",
                CharacterClassContentsKind::Subtraction => "--",
                CharacterClassContentsKind::Intersection => "&&",
            };

            write_join_with(f, sep, &self.body, |iter| {
                let next = iter.next()?;
                let Some(next) = as_character(next) else {
                    return Some(Cow::Owned(next.to_string()));
                };

                let peek = iter.peek().and_then(|it| as_character(it));
                let (result, eat) = character_to_string(next, peek);
                if eat {
                    iter.next();
                }

                Some(result)
            })?;
        }

        f.write_str("]")
    }
}

impl Display for CharacterClassContents<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacterClassRange(it) => it.fmt(f),
            Self::CharacterClassEscape(it) => it.fmt(f),
            Self::UnicodePropertyEscape(it) => it.fmt(f),
            Self::Character(it) => it.fmt(f),
            Self::NestedCharacterClass(it) => it.fmt(f),
            Self::ClassStringDisjunction(it) => it.fmt(f),
        }
    }
}

impl Display for CharacterClassRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

impl Display for ClassStringDisjunction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(r"\q{")?;
        write_join(f, "|", &self.body)?;
        f.write_str("}")
    }
}

impl Display for ClassString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_join(f, "", &self.body)
    }
}

impl Display for CapturingGroup<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        if let Some(name) = &self.name {
            write!(f, "?<{name}>")?;
        }
        write!(f, "{})", &self.body)
    }
}

impl Display for IgnoreGroup<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_flags(f: &mut fmt::Formatter<'_>, flags: Modifier) -> fmt::Result {
            if flags.contains(Modifier::I) {
                f.write_str("i")?;
            }
            if flags.contains(Modifier::M) {
                f.write_str("m")?;
            }
            if flags.contains(Modifier::S) {
                f.write_str("s")?;
            }
            Ok(())
        }

        f.write_str("(?")?;

        if let Some(modifiers) = &self.modifiers {
            if !modifiers.enabling.is_empty() {
                write_flags(f, modifiers.enabling)?;
            }
            if !modifiers.disabling.is_empty() {
                f.write_str("-")?;
                write_flags(f, modifiers.disabling)?;
            }
        }

        write!(f, ":{})", self.body)
    }
}

impl Display for IndexedReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r"\{}", self.index)
    }
}

impl Display for NamedReference<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r"\k<{}>", self.name)
    }
}

// ---

fn character_to_string(
    this: &Character,
    peek: Option<&Character>,
) -> (/* result */ Cow<'static, str>, /* true of peek should be consumed */ bool) {
    let cp = this.value;

    if matches!(this.kind, CharacterKind::Symbol | CharacterKind::UnicodeEscape) {
        // Trail only
        if is_trail_surrogate(cp) {
            return (Cow::Owned(format!(r"\u{cp:X}")), false);
        }

        if is_lead_surrogate(cp) {
            if let Some(peek) = peek.filter(|peek| is_trail_surrogate(peek.value)) {
                // Lead+Trail
                let cp = combine_surrogate_pair(cp, peek.value);
                let ch = char::from_u32(cp).expect("Invalid surrogate pair `Character`!");
                return (Cow::Owned(format!("{ch}")), true);
            }

            // Lead only
            return (Cow::Owned(format!(r"\u{cp:X}")), false);
        }
    }

    let ch = char::from_u32(cp).expect("Invalid `Character`!");
    let result = match this.kind {
        // Not a surrogate, like BMP, or all units in unicode mode
        CharacterKind::Symbol => Cow::Owned(ch.to_string()),
        CharacterKind::ControlLetter => match ch {
            '\n' => Cow::Borrowed(r"\cJ"),
            '\r' => Cow::Borrowed(r"\cM"),
            '\t' => Cow::Borrowed(r"\cI"),
            '\u{0019}' => Cow::Borrowed(r"\cY"),
            _ => Cow::Owned(format!(r"\c{ch}")),
        },
        CharacterKind::Identifier => Cow::Owned(format!(r"\{ch}")),
        CharacterKind::SingleEscape => match ch {
            '\n' => Cow::Borrowed(r"\n"),
            '\r' => Cow::Borrowed(r"\r"),
            '\t' => Cow::Borrowed(r"\t"),
            '\u{b}' => Cow::Borrowed(r"\v"),
            '\u{c}' => Cow::Borrowed(r"\f"),
            '\u{8}' => Cow::Borrowed(r"\b"),
            '\u{2D}' => Cow::Borrowed(r"\-"),
            _ => Cow::Owned(format!(r"\{ch}")),
        },
        CharacterKind::Null => Cow::Borrowed(r"\0"),
        CharacterKind::UnicodeEscape => {
            let hex = &format!("{cp:04X}");
            if hex.len() <= 4 {
                Cow::Owned(format!(r"\u{hex}"))
            } else {
                Cow::Owned(format!(r"\u{{{hex}}}"))
            }
        }
        CharacterKind::HexadecimalEscape => Cow::Owned(format!(r"\x{cp:02X}")),
        CharacterKind::Octal1 => Cow::Owned(format!(r"\{cp:o}")),
        CharacterKind::Octal2 => Cow::Owned(format!(r"\{cp:02o}")),
        CharacterKind::Octal3 => Cow::Owned(format!(r"\{cp:03o}")),
    };

    (result, false)
}

// ---

fn write_join<S, I, E>(f: &mut fmt::Formatter<'_>, sep: S, items: I) -> fmt::Result
where
    S: AsRef<str>,
    E: Display,
    I: IntoIterator<Item = E>,
{
    write_join_with(f, sep, items, |iter| iter.next().map(|it| it.to_string()))
}

fn write_join_with<S, I, E, F, D>(
    f: &mut fmt::Formatter<'_>,
    sep: S,
    items: I,
    next: F,
) -> fmt::Result
where
    S: AsRef<str>,
    E: Display,
    I: IntoIterator<Item = E>,
    F: Fn(&mut Peekable<I::IntoIter>) -> Option<D>,
    D: Display,
{
    let sep = sep.as_ref();
    let iter = &mut items.into_iter().peekable();

    if let Some(first) = next(iter) {
        first.fmt(f)?;
    }

    while let Some(it) = next(iter) {
        write!(f, "{sep}{it}")?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use oxc_allocator::Allocator;

    use crate::{LiteralParser, Options};

    type Case<'a> = (
        &'a str,
        /* expected display, None means expect the same as original */ Option<&'a str>,
    );

    static CASES: &[Case] = &[
        ("/ab/", None),
        ("/ab/u", None),
        ("/abc/i", None),
        ("/abc/iu", None),
        ("/a*?/i", None),
        ("/a*?/iu", None),
        ("/emo👈🏻ji/", None),
        ("/emo👈🏻ji/u", None),
        ("/ab|c/i", None),
        ("/ab|c/iu", None),
        ("/a|b+|c/i", None),
        ("/a|b+|c/iu", None),
        ("/(?=a)|(?<=b)|(?!c)|(?<!d)/i", None),
        ("/(?=a)|(?<=b)|(?!c)|(?<!d)/iu", None),
        (r"/(cg)(?<n>cg)(?:g)/", None),
        (r"/(cg)(?<n>cg)(?:g)/u", None),
        (r"/^(?=ab)\b(?!cd)(?<=ef)\B(?<!gh)$/", None),
        (r"/^(?=ab)\b(?!cd)(?<=ef)\B(?<!gh)$/u", None),
        (r"/^(?<!ab)$/", None),
        (r"/^(?<!ab)$/u", None),
        (r"/[abc]/", None),
        (r"/[abc]/u", None),
        (r"/[a&&b]/v", None),
        (r"/[a--b]/v", None),
        (r"/[^a--b--c]/v", None),
        (r"/[a[b[c[d[e[f[g[h[i[j[k[l]]]]]]]]]]]]/v", None),
        (r"/[\q{abc|d|e|}]/v", None),
        (r"/\p{Basic_Emoji}/v", None),
        (r"/[|\]]/", None),
        (r"/[|\]]/u", None),
        (r"/c\]/", None),
        (r"/c\]/u", None),
        ("/a{0}|b{1,2}|c{3,}/i", None),
        ("/a{0}|b{1,2}|c{3,}/iu", None),
        (r"/Em🥹j/", None),
        (r"/Em🥹j/u", None),
        (r"/\n\cM\0\x41\./", None),
        (r"/\n\cM\0\x41\./u", None),
        (r"/\n\cM\0\x41\u1234\./", None),
        (r"/\n\cM\0\x41\u1234\./u", None),
        (r"/[\bb]/", None),
        (r"/[\bb]/u", None),
        (r"/\d+/g", None),
        (r"/\d+/gu", None),
        (r"/\D/g", None),
        (r"/\D/gu", None),
        (r"/\w/g", None),
        (r"/\w/gu", None),
        (r"/\w+/g", None),
        (r"/\w+/gu", None),
        (r"/\s/g", None),
        (r"/\s/gu", None),
        (r"/\s+/g", None),
        (r"/\s+/gu", None),
        (r"/\t\n\v\f\r/", None),
        (r"/\t\n\v\f\r/u", None),
        (r"/\p{L}/u", None),
        (r"/\d/g", None),
        ("/abcd/igv", Some("/abcd/igv")),
        (r"/\d/ug", Some(r"/\d/ug")),
        (r"/\cY/", None),
        // we capitalize hex unicodes.
        (r"/\n\cM\0\x41\u{1f600}\./u", Some(r"/\n\cM\0\x41\u{1F600}\./u")),
        (r"/\u02c1/u", Some(r"/\u02C1/u")),
        (r"/c]/", None),
        // Octal tests from: <https://github.com/tc39/test262/blob/d62fa93c8f9ce5e687c0bbaa5d2b59670ab2ff60/test/annexB/language/literals/regexp/legacy-octal-escape.js>
        (r"/\1/", None),
        (r"/\2/", None),
        (r"/\3/", None),
        (r"/\4/", None),
        (r"/\5/", None),
        (r"/\6/", None),
        (r"/\7/", None),
        (r"/\00/", None),
        (r"/\07/", None),
        (r"/\30/", None),
        (r"/\37/", None),
        (r"/\40/", None),
        (r"/\47/", None),
        (r"/\70/", None),
        (r"/\77/", None),
        (r"/\000/", None),
        (r"/\007/", None),
        (r"/\070/", None),
        (r"/\300/", None),
        (r"/\307/", None),
        (r"/\370/", None),
        (r"/\377/", None),
        (r"/\0111/", None),
        (r"/\0022/", None),
        (r"/\0003/", None),
        (r"/(.)\1/", None),
        // Identity escape from: <https://github.com/tc39/test262/blob/d62fa93c8f9ce5e687c0bbaa5d2b59670ab2ff60/test/annexB/language/literals/regexp/identity-escape.js>
        (r"/\C/", None),
        (r"/O\PQ/", None),
        (r"/\8/", None),
        (r"/7\89/", None),
        (r"/\9/", None),
        (r"/8\90/", None),
        (r"/(.)(.)(.)(.)(.)(.)(.)(.)\8\8/", None),
        // Class escape from: <https://github.com/tc39/test262/blob/d62fa93c8f9ce5e687c0bbaa5d2b59670ab2ff60/test/annexB/language/literals/regexp/class-escape.js>
        (r"/\c0/", None),
        (r"/[\c0]/", None),
        (r"/\c1/", None),
        (r"/[\c10]+/", None),
        (r"/\c8/", None),
        (r"/[\c8]/", None),
        (r"/[\c80]+/", None),
        (r"/\c_/", None),
        // Capitalize hex unicodes --
        (r"/^|\udf06/gu", Some(r"/^|\uDF06/gu")),
        (r"/\udf06/", Some(r"/\uDF06/")),
        (r"/\udf06/u", Some(r"/\uDF06/u")),
        (r"/^|\udf06/g", Some(r"/^|\uDF06/g")),
        // --
        (r"/[\-]/", None),
        (r"/[\-]/u", None),
        (r"/[\-]/v", None),
        (r"/([\-a-z]{0,31})/iu", None),
        // ES2025 ---
        (r"/(?i:.)/", None),
        (r"/(?-s:.)/", None),
        (r"/(?im-s:.)/u", None),
        (r"/(?m-is:.)/v", None),
        (r"/(?smi:.)/v", Some(r"/(?ims:.)/v")),
    ];

    #[test]
    fn test_display() {
        let allocator = &Allocator::default();

        for (input, output) in CASES {
            let (left_slash, right_slash) = (input.find('/').unwrap(), input.rfind('/').unwrap());

            let pattern = &input[left_slash + 1..right_slash];
            let flags = &input[right_slash + 1..];

            let actual = LiteralParser::new(allocator, pattern, Some(flags), Options::default())
                .parse()
                .unwrap();

            let expect = output.unwrap_or(input);
            assert_eq!(expect, format!("/{actual}/{flags}")); // This uses `Display` impls
        }
    }
}
