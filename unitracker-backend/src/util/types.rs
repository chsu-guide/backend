macro_rules! generate_either {
    ($name:ident, $lhs_name:ident, $lhs:ty, $rhs_name:ident, $rhs:ty) => {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(untagged)]
        pub enum $name<$lhs, $rhs> {
            $lhs_name($lhs),
            $rhs_name($rhs),
        }
    };
}

generate_either!(IdOrName, Id, u64, Name, String);
