use crate::parser::Value;

pub enum Rule {
    RuleUnion(Vec<Rule>),
    RuleList(Vec<Rule>),
    String,
    Integer,
    Float,
    Boolean,
    Any,
    List,
    Object(Vec<(String, Rule)>),
    None,
    AllUppercase,
    AllLowercase,
    Url,
    Number,
    Length(u64),
    MinLength(u64),
    MaxLength(u64),
    ListAll(Box<Rule>),
    Min(i64),
    Max(i64),
    Default(Value)
}

impl Rule {
    pub fn reduce(&self) -> Rule {
        todo!()
    }
}
