

pub enum Errors {
    BadMeleniumFalconFile,
    BadEmpireFile,
}

impl Errors {
    /*
     * Get the error message
     * @return String
     */
    pub fn get_error(&self) -> String {
        let message_bad_melenium_falcon_file = "Unreacheable file or wrong file structure:\n\
        {\n\
        \"autonomy\": 6,\n\
        \"departure\": \"Tatooine\",\n\
        \"arrival\": \"Endor\",\n\
        \"routes_db\": \"universe.db\"\n\
        }";

        let message_bad_empire_file = "Unreacheable file or wrong file structure:\n\
        {\n\
        \"countdown\": 10,\n\
        \"bounty_hunters\": [\n\
        {\n\
        \"planet\": \"Tatooine\",\n\
        \"day\": 1\n\
        },\n\
        {\n\
        \"planet\": \"Endor\",\n\
        \"day\": 3\n\
        }\n\
        ]\n\
        }";
        match self {
            Errors::BadMeleniumFalconFile => message_bad_melenium_falcon_file.to_string(),
            Errors::BadEmpireFile => message_bad_empire_file.to_string(),
        }
    }
}