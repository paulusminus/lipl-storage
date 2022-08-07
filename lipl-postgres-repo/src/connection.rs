use derive_builder::Builder;

#[derive(Builder)]
pub struct Connection {
    #[builder(setter(into, strip_option))]
    host: Option<String>,
    #[builder(setter(into, strip_option))]
    user: Option<String>,
    #[builder(setter(into, strip_option), default)]
    password: Option<String>,
    #[builder(setter(into, strip_option))]
    dbname: Option<String>,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut args = vec![];

        if let Some(host) = &self.host {
            args.push(format!("host={host}"));
        }

        if let Some(user) = &self.user {
            args.push(format!("user={user}"));
        }

        if let Some(password) = &self.password {
            args.push(format!("password={password}"));
        }

        if let Some(dbname) = &self.dbname {
            args.push(format!("dbname={dbname}"));
        }

        write!(f, "{}", args.join(" "))
    }
}