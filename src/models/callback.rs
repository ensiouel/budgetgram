use crate::proto::callback::v1::{Callback, CategoryDirection};
use prost::Message;

impl TryFrom<Callback> for String {
    type Error = ();

    fn try_from(value: Callback) -> Result<Self, Self::Error> {
        Ok(base122_rs::encode(&value.encode_to_vec()))
    }
}

impl TryFrom<String> for Callback {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match base122_rs::decode(value.as_str()) {
            Ok(bytes) => match Callback::decode(bytes.as_slice()) {
                Ok(callback) => Ok(callback),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }
}

pub trait Declinable {
    fn decline(&self, case: NameCase, number: GrammaticalNumber) -> &'static str;
}

pub trait Labeled {
    fn label(&self) -> &'static str;
}

pub enum GrammaticalNumber {
    Singular,
    Plural,
}

pub enum NameCase {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Creative,
    Prepositional,
}

const EXPENSE_FORMS: [[&str; 6]; 2] = [
    [
        "расход",
        "расхода",
        "расходу",
        "расход",
        "расходом",
        "расходе",
    ],
    [
        "расходы",
        "расходов",
        "расходам",
        "расходы",
        "расходами",
        "расходах",
    ],
];

const INCOME_FORMS: [[&str; 6]; 2] = [
    ["доход", "дохода", "доходу", "доход", "доходом", "доходе"],
    [
        "доходы",
        "доходов",
        "доходам",
        "доходы",
        "доходами",
        "доходах",
    ],
];

impl NameCase {
    fn as_index(&self) -> usize {
        match self {
            NameCase::Nominative => 0,
            NameCase::Genitive => 1,
            NameCase::Dative => 2,
            NameCase::Accusative => 3,
            NameCase::Creative => 4,
            NameCase::Prepositional => 5,
        }
    }
}

impl GrammaticalNumber {
    fn as_index(&self) -> usize {
        match self {
            GrammaticalNumber::Singular => 0,
            GrammaticalNumber::Plural => 1,
        }
    }
}

impl Declinable for CategoryDirection {
    fn decline(&self, case: NameCase, number: GrammaticalNumber) -> &'static str {
        match self {
            CategoryDirection::Expense => EXPENSE_FORMS[number.as_index()][case.as_index()],
            CategoryDirection::Income => INCOME_FORMS[number.as_index()][case.as_index()],
            CategoryDirection::Unspecified => unreachable!(),
        }
    }
}

impl Labeled for CategoryDirection {
    fn label(&self) -> &'static str {
        match self {
            CategoryDirection::Expense => "📉",
            CategoryDirection::Income => "📈",
            CategoryDirection::Unspecified => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::callback::v1::UpdateCategory;
    use crate::proto::callback::v1::callback;
    use crate::proto::callback::v1::update_category;

    #[test]
    fn test() {
        let callback = Callback {
            query: Option::from(callback::Query::UpdateCategory(UpdateCategory {
                category_id: 33,
                field: i32::from(update_category::Field::Label),
            })),
        };
        let str = String::try_from(callback).unwrap();
        assert_eq!(str.as_str(), "\u{15}\u{1}\u{1}\u{2}\u{8}@\u{4}");
        assert_eq!(Callback::try_from(str).unwrap(), callback);
    }
}
