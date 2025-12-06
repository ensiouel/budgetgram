use crate::proto::callback::v1::Callback;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::callback::v1::callback;
    use crate::proto::callback::v1::update_category;
    use crate::proto::callback::v1::UpdateCategory;

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
