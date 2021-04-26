use google_sign_in_wasm::BasicProfile;

#[derive(Clone, Debug)]
pub struct GoogleIdentifiedUser {
    access_token: String,
    name: String,
    given_name: String,
    family_name: String,
    image_url: String,
    email: String,
}

impl GoogleIdentifiedUser {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn given_name(&self) -> &str {
        &self.given_name
    }
    pub fn family_name(&self) -> &str {
        &self.family_name
    }
    pub fn image_url(&self) -> &str {
        &self.image_url
    }
    pub fn email(&self) -> &str {
        &self.email
    }
}

impl GoogleIdentifiedUser {
    pub fn new(profile: BasicProfile, token: String) -> GoogleIdentifiedUser {
        GoogleIdentifiedUser {
            access_token: token,
            name: profile.getName().unwrap(),
            given_name: "".to_string(),
            family_name: "".to_string(),
            image_url: "".to_string(),
            email: profile.getEmail().unwrap(),
        }
    }
}
