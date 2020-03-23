//! Data struture serialization related stuffs for RPC

use ra_tt::{
    Delimiter, DelimiterKind, Ident, Leaf, Literal, Punct, Spacing, Subtree, TokenId, TokenTree,
};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::path::PathBuf;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExpansionTask {
    /// Argument of macro call.
    ///
    /// In custom derive that would be a struct or enum; in attribute-like macro - underlying
    /// item; in function-like macro - the macro body.
    #[serde(with = "SubtreeDef")]
    pub macro_body: Subtree,

    /// Names of macros to expand.
    ///
    /// In custom derive those are names of derived traits (`Serialize`, `Getters`, etc.). In
    /// attribute-like and functiona-like macros - single name of macro itself (`show_streams`).
    pub macro_name: String,

    /// Possible attributes for the attribute-like macros.
    #[serde(with = "opt_subtree_def")]
    pub attributes: Option<Subtree>,

    pub lib: PathBuf,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExpansionResult {
    #[serde(rename = "success", with = "SubtreeDef")]
    Success { expansion: Subtree },
    #[serde(rename = "error")]
    Error { reason: String },
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "DelimiterKind")]
enum DelimiterKindDef {
    Parenthesis,
    Brace,
    Bracket,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TokenId")]
struct TokenIdDef(u32);

#[derive(Serialize, Deserialize)]
#[serde(remote = "Delimiter")]
struct DelimiterDef {
    #[serde(with = "TokenIdDef")]
    pub id: TokenId,
    #[serde(with = "DelimiterKindDef")]
    pub kind: DelimiterKind,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Subtree")]
struct SubtreeDef {
    #[serde(default, with = "opt_delimiter_def")]
    pub delimiter: Option<Delimiter>,
    #[serde(with = "vec_token_tree")]
    pub token_trees: Vec<TokenTree>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TokenTree")]
enum TokenTreeDef {
    #[serde(with = "LeafDef")]
    Leaf(Leaf),
    #[serde(with = "SubtreeDef")]
    Subtree(Subtree),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Leaf")]
enum LeafDef {
    #[serde(with = "LiteralDef")]
    Literal(Literal),
    #[serde(with = "PunctDef")]
    Punct(Punct),
    #[serde(with = "IdentDef")]
    Ident(Ident),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Literal")]
struct LiteralDef {
    pub text: SmolStr,
    #[serde(with = "TokenIdDef")]
    pub id: TokenId,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Punct")]
struct PunctDef {
    pub char: char,
    #[serde(with = "SpacingDef")]
    pub spacing: Spacing,
    #[serde(with = "TokenIdDef")]
    pub id: TokenId,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Spacing")]
enum SpacingDef {
    Alone,
    Joint,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Ident")]
struct IdentDef {
    pub text: SmolStr,
    #[serde(with = "TokenIdDef")]
    pub id: TokenId,
}

mod opt_delimiter_def {
    use super::{Delimiter, DelimiterDef};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Delimiter>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "DelimiterDef")] &'a Delimiter);
        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Delimiter>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "DelimiterDef")] Delimiter);
        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

mod opt_subtree_def {
    use super::{Subtree, SubtreeDef};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Subtree>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "SubtreeDef")] &'a Subtree);
        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Subtree>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "SubtreeDef")] Subtree);
        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

mod vec_token_tree {
    use super::{TokenTree, TokenTreeDef};
    use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Vec<TokenTree>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "TokenTreeDef")] &'a TokenTree);

        let items: Vec<_> = value.iter().map(Helper).collect();
        let mut seq = serializer.serialize_seq(Some(items.len()))?;
        for element in items {
            seq.serialize_element(&element)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<TokenTree>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "TokenTreeDef")] TokenTree);

        let helper = Vec::deserialize(deserializer)?;
        Ok(helper.into_iter().map(|Helper(external)| external).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proc_macro_rpc_works() {
        let tt = ra_mbe::parse_to_token_tree("struct Foo {}").unwrap().0;
        let task = ExpansionTask {
            macro_body: tt.clone(),
            macro_name: Default::default(),
            attributes: None,
            lib: Default::default(),
        };

        let json = serde_json::to_string(&task).unwrap();
        let back: ExpansionTask = serde_json::from_str(&json).unwrap();

        assert_eq!(task.macro_body, back.macro_body);

        let result = ExpansionResult::Success { expansion: tt.clone() };
        let json = serde_json::to_string(&task).unwrap();
        let back: ExpansionResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result, back);
    }
}
