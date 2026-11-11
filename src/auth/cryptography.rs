use zbus::interface;

use crate::local::service::LocalService;

pub struct Cryptography {
    pub service: LocalService,
}

impl Cryptography {
    pub fn new(service: LocalService) -> Self {
        Cryptography { service }
    }
}

#[interface(name = "inc.kora.auth.Cryptography")]
impl Cryptography {
    /// Returns the public key used to send encrypted secrets.
    /// The data must be returned in a PEM encoded string. The
    ///  key type must be one of: ["RSA-SHA256"].
    ///
    /// # Example
    ///
    /// busctl --user call inc.kora.EpicGames \
    ///   /inc/kora/EpicGames/LegendaryClient0 \
    ///   inc.kora.auth.Cryptography \
    ///   GetPublicKey
    async fn get_public_key(&mut self) -> (String, String) {
        let key_type: &str = "RSA-SHA256";
        (key_type.to_string(), self.service.get_public_key())
    }
}
